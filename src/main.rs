use std::{cell::RefCell, rc::Rc};

use search::Node;

mod map_gen;
pub mod search;


fn main() {
    //acquire map
    let col=16;
    let row=16;
    let map=map_gen::map_rand_gen(col, row);
    //print_map(&map)
    print_map(&map);
    let end:Rc<RefCell<Node>>=Node::new(0, 0, 0, (15,15), None);
    let begin:Rc<RefCell<Node>>=Node::new(0, 0, 0, (0,0), None);
    let mut path:Vec<(i32,i32)>=Vec::new();
    search::search_func(&map, begin, end, &mut path);
    print_map_path(&path, &map);
    
}

fn print_map(map: &[Vec<i8>]) {
    for row in map.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}
fn print_map_path(path: &Vec<(i32,i32)>,map: &[Vec<i8>]) {
    // Create a mutable copy of the map
    let mut map_with_path: Vec<Vec<char>> = map.iter()
        .map(|row| row.iter().map(|&cell| if cell == 0 { '.' } else { '#' }).collect())
        .collect();
    
    // Mark the path on the map
    for &(x, y) in path {
        if x >= 0 && y >= 0 && (x as usize) < map_with_path.len() && (y as usize) < map_with_path[0].len() {
            map_with_path[x as usize][y as usize] = '*';
        }
    }
    
    // Print the map with the path
    for row in map_with_path {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}


//8888888888check sort of vec
// let mut open = vec![Node::new(1, 1, 0, (2,3), None),
// Node::new(15, 2, 13, (2,3), None),
// Node::new(12, 1, 11, (3,2), None)];
// open.sort_by(|a,b| b.borrow().f.cmp(&a.borrow().f));
// for item in open{
// println!("{:?}",item.borrow().f) 
// }