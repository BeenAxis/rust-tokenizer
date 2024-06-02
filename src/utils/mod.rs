pub mod linkedlist;
pub mod voids;

use std::io::{BufRead, BufReader};

use crate::File;


pub fn read_lines(path: impl Into<String>) -> Vec<String> {
    let file = path.into();
    let file = File::open(file).unwrap();
    let buf = BufReader::new(file);    
    let lines:Vec<String> = buf.lines().map(|x| x.unwrap()).collect(); 
    lines
}
