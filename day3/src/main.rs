use std::{
    env,
    io::{BufRead, BufReader},
    process::exit,
    str::LinesAny,
};

use stable_eyre::eyre::{bail, eyre, Context, Report};

#[derive(Debug, thiserror::Error)]
enum Day3Error {
    #[error("You must pass the input file as a parameter! Example:\nday3 ./input.txt")]
    NoInputFile,
    #[error("The supplied file must have at least 2 lines.")]
    NotEnoughLines,
    #[error("Found a gear with more than 2 numbers.")]
    OverflowingGear,
}

const BLANK: char = '.';
const BLANK_STR: &str = ".";

fn is_special_char(c: char) -> bool {
    match c {
        BLANK | '0'..='9' | '\n' | '\r' => false,
        _ => true,
    }
}

fn is_gear(c: char) -> bool {
    c == '*'
}

enum Position {
    NotADigit,
    Found(usize),
    AtTheEnd,
}

fn find_first_non_digit_index(mut iter: impl Iterator<Item = (usize, char)> + Clone) -> Position {
    match iter.clone().peekable().peek() {
        Some((_, c)) if c.is_ascii_digit() => {}
        _ => return Position::NotADigit,
    }

    match iter.find(|(_, c)| !c.is_ascii_digit()) {
        Some((i, _)) => Position::Found(i),
        None => Position::AtTheEnd,
    }
}

fn parse_number(line: &mut String, pivot_index: usize) -> Result<Option<u32>, Report> {
    let start = match find_first_non_digit_index(line[..=pivot_index].char_indices().rev()) {
        Position::NotADigit => return Ok(None),
        Position::Found(index) => index + 1,
        Position::AtTheEnd => 0,
    };
    let end = match find_first_non_digit_index(line[pivot_index..].char_indices()) {
        Position::NotADigit => return Ok(None),
        Position::Found(index) => pivot_index + index,
        Position::AtTheEnd => line.len(),
    };

    if start == end {
        Ok(None)
    } else {
        let num = line[start..end].parse()?;
        line.replace_range(start..end, BLANK_STR.repeat(end - start).as_str());
        Ok(Some(num))
    }
}

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
    first_num: Option<u32>,
    second_num: Option<u32>,
}

impl Gear {
    fn get_ratio(&self) -> Option<u32> {
        match (self.first_num, self.second_num) {
            (Some(first), Some(second)) => Some(first * second),
            _ => None,
        }
    }

    fn at(&self, x: usize, y: usize) -> bool {
        self.x == x && self.y == y
    }

    fn add_num(&mut self, num: u32) -> Result<(), Report> {
        if self.first_num.is_none() {
            self.first_num = Some(num);
            Ok(())
        } else if self.second_num.is_none() {
            self.second_num = Some(num);
            Ok(())
        } else {
            Err(eyre!(Day3Error::OverflowingGear))
        }
    }
}

fn main() -> Result<(), Report> {
    stable_eyre::install()?;

    if env::args().len() != 2 {
        bail!(Day3Error::NoInputFile)
    }

    let filename = env::args().nth(1).unwrap();

    let file = std::fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut prev_line = String::new();
    let mut line = String::new();

    let mut gears: Vec<Gear> = Vec::new();

    let mut sum = 0;
    let mut line_num = 0;

    while reader.read_line(&mut line)? != 0 {
        line = line.trim().to_string();
        if prev_line.is_empty() {
            prev_line = BLANK_STR.repeat(line.len());
        }
        println!("---------------------");
        println!("prev_line: {prev_line}");
        println!("     line: {line}");

        let prev_copy = prev_line.clone();
        for (pos, c) in prev_copy.chars().enumerate() {
            if !is_special_char(c) {
                continue;
            }

            let mut gear = if is_gear(c) {
                if let Some(g) = gears.iter_mut().find(|g| g.at(pos, line_num - 1)) {
                    Some(g)
                } else {
                    gears.push(Gear {
                        x: pos,
                        y: line_num - 1,
                        first_num: None,
                        second_num: None,
                    });
                    Some(gears.last_mut().unwrap())
                }
            } else {
                None
            };

            if let Some(num) = parse_number(&mut prev_line, pos + 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut prev_line, pos - 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut line, pos)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut line, pos + 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut line, pos - 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
        }

        let line_copy = line.clone();
        for (pos, c) in line_copy.chars().enumerate() {
            if !is_special_char(c) {
                continue;
            }

            let mut gear = if is_gear(c) {
                if let Some(g) = gears.iter_mut().find(|g| g.at(pos, line_num)) {
                    Some(g)
                } else {
                    gears.push(Gear {
                        x: pos,
                        y: line_num,
                        first_num: None,
                        second_num: None,
                    });
                    Some(gears.last_mut().unwrap())
                }    
            } else {
                None
            };

            if let Some(num) = parse_number(&mut prev_line, pos + 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut prev_line, pos - 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut prev_line, pos)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut line, pos + 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
            if let Some(num) = parse_number(&mut line, pos - 1)? {
                sum += num;
                if let Some(g) = gear.as_mut() {
                    g.add_num(num)?;
                }
            }
        }

        prev_line = line.clone();
        line.clear();
        line_num += 1;
    }

    println!("Sum = {sum}");
    println!("Gears Ratio Sum = {}", gears.iter().map(|g| g.get_ratio().unwrap_or(0)).sum::<u32>());

    Ok(())
}
