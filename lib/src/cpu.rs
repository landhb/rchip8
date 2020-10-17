use crate::instructions::inst;
use anyhow::{bail, Result};
use bitvec::prelude::*;
use byteorder::{BigEndian, ByteOrder};
use std::fs::File;
use std::io::Read;

pub const MEM_SIZE: usize = 0xFFF;
pub const TXT_OFFSET: usize = 0x200;
pub const FLAG_REGISTER: usize = 15; // VF register

pub const DISP_WIDTH: usize = 64;
pub const DISP_HEIGHT: usize = 32;

pub struct Cpu {
    pub stack: Vec<u16>,
    pub memory: [u8; MEM_SIZE],

    pub registers: [u8; 16],

    // special registers/timers
    pub i_register: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,

    // pc
    pub program_counter: usize,

    // peripherals
    pub keyboard: bitvec::vec::BitVec<LocalBits, usize>,
    pub display: [u8; DISP_HEIGHT * DISP_WIDTH],

    // internal state, true if halted
    // to pause for a key press event
    halted: bool,
    store_key: usize,
}

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/**
 * Translate a modern key code to its
 * chip8 equivalent
 */
fn translate_key(code: usize) -> usize {
    match code {
        49 => 1,   // 1
        50 => 2,   // 2
        51 => 3,   // 3
        52 => 0xC, // 4
        81 => 4,   // Q
        87 => 5,   // W
        69 => 6,   // E
        82 => 0xD, // R
        64 => 7,   // A
        83 => 8,   // S
        68 => 9,   // D
        70 => 0xE, // F
        90 => 0xA, // Z
        88 => 0x0, // X
        67 => 0xB, // C
        86 => 0xF, // V
        _ => 0,
    }
}

impl Cpu {
    /**
     * Create a new CPU instance
     */
    pub fn new() -> Self {
        let mut res = Cpu {
            stack: Vec::<u16>::new(),
            memory: [0; MEM_SIZE],
            registers: [0; 16],
            i_register: 0u16,
            program_counter: TXT_OFFSET,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: bitvec![mut 0u8; 16],
            display: [0u8; DISP_HEIGHT * DISP_WIDTH],
            halted: false,
            store_key: 0,
        };

        res.memory[0..FONT_SET.len()].copy_from_slice(&FONT_SET);
        res
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
     * Load a chip8 program into memory
     */
    pub fn load_from_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > MEM_SIZE - TXT_OFFSET {
            bail!("[!] ROM too large to load.")
        }
        // the source is of unknown length, so we must get the length first
        let len = bytes.len();
        self.memory[TXT_OFFSET..TXT_OFFSET + len].copy_from_slice(bytes);
        Ok(())
    }

    /**
     * Obtain a reference to the display buffer
     */
    pub fn get_display(&self) -> &[u8] {
        &self.display
    }

    /**
     * Set the given key to the down position
     * If halted, resume execution
     */
    pub fn key_down(&mut self, key: usize) {
        if self.halted {
            self.halted = false;
            self.registers[self.store_key] = translate_key(key) as u8;
        }
        self.keyboard.set(translate_key(key), true);
    }

    /**
     * Set the given key to the up position
     */
    pub fn key_up(&mut self, key: usize) {
        self.keyboard.set(translate_key(key), false);
    }

    /**
     * Decrement timers
     */
    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /**
     * Fetch the next 16 bit opcode from memory
     */
    pub fn fetch_instruction(&self) -> u16 {
        BigEndian::read_u16(&self.memory[self.program_counter..self.program_counter + 2])
    }

    /**
     * Each cycle will call this after reading in another 16 bit opcode
     */
    pub fn execute_instruction(&mut self, opcode: u16) -> Result<()> {
        // All execution will be halted until
        // a key down event occurs
        if self.halted {
            return Ok(());
        }

        let nums = [
            (opcode >> 12) & 0xF,
            (opcode >> 8) & 0xF,
            (opcode >> 4) & 0xF,
            opcode & 0xF,
        ];

        match nums {
            [0, 0, 0xE, 0] => inst::cls(self),
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
            [0xE, x, 9, 0xE] => inst::skp_vx(self, x),
            [0xE, x, 0xA, 1] => inst::sknp_vx(self, x),
            [0xF, x, 0, 7] => inst::ld_vx_dt(self, x),
            [0xF, x, 0, 0xA] => {
                self.halted = true;
                self.store_key = x.into();
            }
            [0xF, x, 1, 5] => inst::ld_dt_vx(self, x),
            [0xF, x, 1, 8] => inst::ld_st_vx(self, x),
            [0xF, x, 1, 0xE] => inst::add_i_vx(self, x),
            [0xF, x, 2, 9] => inst::ld_f_vx(self, x),
            [0xF, x, 3, 3] => inst::ld_b_vx(self, x),
            [0xF, x, 5, 5] => inst::ld_i_vx(self, x),
            [0xF, x, 6, 5] => inst::ld_vx_i(self, x),
            [_, _, _, _] => {
                bail!("[-] opcode 0x{:x} not implemented", opcode);
            }
        }
        // move to next opcode
        self.program_counter += 2;
        Ok(())
    }
}
