use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;
    let mut pc = 0; /* program instruction pointer */
    let mut cells = vec![0u8; 10000]; /* memory block */
    let mut cc = 0; /* Cell pointer*/

    while pc < prog.len() { 
        match prog[pc] as char {
            '<' => cc = cc-1, // shift `cc` to the left by 1.
            '>' => cc = cc+1, // shift `cc` to the right by 1.
            '+' => cells[cc] = cells[cc] + 1, // increment value in memory at the `cc` by 1.
            '-' => cells[cc] = cells[cc] - 1, // decrement value in memory at the `cc` by 1.
            '[' if cells[cc] == 0 => {
                let mut nest_count = 1; // Increase nest depth if we find another same bracket, decrement if 
                let mut scan = pc+1;
                while scan < prog.len() {
                    match prog[scan] as char {
                        '[' => nest_count+=1, // Increase nest depth
                        ']' => nest_count-=1, // Decrease nest depth
                        _ => ()
                    }
                    if nest_count==0 {
                        // Found matching bracket
                        pc = scan;
                        break;
                    }
                    scan+=1;
                }
                continue;
            }
            ']' if cells[cc] != 0 => {
                let mut nest_count = 1; // Increase nest depth if we find another same bracket, decrement if 
                let mut scan = pc-1;
                while scan > 0 {
                    match prog[scan] as char {
                        ']' => nest_count+=1, // Increase nest depth
                        '[' => nest_count-=1, // Decrease nest depth
                        _ => ()
                    }
                    if nest_count==0 {
                        // Found matching bracket
                        pc = scan;
                        break;
                    }
                    scan-=1;
                }
                continue;
            }
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
