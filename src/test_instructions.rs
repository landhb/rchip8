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


    #[test]
    fn test_or_vx_vy() {

        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x09
        assert!(cpu.general_registers[1] == 0);
        cpu.execute_instruction(0x6109);       
        assert!(cpu.general_registers[1] == 0x09);

        // LD V0, 0x10
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x6010);       
        assert!(cpu.general_registers[0] == 0x10);

        // OR V1, V0
        // 9 | 16 == 25
        cpu.execute_instruction(0x8101); 
        assert!(cpu.general_registers[1] == 0x19);
    }

    #[test]
    fn test_and_vx_vy() {

        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x09
        assert!(cpu.general_registers[1] == 0);
        cpu.execute_instruction(0x6109);       
        assert!(cpu.general_registers[1] == 0x09);

        // LD V0, 0x10
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x6010);       
        assert!(cpu.general_registers[0] == 0x10);

        // AND V1, V0
        // 9 & 16 == 0
        cpu.execute_instruction(0x8102); 
        assert!(cpu.general_registers[1] == 0x0);

        // LD V1, 0x0d
        // LD V0, 0x0a
        // V1 & V0 = 13 & 10 == 8
        cpu.execute_instruction(0x610d);  
        assert!(cpu.general_registers[1] == 0x0d);
        cpu.execute_instruction(0x600a);       
        assert!(cpu.general_registers[0] == 0x0a);
        cpu.execute_instruction(0x8102); 
        assert!(cpu.general_registers[1] == 0x08);
    }

    #[test]
    fn test_xor_vx_vy() {

        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x0b
        assert!(cpu.general_registers[1] == 0);
        cpu.execute_instruction(0x610b);       
        assert!(cpu.general_registers[1] == 0x0b);

        // LD V0, 0x05
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x6005);       
        assert!(cpu.general_registers[0] == 0x05);

        // XOR V1, V0
        // V1 ^ V0 = 11 ^ 5 = 14
        cpu.execute_instruction(0x8103); 
        assert!(cpu.general_registers[1] == 0xe);
    }


    #[test]
    fn test_add_vx_vy() {

        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        assert!(cpu.general_registers[1] == 0);
        cpu.execute_instruction(0x61ff);       
        assert!(cpu.general_registers[1] == 0xff);

        // LD V0, 0x05
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x6005);       
        assert!(cpu.general_registers[0] == 0x05);

        // ADD V1, V0
        // V1 + V0 = 255 + 5 = 4
        // VF register should equal 0x1
        cpu.execute_instruction(0x8104); 
        assert!(cpu.general_registers[1] == 0x04);
        assert!(cpu.vf_register == 1);

        // ADD V1, V0
        // V1 + V0 = 4 + 5 = 9
        // VF register should equal 0x0
        cpu.execute_instruction(0x8104); 
        assert!(cpu.general_registers[1] == 0x09);
        assert!(cpu.vf_register == 0);
    }

    #[test]
    fn test_sub_vx_vy() {

        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        assert!(cpu.general_registers[1] == 0);
        cpu.execute_instruction(0x61ff);       
        assert!(cpu.general_registers[1] == 0xff);

        // LD V0, 0x05
        assert!(cpu.general_registers[0] == 0);
        cpu.execute_instruction(0x6005);       
        assert!(cpu.general_registers[0] == 0x05);

        // SUB V1, V0
        // V1 - V0 = 255 - 5 = 250
        // VF register should equal 0x1
        cpu.execute_instruction(0x8105); 
        assert!(cpu.general_registers[1] == 0xfa);
        assert!(cpu.vf_register == 1);

        // set V0 > V1 before the next test
        cpu.execute_instruction(0x60ff);       
        assert!(cpu.general_registers[0] == 0xff);

        // SUB V1, V0
        // V1 - V0 = 250 - 255 = 5
        // VF register should equal 0x0
        cpu.execute_instruction(0x8105); 
        assert!(cpu.general_registers[1] == 0xfb);
        assert!(cpu.vf_register == 0);
    }
}