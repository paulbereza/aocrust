use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;

fn toggle(board : &mut [[bool; 1000]; 1000], x0: usize, x1: usize, y0: usize, y1:usize) {
    for x in x0..x1 {
        for y in y0..y1 {
            board[x][y] = !board[x][y];
        }
    }
}

fn turn_on(board : &mut [[bool; 1000]; 1000], x0: usize, x1: usize, y0: usize, y1:usize) {
    for x in x0..x1 {
        for y in y0..y1 {
            board[x][y] = true;
        }
    }
}

fn turn_off(board : &mut [[bool; 1000]; 1000], x0: usize, x1: usize, y0: usize, y1:usize) {
    for x in x0..x1 {
        for y in y0..y1 {
            board[x][y] = false;
        }
    }
}


fn handle_line(board : &mut [[bool; 1000]; 1000], line : String){
}

fn count_board(board : [[bool; 1000]; 1000]) -> i32 {
    let mut c = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if board[x][y] {
                c+=1;
            }
        }
    }
    return c;
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

    for line in reader.lines() {
        handle_line(&mut board, line.unwrap());
    }


    println!("lights on = {}", count_board(board));

}

