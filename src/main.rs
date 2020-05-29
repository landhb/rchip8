use anyhow::Result;
use std::env;

// chip8 implementation
mod cpu;
mod instructions;

// bring the Debugger Trait in-scope
// so that we may invoke the debugger
mod debugger;
use crate::debugger::Debugger;

#[cfg(test)]
mod test_instructions; 


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


fn main() -> Result <()> {

    let args: Vec<String> = env::args().collect();
    let mut cpu = cpu::Cpu::new();

    if args.len() < 3 {
        println!("[!] Must provide path to rom");
        std::process::exit(-1);
    }

    let _prog_len = cpu.load_program(&args[2])?;

    if args[1] == "run" {


    } else if args[1] == "debug" {
        cpu.debug()?;
    } else {
        println!("[-] unrecognized commmand");
    }
    

    Ok(())
}
