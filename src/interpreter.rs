use std::{num::Wrapping, borrow::BorrowMut, io::{Write, Read}, collections::HashMap};

use crate::token::{Token, Tokens};

type JumpTable = HashMap<usize, usize>;

pub struct Interpreter {
    memory: Vec<Wrapping<u8>>,
    pointer: usize
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            memory: vec![Wrapping(0); 30_000],
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

    pub fn create_jump_table(&mut self, tokens: &Tokens) -> JumpTable {
        let mut jump_table: JumpTable = HashMap::new();
        let mut start_loop_index_stack: Vec<usize> = Vec::new();

        for (index, token) in tokens.iter().enumerate() {
            match token {
                Token::StartLoop => {
                    start_loop_index_stack.push(index);
                },
                Token::EndLoop => {
                    let found_start_loop_index = start_loop_index_stack.pop()
                        .expect("Failed to create jump table.");

                    jump_table.insert(found_start_loop_index, index);
                    jump_table.insert(index, found_start_loop_index);
                }
                _ => continue
            }
        }

        jump_table 
    }

    pub fn run<W, R>(
        &mut self,
        tokens: &Tokens,
        jump_table: &JumpTable,
        writer: &mut W,
        reader: &mut R
    ) 
    where 
        W: Write,
        R: Read
    { 
        let mut index = 0;

        while index < tokens.len() {
            let token = &tokens[index];

            match token {
                Token::IncreaseValue => {
                    *self.current_cell() += 1;
                },
                Token::DecreaseValue => {
                    *self.current_cell() -= 1;
                },
                Token::MovePointerToRight => {
                    self.next_cell();
                },
                Token::MovePointerToLeft => {
                    self.prev_cell();
                },
                Token::StartLoop => {
                    if self.current_cell().0 == 0 {
                        index = *jump_table.get(&index).unwrap();
                    }
                },
                Token::EndLoop => {
                    if self.current_cell().0 != 0 {
                        index = *jump_table.get(&index).unwrap();
                    }
                },
                Token::Read => {
                    let mut buffer = [0u8; 1];

                    reader
                        .take(1)
                        .read_exact(&mut buffer)
                        .expect("Failed to read from the reader.");

                    *self.current_cell() = Wrapping(buffer[0]);
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