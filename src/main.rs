use itertools::Itertools;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let reg = Regex::new(r"(?<mul>mul\((\d{1,3})\,(\d{1,3})\))|(?<do>do\(\))|(?<dont>don\'t\(\))")
        .unwrap();
    let names: Vec<_> = reg.capture_names().skip(1).filter_map(|c| c).collect();
    let commands: Vec<_> = reg
        .captures_iter(&input)
        .map(|captures| {
            let name = names
                .iter()
                .find(|name| captures.name(name).is_some())
                .unwrap();

            return match *name {
                "mul" => {
                    let (a, b) = captures
                        .iter()
                        .skip(2) //skip full group and named group
                        .map(Option::unwrap)
                        .map(|s| s.as_str())
                        .map(|s| s.parse().unwrap())
                        .next_tuple()
                        .unwrap();

                    Command::Mul(a, b)
                }
                "do" => Command::Do,
                "dont" => Command::Dont,
                _ => unreachable!(),
            };
        })
        .collect();

    let result: u32 = commands
        .iter()
        .filter_map(|command| match command {
            Command::Mul(a, b) => Some((*a as u32) * (*b as u32)),
            _ => None,
        })
        .sum();

    println!("Part 1: {}", result);

    let result: u32 = commands
        .into_iter()
        .fold((true, 0), |(mut enabled, mut sum), command| {
            match command {
                Command::Mul(a, b) if enabled => {
                    sum += (a as u32) * (b as u32);
                }
                Command::Do => enabled = true,
                Command::Dont => enabled = false,
                _ => {}
            }

            (enabled, sum)
        })
        .1;

    println!("Part 2: {}", result);
}

#[derive(Debug)]
enum Command {
    Mul(u16, u16),
    Do,
    Dont,
}
