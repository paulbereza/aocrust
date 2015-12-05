use std::fs::File;
use std::io::Read;

fn main() {

    let mut f = File::open("day1.input").unwrap();

    let mut floor = 0;
    let mut pos = 0;
    let mut basement = -1;

    let mut buffer: [u8; 1] = [0];

    while f.read(&mut buffer).unwrap() > 0 {
        pos+=1;
        let c = buffer[0] as char;
        
        match c  {
            '(' => floor+=1,
            ')' => floor-=1,
            _ => panic!("unexpected char {} in input", buffer[0]),
        }
        if floor == -1 && basement == -1 {
            basement = pos;
        }
    }

    println!("floor reached: {}", floor);
    
    match basement {
        -1 => println!("No basement"),
        _ =>  println!("Basement at {}", basement),
    }

}

