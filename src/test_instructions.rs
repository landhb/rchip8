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

        for i in 1..0xf {
            assert!(cpu.general_registers[i] == 0);
            cpu.execute_instruction(0x6050+((i as u16)*0x100)+(i as u16));       // LD VX, 0x5X
            assert!(cpu.general_registers[i] == 0x50 + (i as u8));
        }
       
    }

    #[test]
    fn test_add_vx_kk() {

        let mut cpu = cpu::Cpu::new();
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x7050);       // ADD V0, 0x50
        assert!(cpu.general_registers[0] == 0x50);
        cpu.execute_instruction(0x7001);
        assert!(cpu.general_registers[0] == 0x51);

        // overflow test
        cpu.execute_instruction(0x70FF);
        assert!(cpu.general_registers[0] == 0x50);
    }


    #[test]
    fn test_ld_vx_vy() {

        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x50
        assert!(cpu.general_registers[1] == 0);
        cpu.execute_instruction(0x6150);       
        assert!(cpu.general_registers[1] == 0x50);

        // LD V0, V1 
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x8010);
        assert!(cpu.general_registers[0] == 0x50);
    }
}