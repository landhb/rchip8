use std::io::Read;
use anyhow::{Result,bail};
use std::fs::File;
use std::env;
use byteorder::{ByteOrder, BigEndian}; 

// chip8 implementation
mod cpu;
use crate::cpu::MEM_SIZE;
use crate::cpu::TXT_OFFSET;


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

fn print_instruction(line: usize, instr: &str, comments: &str) {
    if comments.len() > 0 {
        println!("0x{:x} <+{:?}>:\t{}\t\t;{}", TXT_OFFSET+line, line*2, instr, comments);
    }
    else {
        println!("0x{:x} <+{:?}>:\t{}\t", TXT_OFFSET+line, line*2, instr);
    }
}


fn disas(data: &[u8;MEM_SIZE], len: usize) {
    let text_section: &[u8] = &data[TXT_OFFSET..TXT_OFFSET+len];
    for (i,v) in text_section.chunks_exact(2).enumerate() { 
        
        // convert to an opcode
        let var = BigEndian::read_u16(&v);
        let nums = [(var >> 12) & 0xF, (var >> 8) & 0xF, (var >> 4) & 0xF, var & 0xF];

        match nums {
            [0,0,0xE,0] => {print_instruction(i,"CLS","clear screen");},
            [0,0,0xE,0xE] => {print_instruction(i,"RET","");},
            [0,n,_,_] => {print_instruction(i,&format!("SYS 0x{:X}{:X}", n,v[1]),"");},
            [1,n,_,_] => {print_instruction(i,&format!("JP 0x{:X}{:X}", n,v[1]),&format!("jump to 0x{:X}{:X}",n,v[1]));},
            [2,n,_,_] => {print_instruction(i,&format!("CALL 0x{:X}{:X}", n,v[1]),"");},
            [3,x,_,_] => {print_instruction(i,&format!("SE V{}, {:X}", x,v[1]),"");}, // skip instruction if Vx == kk
            [4,x,_,_] => {print_instruction(i,&format!("SNE V{}, {:X}", x,v[1]),"");}, // skip instruction if Vx != kk
            [5,x,y,_] => {print_instruction(i,&format!("SNE V{}, V{}", x,y),"");}, // skip instruction if Vx == Vy
            [6,x,_,_] => {print_instruction(i,&format!("LD V{}, {:X}", x,v[1]),"");}, // load kk into Vx
            [7,x,_,_] => {print_instruction(i,&format!("ADD V{}, {:X}", x,v[1]),"");}, // add kk to Vx
            [8,x,y,0] => {print_instruction(i,&format!("LD V{}, V{}", x,y),"");}, // set Vx = Vy
            [8,x,y,1] => {print_instruction(i,&format!("OR V{}, V{}", x,y),"");}, // Vx = Vx OR Vy
            [8,x,y,2] => {print_instruction(i,&format!("AND V{}, V{}", x,y),"");}, // Vx = Vx AND Vy
            [8,x,y,3] => {print_instruction(i,&format!("XOR V{}, V{}", x,y),"");}, // Vx = Vx XOR Vy
            [8,x,y,4] => {print_instruction(i,&format!("ADD V{}, V{}", x,y),"");}, // Vx = Vx + Vy, VF = carry
            [8,x,y,5] => {print_instruction(i,&format!("SUB V{}, V{}", x,y),"");}, // Vx = Vx - Vy, set VF = NOT borrow.
            [8,x,y,6] => {print_instruction(i,&format!("SHR V{}, {{,V{}}}", x,y),"");}, // shift right
            [8,x,y,7] => {print_instruction(i,&format!("SUBN V{}, V{}", x,y),"");}, 
            [8,x,y,0xE] => {print_instruction(i,&format!("SHL V{}, V{}", x,y),"");}, //shift left
            [9,x,y,0] => {print_instruction(i,&format!("SNE V{}, V{}", x,y),"");}, // same as before?
            [0xA,n,_,_] => {print_instruction(i,&format!("LD I, 0x{:X}{:X}", n,v[1]),"");},
            [0xB,n,_,_] => {print_instruction(i,&format!("JP V0, 0x{:X}{:X}", n,v[1]),"");},
            [0xC,x,_,_] => {print_instruction(i,&format!("JP V{}, 0x{:X}", x,v[1]),"");},
            [0xD,x,y,n] => {print_instruction(i,&format!("DRW V{}, V{}, {:X}", x,y,n),"");},
            [0xE,x,9,0xE] => {print_instruction(i,&format!("SKP V{}", x),"");},
            [0xE,x,0xA,1] => {print_instruction(i,&format!("SKNP V{}", x),"");},
            [0xF,x,0,7] => {print_instruction(i,&format!("LD V{}, DT", x),"");},
            [0xF,x,0,0xA] => {print_instruction(i,&format!("LD V{}, K", x),"");},
            [0xF,x,1,5] => {print_instruction(i,&format!("LD DT, V{}", x),"");},
            [0xF,x,1,8] => {print_instruction(i,&format!("LD ST, V{}", x),"");},
            [0xF,x,1,0xE] => {print_instruction(i,&format!("ADD I, V{}", x),"");},
            [0xF,x,2,9] => {print_instruction(i,&format!("LD F, V{}", x),"");},
            [0xF,x,3,3] => {print_instruction(i,&format!("LD B, V{}", x),"");},
            [0xF,x,5,5] => {print_instruction(i,&format!("LD [I], V{}", x),"");},
            [0xF,x,6,5] => {print_instruction(i,&format!("LD V{}, [I]", x),"");},
            [_,_,_,_] => {println!("{:X}", var); unimplemented!();}, 
        } 
    }
}

fn main() -> Result <()> {

    let args: Vec<String> = env::args().collect();
    let mut cpu = cpu::Cpu::new();

    if args.len() < 2 {
        println!("[!] Must provide path to rom");
        std::process::exit(-1);
    }

    let _prog_len = load_program(&args[1],&mut cpu.memory)?;

    disas(&cpu.memory, 10);

    println!("PC: {:?}", cpu.program_counter);
    cpu.execute_instruction(0x124e);
    println!("PC: {:?}", cpu.program_counter);

    Ok(())
}
