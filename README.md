# watersim
Water sim for the Mathematics with Computers class at FMF

# Installation (using Arch Linux)
## Install Rust
sudo pacman -S rustup
rustup default stable
## Setup repository
git clone git@github.com:tadejpetric/watersim.git
cargo init --bin

# Running the program
cargo run -- /home/tadej/programming/watersim/config.txt