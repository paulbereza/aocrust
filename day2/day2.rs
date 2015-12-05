use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::max;

fn required_for_box(w:i64, l: i64, h: i64) -> (i64, i64) {

    let maxd = max(max(l, w), h);

    let paper = 2 * (l*w + w*h + h*l) + w*l*h/maxd;
    let ribbon = w*l*h + 2*(l+w+h-maxd);

    (paper, ribbon)

}

fn required_for_line(line: String) -> (i64, i64) {
    let bits: Vec<&str> = line.trim().split('x').collect();

    assert!(bits.len() == 3, "malformed line \"{}\"");

    required_for_box(
        bits[0].parse().unwrap(),
        bits[1].parse().unwrap(),
        bits[2].parse().unwrap()
        )
}

fn main() {

    let file = File::open("day2.input").unwrap();
    let reader = BufReader::new(&file);


    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let (paper, ribbon) = required_for_line(l);
        total_paper += paper;
        total_ribbon += ribbon;

    }
    println!("paper={}", total_paper);
    println!("ribbon={}", total_ribbon);
}

