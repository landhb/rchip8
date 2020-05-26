// TODO - possibly just add this to the Cpu implementation
// and reference self as the first param for each function
pub mod instructions {

    use crate::cpu::Cpu;
    use crate::cpu::FLAG_REGISTER;


    /*
     *  Jump to address nnn
     */
    pub fn jmp_nnn(cpu: &mut Cpu, opcode:u16) {
        let addr = (opcode & 0x0FFF) as usize;
        cpu.program_counter = addr;
    }

    /*
     *  Loads the value kk into register Vx.
     */ 
    pub fn ld_vx(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = (opcode & 0x00FF) as u8;
        cpu.registers[reg as usize] = value;
    }

    /*
     *  Adds the value kk to the value of register Vx, 
     *  then stores the result in Vx. 
     */
    pub fn add_vx_kk(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = opcode & 0x00FF;
        cpu.registers[reg as usize] = 
        cpu.registers[reg as usize].wrapping_add(value as u8);
    }

    /*
     *  Stores the value of register Vy in register Vx.
     */
    pub fn ld_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        cpu.registers[regx as usize] =
        cpu.registers[regy as usize];
    }

    
    /*  
     *  Performs a bitwise OR on the values of Vx and Vy, 
     *  then stores the result in Vx. A bitwise OR compares 
     *  the corrseponding bits from two values, and if either 
     *  bit is 1, then the same bit in the result is also 1. 
     *  Otherwise, it is 0. 
     */
    pub fn or_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        cpu.registers[regx as usize] =
        cpu.registers[regx as usize] |
        cpu.registers[regy as usize];
    }

    
    /*  
     *  Performs a bitwise AND on the values of Vx and Vy, 
     *  then stores the result in Vx. A bitwise AND compares 
     *  the corrseponding bits from two values, and if both 
     *  bits are 1, then the same bit in the result is also 1. 
     *  Otherwise, it is 0. 
     */
    pub fn and_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        cpu.registers[regx as usize] =
        cpu.registers[regx as usize] &
        cpu.registers[regy as usize];
    }

    /*  
     *  Performs a bitwise exclusive OR on the values of Vx and Vy,
     *  then stores the result in Vx. An exclusive OR compares the 
     *  corrseponding bits from two values, and if the bits are not 
     *  both the same, then the corresponding bit in the result is 
     *  set to 1. Otherwise, it is 0. 
     */
    pub fn xor_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        cpu.registers[regx as usize] =
        cpu.registers[regx as usize] ^
        cpu.registers[regy as usize];
    }

    /*  
     *  The values of Vx and Vy are added together. If the result is 
     *  greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. 
     *  Only the lowest 8 bits of the result are kept, and stored in Vx.
     */
    pub fn add_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        match cpu.registers[regx as usize]
            .overflowing_add(cpu.registers[regy as usize] as u8) {
            (v, true) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 1u8;
            }
            (v,false) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 0u8;
            }
        }
    }

    /*  
     *  Subtract the value of register VY from register VX
     *  If Vx > Vy, then VF is set to 1, otherwise 0. 
     */
    pub fn sub_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        match cpu.registers[regx as usize]
            .overflowing_sub(cpu.registers[regy as usize] as u8) {
            (v, true) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 0u8; // 0 if underflow occured
            }
            (v,false) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 1u8; // 1 if underflow didn't occur
            }
        }
    }

    /* 
     * Store the value of register VY shifted right one bit in register VX
     * Set register VF to the least significant bit prior to the shift
     */
    pub fn shr_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        if cpu.registers[regy as usize] & 0x1 == 0x1 {
            cpu.registers[FLAG_REGISTER] = 1u8;
        }
        else {
            cpu.registers[FLAG_REGISTER] = 0u8;
        }
        cpu.registers[regx as usize] =
        cpu.registers[regy as usize].wrapping_shr(1);
    }

     /* 
     *  Set register VX to the value of VY minus VX
     *  Set VF to 00 if a borrow occurs
     *  Set VF to 01 if a borrow does not occur
     */
    pub fn subn_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        match cpu.registers[regy as usize]
            .overflowing_sub(cpu.registers[regx as usize] as u8) {
            (v, true) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 0u8; // 0 if underflow occured
            }
            (v,false) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 1u8; // 1 if underflow didn't occur
            }
        }
    }

    /* 
     *  Store the value of register VY shifted left one bit in register VX
     *  Set register VF to the most significant bit prior to the shift
     */
    pub fn shl_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16){
        if cpu.registers[regy as usize] & (1 << 7) != 0 {
            cpu.registers[FLAG_REGISTER] = 1u8;
        }
        else {
            cpu.registers[FLAG_REGISTER] = 0u8;
        }
        cpu.registers[regx as usize] =
        cpu.registers[regy as usize].wrapping_shl(1);
    }

    /*  
     *  The values of Vx and Vy are compared, and if they are not 
     *  equal, the program counter is increased by 2.
     */
    pub fn sne_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) { 
        if cpu.registers[regx as usize] !=
           cpu.registers[regy as usize] {
            cpu.program_counter += 2;
        }
    }

    pub fn ld_i_nnn(cpu: &mut Cpu, opcode:u16) { 
        let addr = (opcode & 0x0FFF) as u16;
        cpu.i_register = addr;
    }

}


    