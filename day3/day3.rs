use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {

    let mut f = File::open("day3.input").unwrap();

    let mut pos = ((0, 0), (0, 0));

    let mut buffer: [u8; 1] = [0];

    let mut visited = HashSet::new();

    visited.insert((0, 0));

    while f.read(&mut buffer).unwrap() > 0 {
        let (pos_before, pos_other) = pos;
        let (x, y) = pos_before;

        let c = buffer[0] as char;
        
        let pos_after = match c {
            '<' => (x-1, y),
            '>' => (x+1, y),
            '^' => (x, y-1),
            'v' => (x, y+1),
            _ => panic!("unexpected char {} in input", c),
        };

        pos = (pos_other, pos_after);

        visited.insert((x, y));
    }

    println!("visited: {}", visited.len());
    

}

