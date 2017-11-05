//! A parser

use std::process;

#[derive(Debug, PartialEq)]
pub enum Token {
    Increase,
    Decrease,
    MoveRight,
    MoveLeft,
    Write,
    Read,
    WhileBegin(usize),
    WhileEnd(usize),
}

pub struct Parser {
    pub input: String,
    pub parsed: Vec<Token>
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            input,
            parsed: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.filter_input();
        for i in 0..self.input.len() {
            if let Some(token) = self.parse_token(i) {
                self.parsed.push(token);
            }
        }
    }

    fn filter_input(&mut self) {
        let input: String = self.input.chars()
            .filter(|c| {
                match *c {
                    '+' | '-' | '>' | '<' | '.' | ',' | '[' | ']' => true,
                    _ => false,
                }
            }).collect();
        self.input = input;
    }

    pub fn parse_token(&self, index: usize) -> Option<Token> {
        let input_vec: Vec<char> = self.input.chars().collect();
        match input_vec[index] {
            '+' => Some(Token::Increase),
            '-' => Some(Token::Decrease),
            '>' => Some(Token::MoveRight),
            '<' => Some(Token::MoveLeft),
            '.' => Some(Token::Write),
            ',' => Some(Token::Read),
            '[' => {
                if let Some(closing_bracket) = find_closing_bracket(&self.input, index) {
                    Some(Token::WhileBegin(closing_bracket))
                } else {
                    eprintln!("Error: couldn't find closing bracket for opening bracket at {}", index);
                    process::exit(1);
                }
            },
            ']' => {
                if let Some(opening_bracket) = find_opening_bracket(&self.input, index) {
                    Some(Token::WhileEnd(opening_bracket))
                } else {
                    eprintln!("Error: couldn't find opening bracket for closing bracket at {}", index);
                    process::exit(1);
                }
            },
            _ => None,
        }
    }
}

fn find_closing_bracket(input: &String, index: usize) -> Option<usize> {
    let input: Vec<char> = input.chars().collect();
    let mut opened = 0;
    for i in index + 1..input.len() {
        match input[i] {
            '[' => opened += 1,
            ']' => {
                if opened == 0 {
                    return Some(i);
                } else {
                    opened -= 1;
                }
            },
            _ => {},
        };
    }

    None
}

fn find_opening_bracket(input: &String, index: usize) -> Option<usize> {
    let input: Vec<char> = input.chars().collect();
    let mut closed = 0;
    for i in (0..index).rev() {
        match input[i] {
            ']' => closed += 1,
            '[' => {
                if closed == 0 {
                    return Some(i);
                } else {
                    closed -= 1;
                }
            },
            _ => {},
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let mut parser = Parser::new(String::from(program));
        parser.parse();
        assert_eq!(parser.parsed.len(), program.len());
    }

    #[test]
    fn some_token() {
        let parser = Parser::new(String::from("."));
        assert!(parser.parse_token(0).is_some());
    }

    #[test]
    fn none_token() {
        let parser = Parser::new(String::from("x"));
        assert!(parser.parse_token(0).is_none());
    }

    #[test]
    fn initial_comment_loop() {
        let program = "[This is an initial comment loop]";
        let mut parser = Parser::new(String::from(program));
        parser.parse();
        assert_eq!(parser.parsed.len(), 2);
    }
}
