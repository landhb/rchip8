use crate::instructions::inst;
use anyhow::{bail, Result};
use std::fs::File;
use std::io::Read;
use byteorder::{BigEndian, ByteOrder};
use bitvec::prelude::*;

pub const MEM_SIZE: usize = 0xFFF;
pub const TXT_OFFSET: usize = 0x200;
pub const FLAG_REGISTER: usize = 15; // VF register

pub const DISP_WIDTH: usize = 64;
pub const DISP_HEIGHT: usize = 32;

pub struct Cpu {
    pub(crate) stack: Vec<u16>,
    pub(crate) memory: [u8; MEM_SIZE],

    pub(crate) registers: [u8; 16],

    // special registers
    pub(crate) i_register: u16,
    pub(crate) dt_register: u8,
    pub(crate) st_register: u8,

    // pc
    pub(crate) program_counter: usize,

    // timers
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,

    // peripherals
    pub(crate) keyboard: bitvec::vec::BitVec<LocalBits,usize>,
    pub(crate) display: [u8;DISP_HEIGHT*DISP_WIDTH],
}


impl Cpu {

    /**
     * Create a new CPU instance
     */
    pub fn new() -> Self {
        Cpu {
            stack: Vec::<u16>::new(),
            memory: [0; MEM_SIZE],
            registers: [0; 16],
            i_register: 0u16,
            dt_register: 0u8,
            st_register: 0u8,
            program_counter: TXT_OFFSET,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: bitvec![mut 0u8; 256],
            display: [0u8;DISP_HEIGHT*DISP_WIDTH],
        }
    }

    /**
     * Load a chip8 program into memory
     */
    pub fn load_program(&mut self, path: &str) -> Result<usize> {
        // sanity check the file size
        let metadata = std::fs::metadata(path)?;
        if !metadata.is_file() && (metadata.len() > (MEM_SIZE - TXT_OFFSET) as u64) {
            bail!("[!] ROM too large to load.")
        }

        // load into memory at TXT_OFFSET
        let mut f = File::open(path)?;
        let len = f.read(&mut self.memory[TXT_OFFSET..MEM_SIZE])?;
        println!("[+] read {} bytes", len);
        Ok(len)
    }

    /**
     * Obtain a reference to the display buffer
     */
    pub fn get_display(&self) -> &[u8] {
        &self.display
    }

    /**
     * Load a chip8 program into memory
     */
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > MEM_SIZE-TXT_OFFSET {
            bail!("[!] ROM too large to load.")
        }
        // the source is of unknown length, so we must get the length first
        let len = bytes.len();
        self.memory[TXT_OFFSET..TXT_OFFSET+len].copy_from_slice(bytes);
        Ok(())
    }

    /**
     * Fetch the next 16 bit opcode from memory
     */
    pub fn fetch_instruction(&self) -> u16 {
        BigEndian::read_u16(
            &self.memory[self.program_counter..self.program_counter + 2],
        )
    }

    /**
     * Each cycle will call this after reading in another 16 bit opcode
     */
    pub fn execute_instruction(&mut self, opcode: u16) -> Result<()> {
        
        let nums = [
            (opcode >> 12) & 0xF,
            (opcode >> 8) & 0xF,
            (opcode >> 4) & 0xF,
            opcode & 0xF,
        ];

        match nums {
            [0,0,0xE,0] => inst::cls(self),
            [0, 0, 0xE, 0xE] => inst::ret(self),
            [0, _, _, _] => inst::sys(),
            [1, _, _, _] => inst::jmp_nnn(self, opcode),
            [2, _, _, _] => inst::call(self, opcode),
            [3, x, _, _] => inst::se_vx_kk(self, x, opcode),
            [4, x, _, _] => inst::sne_vx_kk(self, x, opcode),
            [5, x, y, _] => inst::se_vx_vy(self, x, y),
            [6, x, _, _] => inst::ld_vx(self, x, opcode),
            [7, x, _, _] => inst::add_vx_kk(self, x, opcode),
            [8, x, y, 0] => inst::ld_vx_vy(self, x, y),
            [8, x, y, 1] => inst::or_vx_vy(self, x, y),
            [8, x, y, 2] => inst::and_vx_vy(self, x, y),
            [8, x, y, 3] => inst::xor_vx_vy(self, x, y),
            [8, x, y, 4] => inst::add_vx_vy(self, x, y),
            [8, x, y, 5] => inst::sub_vx_vy(self, x, y),
            [8, x, y, 6] => inst::shr_vx_vy(self, x, y),
            [8, x, y, 7] => inst::subn_vx_vy(self, x, y),
            [8, x, y, 0xE] => inst::shl_vx_vy(self, x, y),
            [9, x, y, 0] => inst::sne_vx_vy(self, x, y),
            [0xA, _, _, _] => inst::ld_i_nnn(self, opcode),
            [0xB, _, _, _] => inst::jmp_v0_nnn(self, opcode),
            [0xC, x, _, _] => inst::rnd_vx_kk(self, x, opcode),
            [0xD, x, y, n] => inst::drw_vx_vy_n(self, x, y, n),
            [0xE,x,9,0xE] => inst::skp_vx(self,x),
            [0xE,x,0xA,1] => inst::sknp_vx(self,x),
            [0xF, x, 0, 7] => inst::ld_vx_dt(self, x),
            //[0xF,x,0,0xA] => {"LD V{:X}, K", x),comments);},
            [0xF, x, 1, 5] => inst::ld_dt_vx(self, x),
            [0xF, x, 1, 8] => inst::ld_st_vx(self, x),
            //[0xF,x,1,0xE] => {"ADD I, V{:X}", x),comments);},
            /*[0xF,x,2,9] => {"LD F, V{:X}", x),comments);},
            [0xF,x,3,3] => {"LD B, V{:X}", x),comments);},
            [0xF,x,5,5] => {"LD [I], V{:X}", x),comments);},
            [0xF,x,6,5] => {"LD V{:X}, [I]", x),comments);}, */
            [_, _, _, _] => {
                bail!("[-] opcode 0x{:x} not implemented",opcode);
            }
        }
        // move to next opcode
        self.program_counter += 2;
        Ok(())
    }
}
