use crate::cpu;

#[cfg(test)]
mod test_instructions {
    use super::*;

    #[test]
    fn test_jmp_nnn() {
        let mut cpu = cpu::Cpu::new();
        assert!(cpu.program_counter == 0x200);
        cpu.execute_instruction(0x124e);       // JMP 0x24e
        assert!(cpu.program_counter == 0x24e);
    }

    #[test]
    fn test_ld_vx() {
        let mut cpu = cpu::Cpu::new();
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x6050);       // LD V0, 0x50
        assert!(cpu.general_registers[0] == 0x50);

        // check that registers have been initialized properly
        for i in 1..0xf {
            assert!(cpu.general_registers[i] == 0);
            cpu.execute_instruction(0x6050+((i as u16)*0x100)+(i as u16));       // LD VX, 0x5X
            assert!(cpu.general_registers[i] == 0x50 + (i as u8));
        }
       
    }
}