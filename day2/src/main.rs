use std::{
    env,
    io::{BufRead, BufReader},
    str::Chars, collections::HashMap,
};

use anyhow::bail;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl CubeColor {
    fn as_str(&self) -> &'static str {
        match self {
            CubeColor::Red => "red",
            CubeColor::Green => "green",
            CubeColor::Blue => "blue",
        }
    }
}

#[derive(Debug)]
enum Separator {
    Showings,
    Cubes,
}

impl Separator {
    fn as_str(&self) -> &'static str {
        match self {
            Separator::Showings => ";",
            Separator::Cubes => ",",
        }
    }
}

#[derive(Debug)]
enum Token {
    Game(u32),
    Number(u32),
    Color(CubeColor),
    Separator(Separator),
}

fn next_number(input: &mut Chars<'_>) -> Result<Token, anyhow::Error> {
    // println!("Next number input: ");
    // dbg!(input.clone().collect::<String>());

    let number_str: String = input
        .peeking_take_while(|chr| chr.is_ascii_digit())
        .collect();

    Ok(Token::Number(number_str.parse()?))
}

fn iter_contains(mut haystack: impl Iterator<Item = char> + Clone, mut needle: Chars<'_>) -> bool {
    while let Some(fibre) = haystack.next() {
        if let Some(to_match) = needle.next() {
            if fibre != to_match {
                return false;
            }
        } else {
            return true;
        }
    }

    if let Some(_) = needle.next() {
        // The haystack ended before finding the needle.
        false
    } else {
        true
    }
}

fn next_color(input: &mut Chars<'_>) -> Result<Token, anyhow::Error> {
    // println!("Next color input: ");
    // dbg!(input.clone().collect::<String>());

    if iter_contains(input.clone(), CubeColor::Red.as_str().chars()) {
        input.nth(CubeColor::Red.as_str().len() - 1);
        Ok(Token::Color(CubeColor::Red))
    } else if iter_contains(input.clone(), CubeColor::Green.as_str().chars()) {
        input.nth(CubeColor::Green.as_str().len() - 1);
        Ok(Token::Color(CubeColor::Green))
    } else if iter_contains(input.clone(), CubeColor::Blue.as_str().chars()) {
        input.nth(CubeColor::Blue.as_str().len() - 1);
        Ok(Token::Color(CubeColor::Blue))
    } else {
        bail!("Did not get a valid color")
    }
}

fn next_separator(input: &mut Chars<'_>) -> Result<Token, anyhow::Error> {
    // println!("Next separator input: ");
    // dbg!(input.clone().collect::<String>());

    if iter_contains(input.clone(), Separator::Showings.as_str().chars()) {
        input.nth(Separator::Showings.as_str().len() - 1);
        Ok(Token::Separator(Separator::Showings))
    } else if iter_contains(input.clone(), Separator::Cubes.as_str().chars()) {
        input.nth(Separator::Cubes.as_str().len() - 1);
        Ok(Token::Separator(Separator::Cubes))
    } else {
        bail!("Expected a separator - ',' or ';'")
    }
}

fn next_game(input: &mut Chars<'_>) -> Result<Token, anyhow::Error> {
    // println!("Next game input: ");
    // dbg!(input.clone().collect::<String>());

    if !iter_contains(input.clone(), "Game".chars()) {
        bail!("Expected 'Game' string");
    }
    input.nth("Game".len() - 1);

    // hijack number tokenization
    let token = if let Token::Number(num) = next_number(input)? {
        Token::Game(num)
    } else {
        bail!("Expected a number after 'Game'")
    };

    match input.next() {
        Some(':') => Ok(token),
        Some(c) => bail!("Expected ':' after Game declaration, found {}", c),
        None => bail!("Unexpected end of input whilst parsing Game. Expected ':'"),
    }
}

fn tokenize_line(line: &str) -> Result<Vec<Token>, anyhow::Error> {
    let mut result = vec![];
    let binding = line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let mut stream = binding.chars();

    result.push(next_game(&mut stream)?);
    loop {
        result.push(next_number(&mut stream)?);
        result.push(next_color(&mut stream)?);

        match next_separator(&mut stream) {
            Ok(token) => result.push(token),
            Err(_) => break,
        }
    }

    Ok(result)
}

#[derive(Debug)]
struct Cube {
    amount: u32,
    color: CubeColor,
}

impl Cube {
    fn valid(&self) -> bool {
        match self.color {
            CubeColor::Red => self.amount <= 12,
            CubeColor::Green => self.amount <= 13,
            CubeColor::Blue => self.amount <= 14,
        }
    }
}

#[derive(Debug)]
struct Game {
    game_number: u32,
    showings: Vec<Vec<Cube>>,
}

impl Game {
    fn valid(&self) -> bool {
        for showing in &self.showings {
            for cube in showing {
                if !cube.valid() {
                    return false;
                }
            }
        }

        true
    }

    fn power(&self) -> u32 {
        let mut max_map = HashMap::new();
        
        for showing in &self.showings {
            for cube in showing {
                max_map.entry(cube.color).and_modify(|max_val| {
                    if *max_val < cube.amount {
                        *max_val = cube.amount
                    }
                }).or_insert(cube.amount);
            }
        }

        
        max_map.iter().fold(1, |acc, (_color, amount)| acc * amount)
    }
}

fn parse_game(mut input: impl Iterator<Item = Token>) -> Result<Game, anyhow::Error> {
    let game_number = if let Some(Token::Game(num)) = input.next() {
        num
    } else {
        bail!("Expected line to start with a game declaration.");
    };

    let mut showings = vec![];
    loop {
        if let Some(showing) = parse_showing(&mut input)? {
            showings.push(showing);
        } else {
            break;
        }
    }

    Ok(Game {
        game_number,
        showings,
    })
}

fn parse_showing(
    input: &mut impl Iterator<Item = Token>,
) -> Result<Option<Vec<Cube>>, anyhow::Error> {
    let mut cube_vec = vec![];
    loop {
        let amount = match input.next() {
            Some(Token::Number(num)) => num,
            Some(token) => bail!("Expected number token, found: {token:?}"),
            None => break Ok(cube_vec),
        };

        let color = if let Some(Token::Color(col)) = input.next() {
            col
        } else {
            bail!("Expected to find a color of a cube");
        };

        cube_vec.push(Cube { amount, color });

        match input.next() {
            Some(Token::Separator(Separator::Cubes)) => continue,
            Some(Token::Separator(Separator::Showings)) | None => break Ok(cube_vec),
            Some(t) => bail!("Expected separator or end of input, got token {:?}", t),
        }
    }
    .map(|res| if res.len() != 0 { Some(res) } else { None })
}

fn main() -> Result<(), anyhow::Error> {
    if env::args().len() != 2 {
        anyhow::bail!("You must pass the input file as a parameter! Example:\nday1 ./input.txt")
    }

    let filename = env::args().nth(1).unwrap();

    let file = std::fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut sum = 0;
    let mut sum_power = 0;

    loop {
        match reader.read_line(&mut buffer)? {
            0 => break,
            _ => {
                let tokens = tokenize_line(&buffer)?;
                let game = parse_game(tokens.into_iter())?;

                println!("Parsed game: {game:?}");

                if game.valid() {
                    println!("Game is valid");
                    sum += game.game_number;
                }

                sum_power += game.power();
            }
        }

        buffer.clear();
    }

    println!("Sum of valid games: {sum}");
    println!("Sum of powers: {sum_power}");

    Ok(())
}
