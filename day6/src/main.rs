
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::cmp::max;
use regex::Regex;

fn toggle(val : i32) -> i32 {
    val + 2
}

fn turn_on(val : i32) -> i32 {
    val + 1
}

fn turn_off(val : i32) -> i32 {
    max(val - 1, 0)
}


fn handle_line(board : &mut [[i32; 1000]; 1000], line : String){
    let re = Regex::new("(turn on|toggle|turn off) (\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();

    let captures = re.captures(&line).unwrap();

    let f = match(captures.at(1)) {
        Some("turn on") => turn_on as fn(i32) -> i32,
        Some("turn off") => turn_off as fn(i32) -> i32,
        Some("toggle") => toggle as fn(i32) -> i32,
        _ => panic!("unmatched line {}", line),
    };

    let x0 = captures.at(2).unwrap().parse::<usize>().unwrap();
    let y0 = captures.at(3).unwrap().parse::<usize>().unwrap();
    let x1 = captures.at(4).unwrap().parse::<usize>().unwrap() + 1;
    let y1 = captures.at(5).unwrap().parse::<usize>().unwrap() + 1;

    for x in x0..x1 {
        for y in y0..y1 {
            board[x][y] = f(board[x][y]);
        }
    }
}

fn count_board(board : &[[i32; 1000]; 1000]) -> i32 {
    let mut c = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            c+=board[x][y];
        }
    }
    c
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len()!=2 {
        panic!("Usage: {} [INPUTFILE]", args[0]);
    }
    let filename = &args[1];
    
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let mut board = [[0; 1000]; 1000];

    for line in reader.lines() {
        handle_line(&mut board, line.unwrap());
    }


    println!("lights on = {}", count_board(&board));

}

