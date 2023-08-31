use std::{io, env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: brainf <source>");
        process::exit(1);
    }

    let instructions = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Failed to read the source file. please check file permissions or its existance.");
            process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new(&instructions);

    interpreter.interpret();
}

type Tokens = Vec<Token>;

enum Token {
    IncreaseValue,
    DecreaseValue,
    MovePointerToRight,
    MovePointerToLeft,
    JumpForward,
    JumpBackward,
    Input,
    Output
}

struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
    tokens: Tokens
}

impl Interpreter {
    fn new(instructions: &str) -> Self {
        let tokens = Self::tokenize(instructions);

        Self {
            memory: Vec::new(),
            pointer: 0,
            tokens
        }
    }

    fn tokenize(instructions: &str) -> Tokens {
        let mut tokens: Tokens = Vec::new();
        let characters: Vec<char> = instructions.chars().collect();

        for character in characters {
            let token = match character {
                '+' => Token::IncreaseValue,
                '-' => Token::DecreaseValue,
                '>' => Token::MovePointerToRight,
                '<' => Token::MovePointerToLeft,
                '[' => Token::JumpForward,
                ']' => Token::JumpBackward,
                '.' => Token::Output,
                ',' => Token::Input,
                _ => {
                    continue;
                }
            };

            tokens.push(token);
        }

        tokens
    }

    fn interpret(&mut self) {
        let mut jump_forward_index_stack: Vec<usize> = Vec::new();
        let mut index = 0;

        while index < self.tokens.len() {
            let token = &self.tokens[index];
            let current_pointer = self.pointer;

            if current_pointer >= self.memory.len() {
                self.memory.push(0);   
            }

            match token {
                Token::IncreaseValue => {
                    self.memory[current_pointer] += 1;
                },
                Token::DecreaseValue => {
                    self.memory[current_pointer] -= 1;
                },
                Token::MovePointerToRight => {
                    self.pointer += 1;
                },
                Token::MovePointerToLeft => {
                    self.pointer -= 1;
                },
                Token::JumpForward => {
                    jump_forward_index_stack.push(index);
                },
                Token::JumpBackward => {
                    if self.memory[current_pointer] != 0 {
                        let jump_forward_index = jump_forward_index_stack.pop();

                        match jump_forward_index {
                            Some(jump_forward_index) => {
                                index = jump_forward_index;
                                jump_forward_index_stack.push(index);
                            },
                            None => {
                                eprintln!("Failed with invalid loop instruction.");
                            }
                        }
                    }
                },
                Token::Output => {
                    let ascii_char = char::from_u32(self.memory[current_pointer] as u32)
                        .expect("Can't convert cell value to ASCII character.");

                    print!("{}", ascii_char);
                },
                Token::Input => {
                    let mut input = String::new();

                    io::stdin().read_line(&mut input)
                        .expect("Failed to read from STDIN.");

                    let characters_length = input.len();

                    for (i, character) in input.chars().enumerate() {
                        if character.is_ascii() {
                            self.memory[current_pointer] = character as u8;

                            if i != characters_length - 1 {                               
                                self.pointer += 1;

                                if self.pointer >= self.memory.len() {
                                    self.memory.push(0);   
                                }
                            }
                        }
                    }
                }
            }

            index += 1;
        }

        if jump_forward_index_stack.len() != 0 {
            eprintln!("Failed with invalid loop instruction.");
        }
    }
}