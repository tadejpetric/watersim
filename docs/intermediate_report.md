# Real time simulation of water surface
The goal of this project is to create a real time simulation of water surface. To achieve this I am writing a program in Rust with the Glow library.

## Infrastructure
Rust is a modern low level programming language. As I've barely done anything with it, the first goal of the project was to learn more about the features of the language. Glow is a Rust API to the OpenGL graphics API. This allows us to run code on the Graphics Processing Unit (henceforth GPU), which makes the goals of the project realistic. The library itself is a very minimal wrapper over OpenGL, it just renames C FFI functions to be in-line with the Rust naming conventions and declares the correct types for the function. The process of writing graphics code (called shaders) is still done in SPIR-V or GLSL. Since SPIR-V is usually produced by compiling GLSL, I chose the more straightforward method and used GLSL for writing the code that executes on the GPU. I'm using the SDL2 library to interface with the OS - it provides the functionality to capture keypresses and display windows with the OpenGL context.

The code written in Rust is processed on the CPU, the code written in GLSL is processed on the GPU. The CPU code prepares the data that GPU processes. The processed data is then directly displayed (and we never send data back to the CPU, as that is usually too slow for real-time rendering). There are two types of data used in the project: uniform variables and vertex buffers. Vertex buffer stores vertices of triangles that make up our water surface. When they are sent to the GPU, each vertex is processed independently and does not see its neighboring vertices. All that the shader code gets is the coordinates of its own vertex. Then the GPU executes code for every vertex in parallel extremely efficiently. The uniform variables are the opposite of this; they are shared values for every execution. A good example of this is the "time" variable. We want to process the shape of the water at a certain point in time, so the program gets "point in space" (from vertex buffer) and "time" as a uniform variable. With that fixed time, we compute the shape for every vertex. Then we increment the time, and run the programs again for every vertex.

The program proceeds with the following sequence of steps:
- Reading the configuration file (which sets the number of vertices, speed of simulation, ...)
- Creating an OpenGL context.
- Attaching the shader programs.
- Creating uniform data that will be sent to the program
    - Current time in the simulation
    - The camera (4x4 projective matrix that allows us to rotate and move around)
    - Perspective matrix (4x4 projective matrix that encodes perspective transformation, so objects further away appear smaller)
    - The parameters of the waves (amplitude, frequency and speed for each wave of the water)
- Creating the vertex buffer data (a flat sheet of triangles)
- Starting the Event loop
    - Process keyboard events (moving and rotating in space)
    - Updating the camera and time uniforms
    - Drawing the scene

## Algorithm

The algorithm for the task has to be fairly sensitive to allow for real time rendering. The water is made up of millions of triangles that have to be drawn in only ~10ms. The simplest, but very successful, approach is the "sum of sines" approximation, as seen in [GPU Gems](https://developer.nvidia.com/gpugems/gpugems/part-i-natural-effects/chapter-1-effective-water-simulation-physical-models). The technique was the state of the art for real time water simulation 20 years ago and, due to the low performance of the machines, was often used with very few waves and triangles. Prior to that, they usually did not bother simulating waves at all and only presented them as a flat sheet (perhaps moving and tilting) with a moving texture.

### Vertex shader
The Sum of Sines algorithm is quite simple: water is presented as the graph of a 2D function of the form
$$
    f(x,y) = \sum_{i=0}^N a_i\sin(b_i\cdot x + c_i\cdot y + d_i\cdot t)
$$

We subdivide the surface into triangles and send it off to the shader along with the parameters a, b, c and d. Currently, N, the number of waves, is fixed and hardcoded as GLSL does not *really* have loops - if one wants to index the arrays in a for loop  
for (int i = i; i < N; ++i) a[i]  
Then the value N must be a compile time constant. The shader compilation process then unrolls the loop into an explicit sequence of steps.

Then we use the vertex shader to apply the transformation to the vertex. We start with z=0, loop over the every wave and add the value of the wave at this point in space. Then we use the `gl_Position` variable to store the final position of the vertex. This is also the point where we multiply with the camera and perspective matrices - for OpenGL the camera is always at the center, so the camera matrix moves every vertex (analogous to moving the camera). The perspective matrix then moves the vertices furthest away to be closest together. These features are very natural to OpenGL, which can be seen by `gl_Position` being a 4 element vector (rather than 3); it's a vector in projective space, where perspective and camera transformations can be done via simple matrix multiplication.  
Regardless, the vertex shader outputs the water surface (3D point in space) into the next shader in the graphics pipeline and stores the value of the vertex position as an OpenGL primitive.

### Fragment shader
The next step is the fragment shader. This is responsible for the colouring, which is a bit more involved at the moment.

We first set various parameters of the shader. One such parameter is the position of the sun. Without simulating shadows, we could not see anything (the water surface would seem like a flat sheet - the shadows give us the features and impression of depth). To compute the shadows at a certain point in space, we first need to compute the normal to the surface. That's because if the normal points directly towards the sun, the surface will be maximally bright. If it points away, it's darker (like how the day is brightest at noon and gets darker as sun starts pointing at a slanted angle). Again we loop through every wave at a point in space, compute the partial derivative of a wave and add them up. We compute the partial derivatives by x and y, which are then used to get the normal to the surface
$$
\left(-\frac{\partial z}{\partial x}, -\frac{\partial z}{\partial y}, 1\right)
$$
Then we compute the dot product with the direction of the sun to determine how much to shade.

A similar process is used for sun reflections.  
Here we store the position of the camera, the position of the sun, the size of the sun and a "critical angle", beyond which light has to be reflected. If the angle from the camera to the water is larger than the critical angle and the reflection points towards the sun, we colour the point whiter.  
For a better quality, we use two sun sizes - a larger one, and a smaller one. If the reflection points towards the smaller size, the influence of the sun is maximal. Outside of the larger one, we do not draw the reflection of the sun at all. For values inbetween, we interpolate the value of the influence. This allows the edges of the reflection to not be as sharp.

## Pictures

The initial working state of the program. I represented water surface as a sum of two perpendicular waves. The colours were black and white (white pointing towards the sun and black away). The perspective was not working yet. It looks very weird, like a rotating square prism.  
![Initial with black and white shading for clarity](images/image.png)

The same image with perspective working and minor modifications.  
![Perspective working](images/image-1.png)

Improved the colour model of the program. I also improved the camera movement, but that cannot be seen in the picture (the movement is now relative, as in programs like Blender or Autodesk Maya. Before it was determined by "current position" and "look at position". You moved the coordinates of each with the keyboard. Now it is "current position" and "look at direction" - pressing "W" will move the camera forward in the direction camera is facing in, rather than moving the "look at" coordinates, which is much more intuitive to use).  
![Updated colours](images/image-2.png)

Large scale expansion of the water surface. It still works <16ms per frame (which is 60FPS, the limit of my windowing system as you have to jump through a lot of hoops to disable Vsync on it).  
![large scale](images/image-3.png)

Reworked lighting system again. Waves are now a linear combination of 32 randomly chosen waves (with frequency inversely proportional to amplitude). The sun reflection has also been added. This is the current state of the program.  
![Current state](images/image-4.png)

## Current state
All described above has been accomplished so far and I've put a fair bit of work into it, since I've had no real experience with graphics programming before, nor with Rust.

Next steps include changing the parameters of the simulation (which is harder than it seems, since you have to get them just right to be realistic). The emphasis will be on the relationship between frequency, amplitude and speed of the waves. I want to find a good way to determine them algorithmically, most existing approaches let the artists hardcode the parameters. I also have to improve the shading of the waves again.

Then the next step is experimenting with other wave types. For instance the GPU Gems article outlines Gerstner waves as an alternative, which have sharper peaks than sine waves. By my evaluation, they would be better for representing rough water, but sine waves are just fine for calm water and water near the coast (at least 10m away but not over 30m).

If there is any time left I will also be experimenting with noise textures to help against tiling, bump maps for better texture and possibly tesselation for improved fidelity. But I am skeptical I will accomplish all of these and I am happy with the progress as it stands.

## Post Scriptum
This project made me more aware of how water looks like in reality. Whenever I go visit the sea or a pond, I observe and record how the water looks like. I usually come to the conclusion that it looks unrealistic.