use anyhow::Result;
use std::env;
use rchip8::cpu;

// bring the Debugger Trait in-scope
// so that we may invoke the debugger
mod debugger;
use crate::debugger::Debugger;

// terminal graphics
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

/*
    Memory Map:
    +---------------+= 0xFFF (4095) End of Chip-8 RAM
    |               |
    |               |
    |               |
    |               |
    |               |
    | 0x200 to 0xFFF|
    |     Chip-8    |
    | Program / Data|
    |     Space     |
    |               |
    |               |
    |               |
    +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
    |               |
    |               |
    |               |
    +---------------+= 0x200 (512) Start of most Chip-8 programs
    | 0x000 to 0x1FF|
    | Reserved for  |
    |  interpreter  |
    +---------------+= 0x000 (0) Start of Chip-8 RAM
*/
fn run(mut cpu: cpu::Cpu) -> Result<()> {

   
    loop {

        // fetch & execute instruction
        let inst = cpu.fetch_instruction();
        cpu.execute_instruction(inst);

        // update screen

        // update timers
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut cpu = cpu::Cpu::new();

    if args.len() < 3 {
        println!("[!] Must provide path to rom");
        std::process::exit(-1);
    }

    let _prog_len = cpu.load_program(&args[2])?;

    match args[1].as_str() {
        "run" => run(cpu)?,
        "debug" => cpu.debug()?,
        _ => {
            println!("[-] unrecognized command");
        }
    }

    Ok(())
}
