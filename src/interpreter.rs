use std::{num::Wrapping, borrow::BorrowMut, io::{Write, Read}};

use crate::token::{Token, Tokens};

pub struct Interpreter {
    memory: Vec<Wrapping<u8>>,
    loop_index_stack: Vec<usize>,
    pointer: usize
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            memory: vec![Wrapping(0); 30_000],
            loop_index_stack: Vec::new(),
            pointer: 0
        }
    }
}

impl Interpreter {
    fn alloc_cell(&mut self) {
        self.memory.push(Wrapping(0));
    }

    fn next_cell(&mut self) {
        if self.pointer == self.memory.len() - 1 {
            self.alloc_cell();
        }

        self.pointer += 1;
    }

    fn prev_cell(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.memory.len() - 1;
        }
        else {
            self.pointer -= 1;
        }
    }

    fn current_cell(&mut self) -> &mut Wrapping<u8> {
        self.memory[self.pointer].borrow_mut()
    }

    pub fn run<W, R>(&mut self, tokens: Tokens, mut writer: W, reader: R) 
    where 
        W: Write,
        R: Read
    { 
        let mut index = 0;

        while index < tokens.len() {
            let token = &tokens[index];

            match token {
                Token::IncreaseValue => {
                    self.current_cell().0 += 1;
                },
                Token::DecreaseValue => {
                    self.current_cell().0 -= 1;
                },
                Token::MovePointerToRight => {
                    self.next_cell();
                },
                Token::MovePointerToLeft => {
                    self.prev_cell();
                },
                Token::StartLoop => {
                    if self.current_cell().0 != 0 {
                        self.loop_index_stack.push(index);
                    }
                },
                Token::EndLoop => {
                    if self.current_cell().0 != 0 {
                        index = *self.loop_index_stack.last()
                            .expect("Faild to run the program with invalid loop instructions.");
                    }
                    else {
                        self.loop_index_stack.pop();
                    }
                },
                Token::Read => {
                    todo!();
                },
                Token::Write => {
                    let buffer = [self.current_cell().0];

                    writer.write(&buffer)
                        .expect("Failed to write output.");
                }
            }

            index += 1;
        }
    }
}