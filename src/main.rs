use std::io::Read;
use anyhow::{Result,bail};
use std::fs::File;
use std::env;

// chip8 implementation
mod cpu;
use crate::cpu::MEM_SIZE;
use crate::cpu::TXT_OFFSET;
mod instructions;


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
fn load_program(path: &str, memory: &mut [u8;MEM_SIZE]) -> Result<usize>{

    // sanity check the file size
    let metadata = std::fs::metadata(path)?;
    if !metadata.is_file() && (metadata.len() > (MEM_SIZE-TXT_OFFSET) as u64) {
        bail!("[!] ROM too large to load.")
    }

    // load into memory at TXT_OFFSET
    let mut f = File::open(path)?;
    let len = f.read(&mut memory[TXT_OFFSET..MEM_SIZE])?;
    println!("[+] read {} bytes", len);
    Ok(len)
}


fn debugger(cpu: &mut cpu::Cpu) -> Result<()> {
    use byteorder::{ByteOrder, BigEndian}; 
    use pretty_hex::*;
    use colored::*;
    use linefeed::{Interface, ReadResult};
    let mut last_command = String::new();

    // setup the debugger
    let reader = Interface::new("rchip8-debugger")?;
    reader.set_prompt(format!("{}","rchip8-debugger> ".yellow()).as_str())?;
    cpu.display_debug_info();

    // loop while recieving debugger commands
    while let ReadResult::Input(mut cmd) = reader.read_line()? {

        // add the current command to history
        // and run previous command if no input
        reader.add_history(cmd.clone());
        if cmd.len() == 0 {
            cmd = last_command.to_string();
        }

        match cmd.as_str().trim_end_matches("\r").trim_end_matches("\n") {

            // next instruction
            "ni" => {
                let var = BigEndian::read_u16(&cpu.memory[cpu.program_counter..cpu.program_counter+2]);
                cpu.execute_instruction(var);
                cpu.display_debug_info();
            },

            // display memory
            x if x.starts_with("x") => {
                let data: Vec<&str> = x.split(" ").collect();
                if data.len() < 2 {
                    println!("[-] please provide an address\n");
                    println!("[*] usage: x/100 0x200");
                    continue;
                }
                let mut without_suffix = data[0].split("/").collect::<Vec<&str>>()[1].to_string();
                without_suffix.retain(|x| "0123456789".contains(x));
                let amount = match without_suffix.parse::<u32>(){
                    Ok(v) => v as usize,
                    Err(e) => {
                        println!("[-] invalid byte ammount {:?}\n", e);
                        println!("[*] usage: x/100 0x200");
                        continue; 
                    }
                };
                let without_prefix = data[1].trim_start_matches("0x");
                let addr = match u16::from_str_radix(without_prefix,16) {
                    Ok(v) => v as usize,
                    Err(e) => {
                        println!("[-] invalid memory address {:?}\n", e);
                        continue;
                    }
                };
                if addr > MEM_SIZE {
                    println!("[-] invalid memory address");
                    continue;
                }
                println!("[*] printing memory at 0x{:X} for {} bytes", addr, amount);
                println!("{:?}", &cpu.memory[addr..addr+amount].to_vec().hex_dump());
            },

            // re-display debugger output
            "disp" => {cpu.display_debug_info();continue;}

            // quit
            "q" => {break;},
            v => {println!("[-] unrecognized debugger command: {}", v); continue;}
        }

        // save the last successfull command
        last_command = cmd;
    }
    Ok(())
}

fn main() -> Result <()> {

    let args: Vec<String> = env::args().collect();
    let mut cpu = cpu::Cpu::new();

    if args.len() < 2 {
        println!("[!] Must provide path to rom");
        std::process::exit(-1);
    }

    let _prog_len = load_program(&args[1],&mut cpu.memory)?;

    //disas(&cpu.memory, 10);

    println!("PC: {:?}", cpu.program_counter);
    //cpu.execute_instruction(0x124e);
    debugger(&mut cpu)?;
    println!("PC: {:?}", cpu.program_counter);

    Ok(())
}
