use std::{
    env, error, fs,
    io::{self, Read, Write},
};
// Enum representing each brainfuck operation with optimization variables.
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;
    // Turn the source code into our intermediate opcode program.
    while i < bytes.len() {
        match bytes[i] as char {
            '<' => {
                let cloned = &bytes[i..bytes.len()];
                let iter = cloned.into_iter().take_while(|x| (**x as char) == '<');
                let iter_len = iter.count();
                prog.push(Ops::Left(iter_len));
                i+=iter_len;
                continue;
            },
            '>' => {
                let cloned = &bytes[i..bytes.len()];
                let iter = cloned.into_iter().take_while(|x| (**x as char) == '>');
                let iter_len = iter.count();
                prog.push(Ops::Right(iter_len));
                i+=iter_len;
                continue;
            },
            '+' => {
                let cloned = &bytes[i..bytes.len()];
                let iter = cloned.into_iter().take_while(|x| (**x as char) == '+');
                let iter_len = iter.count();
                let iter_len_u8: u8 = iter_len.try_into().unwrap();
                prog.push(Ops::Add(iter_len_u8));
                i+=iter_len;
                continue;
            },
            '-' => {
                let cloned = &bytes[i..bytes.len()];
                let iter = cloned.into_iter().take_while(|x| (**x as char) == '-');
                let iter_len = iter.count();
                let iter_len_u8: u8 = iter_len.try_into().unwrap();
                prog.push(Ops::Sub(iter_len_u8));
                i+=iter_len;
                continue;
            },
            '[' => prog.push(Ops::LBrack(usize::max_value())),
            ']' => prog.push(Ops::RBrack(usize::max_value())),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => ()
        }
        i += 1;
    }
    let mut bstack = vec![];
    // Preprocess for brackets
    let mut current_position = 0;
    while current_position < prog.len() {
        match prog[current_position] {
            Ops::LBrack(_value) => {
                bstack.push(current_position);
            }
            Ops::RBrack(_value) => {
                let top = bstack.pop();
                match top {
                    Some(top) => {
                        prog[top] = Ops::LBrack(current_position);
                        prog[current_position] = Ops::RBrack(top);
                    }
                    _ => ()
                }         
            }
            _ => (), /* Ignore non-brackets */
        }
        current_position += 1;
    }


    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left(value) => cc-=value, // shift `cc` to the left by 1.
            Ops::Right(value) => cc+=value, // shift `cc` to the right by 1.
            Ops::Add(value) => cells[cc] += value, // increment value in memory at the `cc` by 1.
            Ops::Sub(value) => cells[cc] -= value, // decrement value in memory at the `cc` by 1.
            Ops::LBrack(value) if cells[cc] == 0 => {
                pc = value;
                continue;
            }
            Ops::RBrack(value) if cells[cc] != 0 => {
                pc = value;
                continue;
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters, treat them as comments */
        }
        pc += 1;
    }
    Ok(())
}
