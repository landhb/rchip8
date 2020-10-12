/*
 * Debugger Trait for our Cpu struct
 */

use crate::cpu::{FLAG_REGISTER,TXT_OFFSET,MEM_SIZE};
use anyhow::Result;


pub trait Debugger {

    // print an instruction
    fn print_instruction(&self, line:usize, instr: &str, comments: &str);

    // display the stack/registers/disassembly of the rom
    fn display_debug_info(&self);

    // dissassemble the next len instructions
    fn disas(&self, len: usize);

    // invoke the debugger
    fn debug(&mut self) -> Result<()>;
}


impl Debugger for crate::cpu::Cpu {

     fn print_instruction(&self, line: usize, instr: &str, comments: &str) {
        use colored::*;
        let addr = (self.program_counter-2)+(line*2);
        let offset: i32 = (addr as i32)-(TXT_OFFSET as i32);

        if comments.len() > 0 {
            println!("0x{:x} <+{:?}>:\t{}\t\t{}", addr, offset, instr.blue().bold(), comments.blue().bold());
        }
        else {
            println!("0x{:x} <+{:?}>:\t{}\t", addr, offset, instr);
        }
    }



    fn display_debug_info(& self) {
        use colored::*;
        println!("────────────────────────────[ REGISTERS ]──────────────────────────────────");
        for i in 0..16 {
            if i == FLAG_REGISTER {
                 println!("{} 0x{:02x}", format!("V{:X}:",i).yellow(), self.registers[i]);
            } else {
                println!("{} 0x{:02x}", format!("V{:X}:",i).red(), self.registers[i]);  
            }
        }
        println!("{} 0x{:02x}", "DT:".yellow(), self.dt_register);
        println!("{} 0x{:02x}", "ST:".yellow(), self.st_register);
        println!("{} 0x{:04x}", " I:".yellow(), self.i_register);

        println!("────────────────────────────[  DISASM   ]──────────────────────────────────");
        self.disas(15);
        println!("─────────────────────────────[  STACK  ]───────────────────────────────────");
        if self.stack.len() > 0 {
            for (offset,value) in self.stack.iter().rev().enumerate() {
                println!("{:?}: 0x{:03x}", offset, value);
            }
        } else {
            println!("{}", "<Empty>".red());
        }
    }


    fn disas(&self, len: usize) {
        use byteorder::{ByteOrder, BigEndian}; 

        let text_section: &[u8] = &self.memory[self.program_counter-2..self.program_counter+len];
        for (i,v) in text_section.chunks_exact(2).enumerate() { 
            
            // convert to an opcode
            let var = BigEndian::read_u16(&v);
            let nums = [(var >> 12) & 0xF, (var >> 8) & 0xF, (var >> 4) & 0xF, var & 0xF];
            let mut comments = "";

            if (self.program_counter-2)+(i*2) == self.program_counter {
                comments = " <-- here";
            }
            match nums {
                [0,0,0xE,0] => {self.print_instruction(i,"CLS",comments);},
                [0,0,0xE,0xE] => {self.print_instruction(i,"RET",comments);},
                [0,n,_,_] => {self.print_instruction(i,&format!("SYS 0x{:X}{:X}", n,v[1]),comments);},
                [1,n,_,_] => {self.print_instruction(i,&format!("JP 0x{:X}{:X}", n,v[1]),comments);},
                [2,n,_,_] => {self.print_instruction(i,&format!("CALL 0x{:X}{:X}", n,v[1]),comments);},
                [3,x,_,_] => {self.print_instruction(i,&format!("SE V{:X}, 0x{:X}", x,v[1]),comments);}, // skip instruction if Vx == kk
                [4,x,_,_] => {self.print_instruction(i,&format!("SNE V{:X}, 0x{:X}", x,v[1]),comments);}, // skip instruction if Vx != kk
                [5,x,y,_] => {self.print_instruction(i,&format!("SE V{:X}, V{:X}", x,y),comments);}, // skip instruction if Vx == Vy
                [6,x,_,_] => {self.print_instruction(i,&format!("LD V{:X}, {:X}", x,v[1]),comments);}, // load kk into Vx
                [7,x,_,_] => {self.print_instruction(i,&format!("ADD V{:X}, {:X}", x,v[1]),comments);}, // add kk to Vx
                [8,x,y,0] => {self.print_instruction(i,&format!("LD V{:X}, V{:X}", x,y),comments);}, // set Vx = Vy
                [8,x,y,1] => {self.print_instruction(i,&format!("OR V{:X}, V{:X}", x,y),comments);}, // Vx = Vx OR Vy
                [8,x,y,2] => {self.print_instruction(i,&format!("AND V{:X}, V{:X}", x,y),comments);}, // Vx = Vx AND Vy
                [8,x,y,3] => {self.print_instruction(i,&format!("XOR V{:X}, V{:X}", x,y),comments);}, // Vx = Vx XOR Vy
                [8,x,y,4] => {self.print_instruction(i,&format!("ADD V{:X}, V{:X}", x,y),comments);}, // Vx = Vx + Vy, VF = carry
                [8,x,y,5] => {self.print_instruction(i,&format!("SUB V{:X}, V{:X}", x,y),comments);}, // Vx = Vx - Vy, set VF = NOT borrow.
                [8,x,y,6] => {self.print_instruction(i,&format!("SHR V{:X}, {{,V{:X}}}", x,y),comments);}, // shift right
                [8,x,y,7] => {self.print_instruction(i,&format!("SUBN V{:X}, V{:X}", x,y),comments);}, 
                [8,x,y,0xE] => {self.print_instruction(i,&format!("SHL V{:X}, V{:X}", x,y),comments);}, //shift left
                [9,x,y,0] => {self.print_instruction(i,&format!("SNE V{:X}, V{:X}", x,y),comments);}, // same as before?
                [0xA,n,_,_] => {self.print_instruction(i,&format!("LD I, 0x{:X}{:X}", n,v[1]),comments);},
                [0xB,n,_,_] => {self.print_instruction(i,&format!("JP V0, 0x{:X}{:X}", n,v[1]),comments);},
                [0xC,x,_,_] => {self.print_instruction(i,&format!("JP V{:X}, 0x{:X}", x,v[1]),comments);},
                [0xD,x,y,n] => {self.print_instruction(i,&format!("DRW V{:X}, V{:X}, {:X}", x,y,n),comments);},
                [0xE,x,9,0xE] => {self.print_instruction(i,&format!("SKP V{:X}", x),comments);},
                [0xE,x,0xA,1] => {self.print_instruction(i,&format!("SKNP V{:X}", x),comments);},
                [0xF,x,0,7] => {self.print_instruction(i,&format!("LD V{:X}, DT", x),comments);},
                [0xF,x,0,0xA] => {self.print_instruction(i,&format!("LD V{:X}, K", x),comments);},
                [0xF,x,1,5] => {self.print_instruction(i,&format!("LD DT, V{:X}", x),comments);},
                [0xF,x,1,8] => {self.print_instruction(i,&format!("LD ST, V{:X}", x),comments);},
                [0xF,x,1,0xE] => {self.print_instruction(i,&format!("ADD I, V{:X}", x),comments);},
                [0xF,x,2,9] => {self.print_instruction(i,&format!("LD F, V{:X}", x),comments);},
                [0xF,x,3,3] => {self.print_instruction(i,&format!("LD B, V{:X}", x),comments);},
                [0xF,x,5,5] => {self.print_instruction(i,&format!("LD [I], V{:X}", x),comments);},
                [0xF,x,6,5] => {self.print_instruction(i,&format!("LD V{:X}, [I]", x),comments);},
                [_,_,_,_] => {println!("{:X}", var); unimplemented!();}, 
            } 
        }
    }


    fn debug(&mut self) -> Result<()> {
        use byteorder::{ByteOrder, BigEndian}; 
        use pretty_hex::*;
        use colored::*;
        use linefeed::{Interface, ReadResult};
        let mut last_command = String::new();

        // setup the debugger
        let reader = Interface::new("rchip8-debugger")?;
        reader.set_prompt(format!("{}","rchip8-debugger> ".yellow()).as_str())?;
        self.display_debug_info();

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
                    let var = BigEndian::read_u16(&self.memory[self.program_counter..self.program_counter+2]);
                    self.execute_instruction(var);
                    self.display_debug_info();
                },

                // display memory
                x if x.starts_with("x") => {
                    let data: Vec<&str> = x.split(" ").collect();
                    if data.len() < 2 {
                        println!("[-] please provide an address");
                        println!("[*] usage: x/100 0x200");
                        continue;
                    }
                    let without_suffix = data[0].split("/").collect::<Vec<&str>>();
                    if without_suffix.len() < 2 {
                        println!("[-] usage: x/[BYTES] [ADDR]");
                        continue;
                    }
                    let mut without_suffix = without_suffix[1].to_string();
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
                    println!("{:?}", &self.memory[addr..addr+amount].to_vec().hex_dump());
                },

                // jump to instruction
                x if x.starts_with("jmp") || x.starts_with("jump") => {
                    let data: Vec<&str> = x.split(" ").collect();
                    if data.len() < 2 {
                        println!("[-] please provide an address");
                        println!("[*] usage: jmp 0x200");
                        continue;
                    }
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
                    self.program_counter = addr;
                    println!("[+] set pc to {:04X}", addr);
                    self.display_debug_info();
                }

                // re-display debugger output
                "disp" => {self.display_debug_info();}

                // re-display debugger output
                "help" => {
                    println!("rchip8-debugger\n");
                    println!("commands:");
                    println!("\tni        - Next Instruction");
                    println!("\tx         - Display Memory at address (example: x/100 0x200)");
                    println!("\tjmp/jump  - Jump to address (example: jmp 0x200)");
                    println!("\tdisp      - Display registers/stack/dissasembly");
                    println!("\tq         - Quit");
                }

                // quit
                "q" => {break;},
                v => {println!("[-] unrecognized debugger command: {} (try \"help\")", v); continue;}
            }

            // save the last successfull command
            last_command = cmd;
        }
        Ok(())
    }

}
