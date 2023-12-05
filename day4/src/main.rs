use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader},
    str::Chars,
};

use itertools::Itertools;
use stable_eyre::eyre::{eyre, Report};

#[derive(Debug)]
enum Token {
    Card(u32),
    Number(u32),
    Separator,
}

#[derive(Debug, thiserror::Error)]
enum TokenizationError {
    #[error("Expected a declaration of a card (+Card+ <number>:), but failed to find it.")]
    ExpectedCardDeclaration,
    #[error("Expected a number for the card (Card +<number>+:), but failed to find it.")]
    ExpectedCardNumber,
    #[error("Expected a separator after the card declaration (Card <number>+:+), but failed to find it.")]
    ExpectedCardSeparator,
    #[error("Expected a separator between numbers (<winning numbers> +|+ <card number>), but failed to find it.")]
    ExpectedInputSeparator,
    #[error("Input ended unexpectedly.")]
    UnexpectedEndOfInput,
}

#[derive(Debug, thiserror::Error)]
enum ParsingError {
    #[error("Expected a card, but failed to find it.")]
    ExpectedCardToken,
    #[error("Expected a number, but failed to find it.")]
    ExpectedNumberToken,
    #[error("Expected a separator, but failed to find it.")]
    ExpectedSeparatorToken,
}
    

fn next_number(
    input: &mut (impl Iterator<Item = char> + Clone + itertools::PeekingNext),
) -> Result<Token, Report> {
    let number_str: String = input
        .peeking_take_while(|chr| chr.is_ascii_digit())
        .collect();

    Ok(Token::Number(number_str.parse()?))
}

fn iter_contains(
    mut haystack: impl Iterator<Item = char> + Clone,
    mut needle: impl Iterator<Item = char>,
) -> bool {
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

fn skip_whitespace(input: &mut (impl Iterator<Item = char> + itertools::PeekingNext)) {
    input
        .peeking_take_while(|chr| chr.is_whitespace())
        .for_each(|_| {})
}

fn next_card(
    input: &mut (impl Iterator<Item = char> + Clone + itertools::PeekingNext),
) -> Result<Token, Report> {
    if !iter_contains(input.clone(), "Card".chars()) {
        return Err(eyre!(TokenizationError::ExpectedCardDeclaration));
    }
    input.nth("Card".len() - 1);
    skip_whitespace(input);

    // hijack number tokenization
    let token = if let Token::Number(num) = next_number(input)? {
        Token::Card(num)
    } else {
        return Err(eyre!(TokenizationError::ExpectedCardNumber));
    };

    match input.next() {
        Some(':') => Ok(token),
        Some(c) => Err(eyre!(TokenizationError::ExpectedCardSeparator)),
        None => Err(eyre!(TokenizationError::UnexpectedEndOfInput)),
    }
}

fn next_separator_peeking(input: &mut (impl Iterator<Item = char> + Clone)) -> Result<Token, Report> {
    match input.clone().peekable().peek() {
        Some('|') => {
            input.next();
            Ok(Token::Separator)
        }
        Some(c) => Err(eyre!(TokenizationError::ExpectedInputSeparator)),
        None => Err(eyre!(TokenizationError::UnexpectedEndOfInput)),
    }
}

fn tokenize_line(line: &str) -> Result<Vec<Token>, Report> {
    let mut result = vec![];
    let mut stream = line.chars();

    // println!("Passing to 'next_card': '{}'", stream.clone().collect::<String>());


    result.push(next_card(&mut stream)?);
    skip_whitespace(&mut stream);
    loop {
        // println!("Passing to 'next_number': '{}'", stream.clone().collect::<String>());
        result.push(next_number(&mut stream)?);
        skip_whitespace(&mut stream);

        // println!("Passing to 'next_separator_peeking': '{}'", stream.clone().collect::<String>());
        match next_separator_peeking(&mut stream).map_err(|e| e.downcast::<TokenizationError>()) {
            Ok(token) => { 
                result.push(token);
                skip_whitespace(&mut stream);
            },
            Err(Ok(TokenizationError::ExpectedInputSeparator)) => {}
            Err(Ok(TokenizationError::UnexpectedEndOfInput)) => break,
            Err(e) => return Err(e?.into()),
        }
        // println!("");
    }

    Ok(result)
}


#[derive(Debug)]
struct Card {
    card_id: u32,
    winning_nums: Vec<u32>,
    card_nums: Vec<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let mut points = 0;

        for num in &self.card_nums {
            if self.winning_nums.contains(num) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        // println!("Card {} has {} points", self.card_id, points);

        points
    }
}

fn parse_card(mut input: impl Iterator<Item = Token>) -> Result<Card, Report> {
    let card_id = if let Some(Token::Card(num)) = input.next() {
        num
    } else {
        return Err(eyre!(ParsingError::ExpectedCardToken));
    };

    let mut winning_nums = vec![];
    let mut card_nums = vec![];
    loop {
        let num = match input.next() {
            Some(Token::Number(num)) => num,
            Some(Token::Separator) => break,
            Some(_) | None => return Err(eyre!(ParsingError::ExpectedNumberToken)),
        };

        winning_nums.push(num);
    }

    loop {
        let num = match input.next() {
            Some(Token::Number(num)) => num,
            Some(_) => return Err(eyre!(ParsingError::ExpectedNumberToken)),
            None => break,
        };

        card_nums.push(num);
    }

    Ok(Card {
        card_id,
        winning_nums,
        card_nums,
    })
}

#[derive(Debug, thiserror::Error)]
enum Day4Error {
    #[error("You must pass the input file as a parameter! Example:\nday4 ./input.txt")]
    InvalidArgs,
}

fn main() -> Result<(), Report> {
    if env::args().len() != 2 {
        return Err(eyre!(Day4Error::InvalidArgs));
    }

    let filename = env::args().nth(1).unwrap();

    let file = std::fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    let mut sum = 0;

    loop {
        match reader.read_line(&mut buffer)? {
            0 => break,
            _ => {
                let tokens = tokenize_line(&buffer)?;
                let card = parse_card(tokens.into_iter())?;

                println!("Parsed card: {card:?}");

                sum += card.points();
            }
        }

        buffer.clear();
    }

    println!("Sum of card points: {sum}");

    Ok(())
}
