
extern crate crypto;

use std::env;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn find_for_target(stem : &str, target :  &str) {
    let mut x = 0;
    let mut digest = Md5::new();

    loop {
        let mut s = String::with_capacity(16);

        s.clear();
        s.push_str(&stem);
        s.push_str(&x.to_string());

        digest.reset();
        digest.input_str(&s);
        
        if digest.result_str().starts_with(target) {
           
            println!("target={:?} key = {}", target, x);

            return;
        }

        x+=1;

    }


}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: day4 [input]");
    }

    find_for_target(&args[1], "00000");
    find_for_target(&args[1], "000000");




}



