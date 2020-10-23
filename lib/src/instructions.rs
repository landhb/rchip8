pub(crate) mod inst {

    use crate::cpu::Cpu;
    use crate::cpu::{DISP_HEIGHT, DISP_WIDTH, FLAG_REGISTER, FONT_SET};

    /**  
     *  0nnn - SYS addr
     *  Jump to a machine code routine at nnn.  This instruction
     *  is only used on the old computers on which Chip-8 was
     *  originally implemented.  It is ignored bymodern interpreters.  
     *  This will not be implemented
     */
    pub fn sys() {}

    /**
     *  00E0 - CLS
     *  Clear the display.
     */
    pub fn cls(cpu: &mut Cpu) {
        if let Some((last, elems)) = cpu.display.split_last_mut() {
            for el in elems {
                *el = 0;
            }
            *last = 0
        }
    }

    /**  
     *  00EE - RET
     *  Return from a subroutine.The interpreter sets the program
     *  counter to theaddress at the top of the stack, then
     *  subtracts 1 from the stack pointer.
     */
    pub fn ret(cpu: &mut Cpu) {
        cpu.program_counter = match cpu.stack.pop() {
            Some(addr) => (addr - 2) as usize,
            None => {
                panic!("[*] segfault! No address on the stack to jump to.");
            }
        }
    }

    /**  
     *  1nnn - JP addr
     *  Jump to address nnn
     */
    pub fn jmp_nnn(cpu: &mut Cpu, opcode: u16) {
        let addr = (opcode & 0x0FFF) as usize;
        cpu.program_counter = addr - 2;
    }

    /**  
     *  2nnn - CALL addr
     *  Call function at address
     *  before jumping, save the next instruction address
     *  on the stack
     */
    pub fn call(cpu: &mut Cpu, opcode: u16) {
        let addr = (opcode & 0x0FFF) as usize;
        cpu.stack.push((cpu.program_counter + 2) as u16);
        cpu.program_counter = addr - 2;
    }

    /**  
     *  3xkk - SE Vx, byte
     *  Skip next instruction if Vx = kk.  The interpreter
     *  compares register Vx tokk, and if they are equal,
     *  increments the program counter by 2.
     */
    pub fn se_vx_kk(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = (opcode & 0x00FF) as u8;
        if cpu.registers[reg as usize] == value {
            cpu.program_counter += 2;
        }
    }

    /**  
     *  4xkk - SNE Vx, byte
     *  Skip next instruction if Vx != kk.  The interpreter
     *  compares register Vx tokk, and if they are not equal,
     *  increments the program counter by 2.
     */
    pub fn sne_vx_kk(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = (opcode & 0x00FF) as u8;
        if cpu.registers[reg as usize] != value {
            cpu.program_counter += 2;
        }
    }

    /**  
     *  5xy0 - SE Vx, Vy
     *  Skip next instruction if Vx = Vy.  The interpreter compares
     *  register Vx toregister Vy, and if they are equal, increments
     *  the program counter by 2
     */
    pub fn se_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        if cpu.registers[regx as usize] == cpu.registers[regy as usize] {
            cpu.program_counter += 2;
        }
    }

    /**  
     *  6xkk - LD Vx, byte
     *  Loads the value kk into register Vx.
     */
    pub fn ld_vx(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = (opcode & 0x00FF) as u8;
        cpu.registers[reg as usize] = value;
    }

    /**  
     *  7xkk - ADD Vx, byte
     *  Adds the value kk to the value of register Vx,
     *  then stores the result in Vx.
     */
    pub fn add_vx_kk(cpu: &mut Cpu, reg: u16, opcode: u16) {
        let value = opcode & 0x00FF;
        cpu.registers[reg as usize] = cpu.registers[reg as usize].wrapping_add(value as u8);
    }

    /**  
     *  8xy0 - LD Vx, Vy
     *  Stores the value of register Vy in register Vx.
     */
    pub fn ld_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        cpu.registers[regx as usize] = cpu.registers[regy as usize];
    }

    /**  
     *  8xy1 - OR Vx, Vy
     *  Performs a bitwise OR on the values of Vx and Vy,
     *  then stores the result in Vx. A bitwise OR compares
     *  the corrseponding bits from two values, and if either
     *  bit is 1, then the same bit in the result is also 1.
     *  Otherwise, it is 0.
     */
    pub fn or_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        cpu.registers[regx as usize] |= cpu.registers[regy as usize];
    }

    /**  
     *  8xy2 - AND Vx, Vy
     *  Performs a bitwise AND on the values of Vx and Vy,
     *  then stores the result in Vx. A bitwise AND compares
     *  the corrseponding bits from two values, and if both
     *  bits are 1, then the same bit in the result is also 1.
     *  Otherwise, it is 0.
     */
    pub fn and_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        cpu.registers[regx as usize] &= cpu.registers[regy as usize];
    }

    /**  
     *  8xy3 - XOR Vx, Vy
     *  Performs a bitwise exclusive OR on the values of Vx and Vy,
     *  then stores the result in Vx. An exclusive OR compares the
     *  corrseponding bits from two values, and if the bits are not
     *  both the same, then the corresponding bit in the result is
     *  set to 1. Otherwise, it is 0.
     */
    pub fn xor_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        cpu.registers[regx as usize] ^= cpu.registers[regy as usize];
    }

    /**  
     *  8xy4 - ADD Vx, Vy
     *  The values of Vx and Vy are added together. If the result is
     *  greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
     *  Only the lowest 8 bits of the result are kept, and stored in Vx.
     */
    pub fn add_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        match cpu.registers[regx as usize].overflowing_add(cpu.registers[regy as usize] as u8) {
            (v, true) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 1u8;
            }
            (v, false) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 0u8;
            }
        }
    }

    /**  
     *  8xy5 - SUB Vx, Vy
     *  Subtract the value of register VY from register VX
     *  If Vx > Vy, then VF is set to 1, otherwise 0.
     */
    pub fn sub_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        match cpu.registers[regx as usize].overflowing_sub(cpu.registers[regy as usize] as u8) {
            (v, true) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 0u8; // 0 if underflow occured
            }
            (v, false) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 1u8; // 1 if underflow didn't occur
            }
        }
    }

    /**  
     *  8xy6 - SHR Vx{, Vy}
     *  Store the value of register VY shifted right one bit in register VX
     *  Set register VF to the least significant bit prior to the shift
     */
    pub fn shr_vx_vy(cpu: &mut Cpu, regx: u16, _regy: u16) {
        cpu.registers[FLAG_REGISTER] = cpu.registers[regx as usize] & 1; 
        cpu.registers[regx as usize] = cpu.registers[regx as usize].wrapping_shr(1);
    }

    /**  
     *  8xy7 - SUBN Vx, Vy
     *  Set register VX to the value of VY minus VX
     *  Set VF to 00 if a borrow occurs
     *  Set VF to 01 if a borrow does not occur
     */
    pub fn subn_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        match cpu.registers[regy as usize].overflowing_sub(cpu.registers[regx as usize] as u8) {
            (v, true) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 0u8; // 0 if underflow occured
            }
            (v, false) => {
                cpu.registers[regx as usize] = v;
                cpu.registers[FLAG_REGISTER] = 1u8; // 1 if underflow didn't occur
            }
        }
    }

    /**  
     *  8xyE - SHL Vx{, Vy}
     *  Store the value of register VY shifted left one bit in register VX
     *  Set register VF to the most significant bit prior to the shift
     */
    pub fn shl_vx_vy(cpu: &mut Cpu, regx: u16, _regy: u16) {
        cpu.registers[FLAG_REGISTER] = cpu.registers[regx as usize] >> 7; 
        cpu.registers[regx as usize] = cpu.registers[regx as usize].wrapping_shl(1);
    }

    /**
     *  9xy0 - SNE Vx, Vy
     *  The values of Vx and Vy are compared, and if they are not
     *  equal, the program counter is increased by 2.
     */
    pub fn sne_vx_vy(cpu: &mut Cpu, regx: u16, regy: u16) {
        if cpu.registers[regx as usize] != cpu.registers[regy as usize] {
            cpu.program_counter += 2;
        }
    }

    /**  
     *  Annn - LD I, addr
     *  Set I = nnn.  The value of register I is set to nnn.
     */
    pub fn ld_i_nnn(cpu: &mut Cpu, opcode: u16) {
        let addr = (opcode & 0x0FFF) as u16;
        cpu.i_register = addr;
    }

    /**
     *  Bnnn - JP V0, addr
     *  Jump  to  location  nnn  +  V0.   The  program  counter
     *  is  set  to  nnn  plus  thevalue of V0.
     */
    pub fn jmp_v0_nnn(cpu: &mut Cpu, opcode: u16) {
        let addr = (opcode & 0x0FFF) as u16;
        cpu.program_counter = ((cpu.registers[0] as u16) + addr - 2) as usize;
    }

    /**
     *  Cxkk - RND Vx, byte
     *  Set Vx = random byte AND kk.  The interpreter generates a random
     *  numberfrom 0 to 255, which is then ANDed with the value kk.  
     *  The results are storedin Vx.  
     */
    pub fn rnd_vx_kk(cpu: &mut Cpu, reg: u16, opcode: u16) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let value = (opcode & 0x00FF) as u8;
        cpu.registers[reg as usize] = rng.gen::<u8>() & value;
    }

    /**
     * Dxyn - DRW Vx, Vy, nibble
     * Display n-byte sprite starting at memory location I at
     * (Vx, Vy), set VF = collision.
     */
    pub fn drw_vx_vy_n(cpu: &mut Cpu, regx: u16, regy: u16, n: u16) {
        let x = ((cpu.registers[regx as usize] as usize) % DISP_WIDTH) as u16;
        let y = ((cpu.registers[regy as usize] as usize) % DISP_HEIGHT) as u16;
        let start_pos = (x + (y * (DISP_WIDTH as u16))) as usize;
        cpu.registers[FLAG_REGISTER] = 0;
        for row in 0..(n as usize) {
            for col in 0..8 {
                let disp_pos = start_pos + col + (row * DISP_WIDTH);
                let mem_pos = (cpu.i_register as usize) + row;

                // check if boundary has been reached
                if disp_pos >= cpu.display.len() {
                    continue;
                }

                // each byte in memory contains 8 pixels for our display
                // so we must get the individual bit value for this row,col
                let mem_val = cpu.memory[mem_pos] >> (7 - col) & 0x01;

                if cpu.display[disp_pos] == 1 && mem_val == 1 {
                    cpu.registers[FLAG_REGISTER] = 1;
                    cpu.phosphor_glow[disp_pos] = 2; // ticks until clear
                }
                cpu.display[disp_pos] ^= mem_val;
            }
        }
    }

    /**
     * Ex9E - SKP Vx
     * Skip next instruction if key with the value of Vx is pressed.
     */
    pub(crate) fn skp_vx(cpu: &mut Cpu, reg: u16) {
        let key = cpu.registers[reg as usize] as usize;
        if cpu.keyboard.get(key) == Some(&true) {
            cpu.program_counter += 2;
        }
    }

    /**
     * ExA1 - SKNP Vx
     * Skip next instruction if key with the value of Vx is not pressed.
     */
    pub(crate) fn sknp_vx(cpu: &mut Cpu, reg: u16) {
        let key = cpu.registers[reg as usize] as usize;
        if cpu.keyboard.get(key) == Some(&false) {
            cpu.program_counter += 2;
        }
    }

    /**
     * Fx07 - LD Vx, DT
     *
     * Set Vx = delay timer value.
     */
    pub(crate) fn ld_vx_dt(cpu: &mut Cpu, reg: u16) {
        cpu.registers[reg as usize] = cpu.delay_timer;
    }

    /**
     * Fx15 - LD DT, Vx
     *
     * Set delay timer = Vx.
     */
    pub(crate) fn ld_dt_vx(cpu: &mut Cpu, reg: u16) {
        cpu.delay_timer = cpu.registers[reg as usize];
    }

    /**
     * Fx18 - LD ST, Vx
     *
     * Set sound timer = Vx.
     */
    pub(crate) fn ld_st_vx(cpu: &mut Cpu, reg: u16) {
        cpu.sound_timer = cpu.registers[reg as usize];
    }

    /**
     * Fx1E - ADD I, Vx
     * Set I = I + Vx.
     */
    pub(crate) fn add_i_vx(cpu: &mut Cpu, reg: u16) {
        cpu.i_register = cpu
            .i_register
            .wrapping_add(cpu.registers[reg as usize].into());
    }

    /**
     * Fx29 - LD F, Vx
     * Set I = location of sprite for digit Vx.
     * The value of I is set to the location for the hexadecimal sprite
     * corresponding to the value of Vx in the font set
     */
    pub(crate) fn ld_f_vx(cpu: &mut Cpu, reg: u16) {
        let addr = cpu.registers[reg as usize] * 5;
        if addr as usize > FONT_SET.len() {
            panic!("No fontset for {:?}", addr);
        }
        cpu.i_register = addr.into();
    }

    /**
     * Fx33 - LD B, Vx
     * Store BCD representation of Vx in memory locations I, I+1, and I+2.
     *
     * I   = the hundreds digit of Vx
     * I+1 = the tens digit of Vx
     * I+2 = the ones digit of Vx
     */
    pub(crate) fn ld_b_vx(cpu: &mut Cpu, reg: u16) {
        let addr = cpu.i_register as usize;
        cpu.memory[addr] = (cpu.registers[reg as usize] / 100) % 10;
        cpu.memory[addr + 1] = (cpu.registers[reg as usize] / 10) % 10;
        cpu.memory[addr + 2] = cpu.registers[reg as usize] % 10;
    }

    /**
     * Fx55 - LD [I], Vx
     * Store registers V0 through Vx in memory starting at location I.
     */
    pub(crate) fn ld_i_vx(cpu: &mut Cpu, reg: u16) {
        let addr = cpu.i_register as usize;
        let n = reg as usize;
        cpu.memory[addr..addr + n+1].copy_from_slice(&cpu.registers[0..n+1]);
    }

    /*
     * Fx65 - LD Vx, [I]
     * Read registers V0 through Vx from memory starting at location I.
     */
    pub(crate) fn ld_vx_i(cpu: &mut Cpu, reg: u16) {
        let addr = cpu.i_register as usize;
        let n = reg as usize;
        cpu.registers[0..n + 1].copy_from_slice(&cpu.memory[addr..addr + n + 1]);
    }
}
