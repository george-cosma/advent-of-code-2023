use std::{
    env,
    io::{BufRead, BufReader},
    process::exit, str::LinesAny,
};

use stable_eyre::eyre::{bail, eyre, Context, Report};

#[derive(Debug, thiserror::Error)]
enum Day3Error {
    #[error("You must pass the input file as a parameter! Example:\nday3 ./input.txt")]
    NoInputFile,
    #[error("The supplied file must have at least 2 lines.")]
    NotEnoughLines,
}

const BLANK: char = '.';
const BLANK_STR: &str = ".";

fn is_special_char(c: char) -> bool {
    match c {
        BLANK | '0'..='9' | '\n' | '\r' => false,
        _ => true,
    }
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
        line.replace_range(start..end, BLANK_STR.repeat(end -start).as_str());
        Ok(Some(num))
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
    
    let mut sum = 0;

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

            if let Some(num) = parse_number(&mut prev_line, pos + 1)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut prev_line, pos - 1)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut line, pos)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut line, pos + 1)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut line, pos - 1)? {
                dbg!(num);
                sum += num;
            }
        }


        let line_copy = line.clone();
        for (pos, c) in line_copy.chars().enumerate() {
            if !is_special_char(c) {
                continue;
            }

            if let Some(num) = parse_number(&mut prev_line, pos + 1)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut prev_line, pos - 1)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut prev_line, pos)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut line, pos + 1)? {
                dbg!(num);
                sum += num;
            }
            if let Some(num) = parse_number(&mut line, pos - 1)? {
                dbg!(num);
                sum += num;
            }
        }



        prev_line = line.clone();
        line.clear();
    }

    println!("Sum = {sum}");

    Ok(())
}
