use std::{
    env, error, fs,
    io::{self, Read, Write},
    collections::HashMap
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    let mut bmap = HashMap::new(); // Map from a position in the program to the jump location [index->jump location]
    let mut bstack = vec![]; // Nested brackets in preprocessing
    // Preprocess the source code
    let mut current_position = 0;
    while current_position < prog.len() {
        match prog[current_position] as char {
            '[' => {
                // Add position to the stack
                bstack.push(current_position);
            }
            ']' => {
                let top = bstack.pop(); // Gets the matching bracket location from the bracket
                                        // stack
                match top {
                    Some(top) => {
                        // Add the birdirectional mapping for the set of matching brackets.
                        bmap.insert(current_position, top);
                        bmap.insert(top, current_position);
                    }
                    _ => ()
                }         
            }
            _ => (), /* Ignore any other characters in preprocessing */
        }
        current_position += 1;
    }


    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] as char {
            '<' => cc = cc-1, // shift `cc` to the left by 1.
            '>' => cc = cc+1, // shift `cc` to the right by 1.
            '+' => cells[cc] = cells[cc] + 1, // increment value in memory at the `cc` by 1.
            '-' => cells[cc] = cells[cc] - 1, // decrement value in memory at the `cc` by 1.
            '[' if cells[cc] == 0 => {
                // We can look at where we are in the current program, and look up where that leads to in the bracket jump cache hashmap.
                pc = bmap[&pc];
                continue;
            }
            ']' if cells[cc] != 0 => {
                // We can look at where we are in the current program, and look up where that leads to in the bracket jump cache hashmap.
                pc = bmap[&pc];
                continue;
            }
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters, they're treated as comments. */
        }
        pc += 1;
    }
    Ok(())
}
