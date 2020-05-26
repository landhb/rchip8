use crate::instructions::instructions;


pub const MEM_SIZE: usize = 0xFFF;
pub const TXT_OFFSET: usize = 0x200;
pub const FLAG_REGISTER: usize = 15; // VF register

pub struct Cpu {


    pub stack: Vec<u8>,
    pub memory: [u8;MEM_SIZE],

    pub registers: [u8;16],

    // special registers
    pub i_register: u16,
    pub dt_register: u8,
    pub st_register: u8,

    // pc
    pub program_counter: usize,

}



impl Cpu {

    /* 
     * Create a new CPU instance, called once at startup
     */
    pub fn new() -> Self {
        Cpu{
            stack: Vec::<u8>::new(),
            memory: [0;MEM_SIZE],
            registers: [0;16],
            i_register: 0u16,
            dt_register: 0u8,
            st_register: 0u8,
            program_counter: TXT_OFFSET,
        }
    }

    /* 
     * Each cycle will call this after reading in another 16 bit opcode
     */
    pub fn execute_instruction(&mut self,opcode: u16) {
        let nums = [(opcode >> 12) & 0xF, (opcode >> 8) & 0xF, (opcode >> 4) & 0xF, opcode & 0xF];
        //println!("{:?}", nums);
        match nums {
            /*[0,0,0xE,0] => {self.print_instruction(i,"CLS","clear screen");},
            [0,0,0xE,0xE] => {self.print_instruction(i,"RET",comments);},
            [0,n,_,_] => {self.print_instruction(i,&format!("SYS 0x{:X}{:X}", n,v[1]),comments);},*/
            [1,_,_,_] => {instructions::jmp_nnn(self,opcode);},
            /*[2,n,_,_] => {self.print_instruction(i,&format!("CALL 0x{:X}{:X}", n,v[1]),comments);},
            [3,x,_,_] => {self.print_instruction(i,&format!("SE V{:X}, {:X}", x,v[1]),comments);}, // skip instruction if Vx == kk
            [4,x,_,_] => {self.print_instruction(i,&format!("SNE V{:X}, {:X}", x,v[1]),comments);}, // skip instruction if Vx != kk
            [5,x,y,_] => {self.print_instruction(i,&format!("SNE V{:X}, V{:X}", x,y),comments);}, // skip instruction if Vx == Vy */
            [6,x,_,_] => {instructions::ld_vx(self,x,opcode);}, // load kk into Vx 
            [7,x,_,_] => {instructions::add_vx_kk(self,x,opcode);}, // add kk to Vx
            [8,x,y,0] => {instructions::ld_vx_vy(self,x,y);}, // set Vx = Vy
            [8,x,y,1] => {instructions::or_vx_vy(self,x,y);}, // Vx = Vx OR Vy
            [8,x,y,2] => {instructions::and_vx_vy(self,x,y);}, // Vx = Vx AND Vy
            [8,x,y,3] => {instructions::xor_vx_vy(self,x,y);}, // Vx = Vx XOR Vy
            [8,x,y,4] => {instructions::add_vx_vy(self,x,y);}, // Vx = Vx + Vy, VF = carry
            [8,x,y,5] => {instructions::sub_vx_vy(self,x,y);}, // Vx = Vx - Vy, set VF = NOT borrow.
            [8,x,y,6] => {instructions::shr_vx_vy(self,x,y);}, // shift right
            [8,x,y,7] => {instructions::subn_vx_vy(self,x,y);}, 
            [8,x,y,0xE] => {instructions::shl_vx_vy(self,x,y);}, //shift left
            [9,x,y,0] => {instructions::sne_vx_vy(self,x,y);}, // same as before?
            [0xA,_,_,_] => {instructions::ld_i_nnn(self,opcode)},
            /*[0xB,n,_,_] => {self.print_instruction(i,&format!("JP V0, 0x{:X}{:X}", n,v[1]),comments);},
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
            [0xF,x,6,5] => {self.print_instruction(i,&format!("LD V{:X}, [I]", x),comments);}, */
            [_,_,_,_] => {println!("{:X}", opcode); unimplemented!();}, 
        } 
        // move to next opcode
        self.program_counter += 2;
    }

    fn print_instruction(&self, line: usize, instr: &str, comments: &str) {
        use colored::*;

        if comments.len() > 0 {
            println!("0x{:x} <+{:?}>:\t{}\t\t{}", (self.program_counter-2)+(line*2), line*2, instr.blue().bold(), comments.blue().bold());
        }
        else {
            println!("0x{:x} <+{:?}>:\t{}\t", (self.program_counter-2)+(line*2), line*2, instr);
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
                [3,x,_,_] => {self.print_instruction(i,&format!("SE V{:X}, {:X}", x,v[1]),comments);}, // skip instruction if Vx == kk
                [4,x,_,_] => {self.print_instruction(i,&format!("SNE V{:X}, {:X}", x,v[1]),comments);}, // skip instruction if Vx != kk
                [5,x,y,_] => {self.print_instruction(i,&format!("SNE V{:X}, V{:X}", x,y),comments);}, // skip instruction if Vx == Vy
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



    pub fn display_debug_info(& self) {
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
                println!("{:?}: {}", offset, value);
            }
        } else {
            println!("{}", "<Empty>".red());
        }
    }
}



