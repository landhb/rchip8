use crate::instructions::instructions;
use anyhow::{Result,bail};
use std::fs::File;
use std::io::Read;

pub const MEM_SIZE: usize = 0xFFF;
pub const TXT_OFFSET: usize = 0x200;
pub const FLAG_REGISTER: usize = 15; // VF register

pub struct Cpu {


    pub stack: Vec<u16>,
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
            stack: Vec::<u16>::new(),
            memory: [0;MEM_SIZE],
            registers: [0;16],
            i_register: 0u16,
            dt_register: 0u8,
            st_register: 0u8,
            program_counter: TXT_OFFSET,
        }
    }

    pub fn load_program(&mut self, path: &str) -> Result<usize>{

        // sanity check the file size
        let metadata = std::fs::metadata(path)?;
        if !metadata.is_file() && (metadata.len() > (MEM_SIZE-TXT_OFFSET) as u64) {
            bail!("[!] ROM too large to load.")
        }

        // load into memory at TXT_OFFSET
        let mut f = File::open(path)?;
        let len = f.read(&mut self.memory[TXT_OFFSET..MEM_SIZE])?;
        println!("[+] read {} bytes", len);
        Ok(len)
    }

    /* 
     * Each cycle will call this after reading in another 16 bit opcode
     */
    pub fn execute_instruction(&mut self,opcode: u16) {
        let nums = [(opcode >> 12) & 0xF, (opcode >> 8) & 0xF, (opcode >> 4) & 0xF, opcode & 0xF];
        //println!("{:?}", nums);
        match nums {
            //[0,0,0xE,0] => {self.print_instruction(i,"CLS","clear screen");},
            [0,0,0xE,0xE] => {instructions::ret(self);},
            [0,_,_,_] => {instructions::sys(self);},
            [1,_,_,_] => {instructions::jmp_nnn(self,opcode);},
            [2,_,_,_] => {instructions::call(self,opcode);},
            [3,x,_,_] => {instructions::se_vx_kk(self,x,opcode);}, // skip instruction if Vx == kk
            [4,x,_,_] => {instructions::sne_vx_kk(self,x,opcode);}, // skip instruction if Vx != kk
            [5,x,y,_] => {instructions::se_vx_vy(self,x,y);}, // skip instruction if Vx == Vy */
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
            [0xB,_,_,_] => {instructions::jmp_v0_nnn(self,opcode);},
            [0xC,x,_,_] => {instructions::rnd_vx_kk(self,x,opcode);},
            /*[0xD,x,y,n] => {self.print_instruction(i,&format!("DRW V{:X}, V{:X}, {:X}", x,y,n),comments);},
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


}



