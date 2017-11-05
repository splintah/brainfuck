extern crate ncurses;

use parser::{Parser, Token};

pub struct Interpreter;

impl Interpreter {
    pub fn run(parser: Parser) -> Result<String, &'static str> {
        let parsed = parser.parsed;
        let takes_input = parsed.contains(&Token::Read);

        let mut memory: Vec<u8> = vec![0];
        let mut pointer: usize = 0;

        let mut output = String::new();

        // ncurses is only used for input, so don't use it when no input is expected.
        if takes_input {
            ncurses::initscr();
        }

        let mut i: usize = 0;
        while i < parsed.len() {
            match parsed[i] {
                Token::Increase => {
                    if memory[pointer] == u8::max_value() {
                        memory[pointer] = u8::min_value();
                    } else {
                        memory[pointer] += 1;
                    }
                },
                Token::Decrease => {
                    if memory[pointer] == u8::min_value() {
                        memory[pointer] = u8::max_value();
                    } else {
                        memory[pointer] -= 1;
                    }
                },
                Token::MoveRight => {
                    pointer += 1;
                    if memory.len() <= pointer {
                        memory.push(0);
                    }
                },
                Token::MoveLeft => {
                    if pointer == 0 {
                        return Err("pointer underflow.");
                    }
                    pointer -= 1;
                },
                Token::Write => {
                    // Don't print EOF.
                    if memory[pointer] > 0 {
                        let ch = memory[pointer] as char;
                        if takes_input {
                            ncurses::printw(format!("{}", ch).as_str());
                        }
                        output.push(ch);
                    }
                },
                Token::Read => {
                    let mut ch = ncurses::getch() as u8;
                    // ^D equals 4, but should equal 0.
                    if ch == 4 {
                        ch = 0;
                    }
                    memory[pointer] = ch;
                },
                Token::WhileBegin(end) => {
                    if memory[pointer] == 0 {
                        i = end;
                    }
                },
                Token::WhileEnd(begin) => {
                    if memory[pointer] != 0 {
                        i = begin;
                    }
                },
            }
            i += 1;
            if takes_input {
                ncurses::refresh();
            }
        }

        if takes_input {
            ncurses::endwin();
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let program = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        let mut parser = Parser::new(program);
        parser.parse();
        assert_eq!(Interpreter::run(parser), Ok(String::from("Hello World!\n")));
    }

    #[test]
    fn complexer_hello_world() {
        // https://esolangs.org/wiki/Brainfuck-ng#Examples
        let program = String::from(">++++++++[<+++++++++>-]<.>>+>+>++>[-]+<[>[->+<<++++>]<<]>.+++++++..+++.>>+++++++.<<<[[-]<[-]>]<+++++++++++++++.>>.+++.------.--------.>>+.>++++.");
        let mut parser = Parser::new(program);
        parser.parse();
        assert_eq!(Interpreter::run(parser), Ok(String::from("Hello World!\n")));
    }

    #[test]
    fn pointer_underflow() {
        let program = String::from("<");
        let mut parser = Parser::new(program);
        parser.parse();
        assert!(Interpreter::run(parser).is_err());
    }

    #[test]
    fn memory_underflow() {
        let program = String::from("-");
        let mut parser = Parser::new(program);
        parser.parse();
        assert!(Interpreter::run(parser).is_ok());
    }
}
