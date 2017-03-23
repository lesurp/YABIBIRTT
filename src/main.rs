use std::io::{Read, stdin};

enum Command {
    Action(char),
    While(Vec<Command>),
}

fn main() {
    let mut memory = [0 as u8; 100];
    let mut index = 0 as usize;

    while let Some(read_char_res) = stdin().bytes().next() {
        match read_char_res {
            Ok(read_char) => {
                handle_char(read_char as char, &mut index, &mut memory)
            }
            Err(_) => println!("There was an error reading the char"),
        }
    }
}

fn handle_char(c: char, index_ptr: &mut usize, memory: &mut [u8; 100]) {
    match c {
        '>' => *index_ptr += 1,
        '<' => *index_ptr -= 1,
        '+' => memory[*index_ptr] += 1,
        '-' => memory[*index_ptr] -= 1,
        '.' => print!("{}", memory[*index_ptr] as char),
        '[' => {
            handle_while_loop(&while_loop(index_ptr, memory),
                              index_ptr,
                              memory);
        }

        ']' => println!("You fucked up and closed a while before opening one"),
        _ => {}
    }
}

fn while_loop(index_ptr: &mut usize, memory: &mut [u8; 100]) -> Vec<Command> {
    let mut commands = Vec::new();

    while let Some(read_char_res) = stdin().bytes().next() {
        match read_char_res {
            Ok(read_char) => {
                match read_char as char {
                    '[' => commands.push(Command::While(
                            while_loop(index_ptr, memory))),
                    ']' => break,
                    _ => commands.push(Command::Action(read_char as char)),
                }
            }
            Err(_) => println!("There was an error reading the char"),
        }
    }
    commands
}

fn handle_while_loop(loop_command: &Vec<Command>,
                     index_ptr: &mut usize,
                     memory: &mut [u8; 100]) {

    while memory[*index_ptr] != 0 {
        for command in loop_command.iter() {
            match *command {
                Command::Action(c) => handle_char(c, index_ptr, memory),
                Command::While(ref nested_loop_commands) => {
                    handle_while_loop(nested_loop_commands, index_ptr, memory)
                }
            }
        }
    }
}
