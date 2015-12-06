use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::env;
use std::collections::HashSet;

fn is_nice(line : &String) -> bool {
    
    let mut prev = '#';

    let mut saw_double = false;
    let mut vowels = 0;


    for c in line.chars() {

        saw_double |= c ==prev;

        if "aeiou".contains(c) {
            vowels+=1;
        }

        prev = c;
    }

    vowels >= 3 && 
    saw_double && 
    !line.contains("ab") &&
    !line.contains("cd") &&
    !line.contains("pq") &&
    !line.contains("xy")

}

fn is_nice2(line : &String) -> bool {
    
    let mut prev = '#';
    let mut prev2 = '#';

    let mut saw_double = false;
    let mut saw_pair = false;

    let mut seen = HashSet::new();

    for c in line.chars() {
        let pair = format!("{}{}", prev, c);

        saw_pair |= seen.contains(&pair);

        if prev2!='#' {
            seen.insert(format!("{}{}", prev2, prev));
        }

        saw_double |= c == prev2;

        prev2 = prev;
        prev = c;
    }


    return saw_double && saw_pair;


}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len()!=2 {
        panic!("Usage: {} [INPUTFILE]", args[0]);
    }
    let filename = &args[1];
    
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let mut nice_count = 0;
    let mut nice2_count = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        if is_nice(&l) {
            nice_count+=1;
        }
        if is_nice2(&l) {
            nice2_count+=1;
        }
    }

    println!("nice_count = {}", nice_count);
    println!("nice2_count = {}", nice2_count);
}

