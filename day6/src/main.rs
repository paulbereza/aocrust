use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;


fn handle_line(board : [[bool; 1000]; 1000], line : String) -> i32 {
    0
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len()!=2 {
        panic!("Usage: {} [INPUTFILE]", args[0]);
    }
    let filename = &args[1];
    
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let mut board = [[false; 1000]; 1000];

    let mut on = 0;

    for line in reader.lines() {
        on += handle_line(board, line.unwrap());
    }

    println!("lights on = {}", on);

}

