pub const MEM_SIZE: usize = 0xFFF;
pub const TXT_OFFSET: usize = 0x200;

pub struct Cpu {


    pub stack: Vec<u8>,
    pub memory: [u8;MEM_SIZE],

    // registers
    pub general_registers: [u8;16],
    pub i_register: u16,
    pub vf_register: u8,


    pub program_counter: usize,

}

impl Cpu {

    pub fn new() -> Self {
        Cpu{
            stack: Vec::<u8>::new(),
            memory: [0;MEM_SIZE],
            general_registers: [0;16],
            i_register: 0u16,
            vf_register: 0u8,
            program_counter: TXT_OFFSET,
        }
    }

    pub fn execute_instruction(&mut self,opcode: u16) {
        let nums = [(opcode >> 12) & 0xF, (opcode >> 8) & 0xF, (opcode >> 4) & 0xF, opcode & 0xF];
        match nums {
            /*[0,0,0xE,0] => {print_instruction(i,"CLS","clear screen");},
            [0,0,0xE,0xE] => {print_instruction(i,"RET","");},
            [0,n,_,_] => {print_instruction(i,&format!("SYS 0x{:X}{:X}", n,v[1]),"");},*/
            [1,_,_,_] => {instructions::jmp_nnn(self,opcode);},
            /*[2,n,_,_] => {print_instruction(i,&format!("CALL 0x{:X}{:X}", n,v[1]),"");},
            [3,x,_,_] => {print_instruction(i,&format!("SE V{}, {:X}", x,v[1]),"");}, // skip instruction if Vx == kk
            [4,x,_,_] => {print_instruction(i,&format!("SNE V{}, {:X}", x,v[1]),"");}, // skip instruction if Vx != kk
            [5,x,y,_] => {print_instruction(i,&format!("SNE V{}, V{}", x,y),"");}, // skip instruction if Vx == Vy */
            [6,x,_,_] => {instructions::ld_vx(self,x,opcode);}, // load kk into Vx 
            [7,x,_,_] => {instructions::add_vx_kk(self,x,opcode);}, // add kk to Vx
            [8,x,y,0] => {instructions::ld_vx_vy(self,x,y);}, // set Vx = Vy
            /*[8,x,y,1] => {print_instruction(i,&format!("OR V{}, V{}", x,y),"");}, // Vx = Vx OR Vy
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
            [0xF,x,6,5] => {print_instruction(i,&format!("LD V{}, [I]", x),"");}, */
            [_,_,_,_] => {println!("{:X}", opcode); unimplemented!();}, 
        } 
    }
}



pub mod instructions {

    use crate::cpu::Cpu;
    //use byteorder::{ByteOrder, BigEndian}; 

    pub fn jmp_nnn(cpu: &mut Cpu, opcode:u16) {
        let addr = (opcode & 0x0FFF) as usize;
        cpu.program_counter = addr;
    }

    pub fn ld_vx(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = (opcode & 0x00FF) as u8;
        cpu.general_registers[reg as usize] = value;
    }

    // Vx = Vx + kk
    pub fn add_vx_kk(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = opcode & 0x00FF;

        // calculate sum in a u16 for overflow safety
        let tmp = (cpu.general_registers[reg as usize] as u16) + value;
 
        // only place the last 8 bits into the register
        cpu.general_registers[reg as usize] = (tmp & 0xFF) as u8;
    }

    pub fn ld_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        cpu.general_registers[regx as usize] =
        cpu.general_registers[regy as usize];
    }
}

    
