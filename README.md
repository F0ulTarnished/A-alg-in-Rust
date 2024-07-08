# A*-alg-in-Rust
Beginner work, could run through, efficiency unknown

# Env
rustc 1.76.0
crate rand 0.8.5

# How to use
This repository contains the source code of my A* Alg. You may complie debug or release version as your wish.
The usage is rather simple, just simply run it.

# About files
## main.rs
fn main() executes in order.  
Fisrt generates a map::vec&lt;vec&lt;i8&gt;&gt;;
Second prints the map; 
Third searchs path;
Fourth prints the map with the path.
## main.rs
fn map_rand_gen() lies here. You may load your map or change the type here.
## search.rs
fn search_fun() and struct Node lives here.
See notes within to find more.
# Reference
https://www.gamedev.net/reference/articles/article2003.asp
# Note
crate rand can't be set as 0.8,it fails.
no detection of whether the begin or end point is accessible or not
