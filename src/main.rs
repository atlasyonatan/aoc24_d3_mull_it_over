use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let reg = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    let muls = reg.captures_iter(&input).map(|captures| {
        let mut iter = captures
            .iter()
            .skip(1)
            .map(Option::unwrap)
            .map(|s| s.as_str().parse::<u16>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    });

    let result: u32 = muls.map(|(n1, n2)| n1 as u32 * n2 as u32).sum();

    println!("Part 1: {}", result);
}
