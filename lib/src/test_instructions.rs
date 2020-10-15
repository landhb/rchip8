use crate::cpu;
use crate::cpu::FLAG_REGISTER;

#[cfg(test)]
mod test_instructions {
    use super::*;

    #[test]
    fn test_call_ret() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.program_counter, 0x200);
        cpu.execute_instruction(0x2412).unwrap(); // CALL 0x412
        assert_eq!(cpu.program_counter, 0x412);
        cpu.execute_instruction(0x00EE).unwrap(); // RET
        assert_eq!(cpu.program_counter, 0x202);
    }

    #[test]
    #[should_panic]
    fn test_ret_fails() {
        let mut cpu = cpu::Cpu::new();
        cpu.execute_instruction(0x00EE).unwrap();
    }

    #[test]
    fn test_jmp_nnn() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.program_counter, 0x200);
        cpu.execute_instruction(0x124e).unwrap(); // JMP 0x24e
        assert_eq!(cpu.program_counter, 0x24e);
    }

    #[test]
    fn test_se_vx_kk() {
        let mut cpu = cpu::Cpu::new();
        cpu.registers[0] = 0x50;
        assert_eq!(cpu.program_counter, 0x200);

        // test skipped
        cpu.execute_instruction(0x3050).unwrap();
        assert_eq!(cpu.program_counter, 0x204);

        // test not skipped
        cpu.execute_instruction(0x3051).unwrap();
        assert_eq!(cpu.program_counter, 0x206);
    }

    #[test]
    fn test_sne_vx_kk() {
        let mut cpu = cpu::Cpu::new();
        cpu.registers[0] = 0x50;
        assert_eq!(cpu.program_counter, 0x200);

        // test not skipped
        cpu.execute_instruction(0x4050).unwrap();
        assert_eq!(cpu.program_counter, 0x202);

        // test skipped
        cpu.execute_instruction(0x4051).unwrap();
        assert_eq!(cpu.program_counter, 0x206);
    }

    #[test]
    fn test_se_vx_vy() {
        let mut cpu = cpu::Cpu::new();
        cpu.registers[0] = 0x50;
        cpu.registers[1] = 0x50;

        // test skipped
        assert_eq!(cpu.program_counter, 0x200);
        cpu.execute_instruction(0x5010).unwrap();
        assert_eq!(cpu.program_counter, 0x204);

        // test not skipped
        cpu.registers[1] = 0x51;
        cpu.execute_instruction(0x5010).unwrap();
        assert_eq!(cpu.program_counter, 0x206);
    }

    #[test]
    fn test_ld_vx() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.registers[0], 0);
        cpu.execute_instruction(0x6050).unwrap(); // LD V0, 0x50
        assert_eq!(cpu.registers[0], 0x50);

        for i in 1..0xf {
            assert_eq!(cpu.registers[i], 0);
            cpu.execute_instruction(0x6050 + ((i as u16) * 0x100)
             + (i as u16)).unwrap(); // LD VX, 0x5X
            assert_eq!(cpu.registers[i], 0x50 + (i as u8));
        }
    }

    #[test]
    fn test_add_vx_kk() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.registers[0], 0);
        cpu.execute_instruction(0x7050).unwrap(); // ADD V0, 0x50
        assert_eq!(cpu.registers[0], 0x50);
        cpu.execute_instruction(0x7001).unwrap();
        assert_eq!(cpu.registers[0], 0x51);

        // overflow test
        cpu.execute_instruction(0x70FF).unwrap();
        assert_eq!(cpu.registers[0], 0x50);
    }

    #[test]
    fn test_ld_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x50
        assert_eq!(cpu.registers[1], 0);
        cpu.execute_instruction(0x6150).unwrap();
        assert_eq!(cpu.registers[1], 0x50);

        // LD V0, V1
        assert_eq!(cpu.registers[0], 0);
        cpu.execute_instruction(0x8010).unwrap();
        assert_eq!(cpu.registers[0], 0x50);
    }

    #[test]
    fn test_or_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x09
        assert_eq!(cpu.registers[1], 0);
        cpu.execute_instruction(0x6109).unwrap();
        assert_eq!(cpu.registers[1], 0x09);

        // LD V0, 0x10
        assert_eq!(cpu.registers[0], 0);
        cpu.execute_instruction(0x6010).unwrap();
        assert_eq!(cpu.registers[0], 0x10);

        // OR V1, V0
        // 9 | 16 == 25
        cpu.execute_instruction(0x8101).unwrap();
        assert_eq!(cpu.registers[1], 0x19);
    }

    #[test]
    fn test_and_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x09
        assert_eq!(cpu.registers[1], 0);
        cpu.execute_instruction(0x6109).unwrap();
        assert_eq!(cpu.registers[1], 0x09);

        // LD V0, 0x10
        assert_eq!(cpu.registers[0], 0);
        cpu.execute_instruction(0x6010).unwrap();
        assert_eq!(cpu.registers[0], 0x10);

        // AND V1, V0
        // 9 & 16 == 0
        cpu.execute_instruction(0x8102).unwrap();
        assert_eq!(cpu.registers[1], 0x0);

        // LD V1, 0x0d
        // LD V0, 0x0a
        // V1 & V0 = 13 & 10 == 8
        cpu.execute_instruction(0x610d).unwrap();
        assert_eq!(cpu.registers[1], 0x0d);
        cpu.execute_instruction(0x600a).unwrap();
        assert_eq!(cpu.registers[0], 0x0a);
        cpu.execute_instruction(0x8102).unwrap();
        assert_eq!(cpu.registers[1], 0x08);
    }

    #[test]
    fn test_xor_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0x0b
        assert!(cpu.registers[1] == 0);
        cpu.execute_instruction(0x610b).unwrap();
        assert!(cpu.registers[1] == 0x0b);

        // LD V0, 0x05
        assert!(cpu.registers[0] == 0);
        cpu.execute_instruction(0x6005).unwrap();
        assert!(cpu.registers[0] == 0x05);

        // XOR V1, V0
        // V1 ^ V0 = 11 ^ 5 = 14
        cpu.execute_instruction(0x8103).unwrap();
        assert!(cpu.registers[1] == 0xe);
    }

    #[test]
    fn test_add_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        assert!(cpu.registers[1] == 0);
        cpu.execute_instruction(0x61ff).unwrap();
        assert!(cpu.registers[1] == 0xff);

        // LD V0, 0x05
        assert!(cpu.registers[0] == 0);
        cpu.execute_instruction(0x6005).unwrap();
        assert!(cpu.registers[0] == 0x05);

        // ADD V1, V0
        // V1 + V0 = 255 + 5 = 4
        // VF register should equal 0x1
        cpu.execute_instruction(0x8104).unwrap();
        assert!(cpu.registers[1] == 0x04);
        assert!(cpu.registers[FLAG_REGISTER] == 1);

        // ADD V1, V0
        // V1 + V0 = 4 + 5 = 9
        // VF register should equal 0x0
        cpu.execute_instruction(0x8104).unwrap();
        assert!(cpu.registers[1] == 0x09);
        assert!(cpu.registers[FLAG_REGISTER] == 0);
    }

    #[test]
    fn test_sub_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        assert!(cpu.registers[1] == 0);
        cpu.execute_instruction(0x61ff).unwrap();
        assert!(cpu.registers[1] == 0xff);

        // LD V0, 0x05
        assert!(cpu.registers[0] == 0);
        cpu.execute_instruction(0x6005).unwrap();
        assert!(cpu.registers[0] == 0x05);

        // SUB V1, V0
        // V1 - V0 = 255 - 5 = 250
        // VF register should equal 0x1
        cpu.execute_instruction(0x8105).unwrap();
        assert!(cpu.registers[1] == 0xfa);
        assert!(cpu.registers[FLAG_REGISTER] == 1);

        // set V0 > V1 before the next test
        cpu.execute_instruction(0x60ff).unwrap();
        assert!(cpu.registers[0] == 0xff);

        // SUB V1, V0
        // V1 - V0 = 250 - 255 = 5
        // VF register should equal 0x0
        cpu.execute_instruction(0x8105).unwrap();
        assert!(cpu.registers[1] == 0xfb);
        assert!(cpu.registers[FLAG_REGISTER] == 0);
    }

    #[test]
    fn test_shr_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        assert!(cpu.registers[1] == 0);
        cpu.execute_instruction(0x61ff).unwrap();
        assert!(cpu.registers[1] == 0xff);

        // SHR v0, v1
        // test VF == 1
        assert!(cpu.registers[0] == 0);
        cpu.execute_instruction(0x8016).unwrap();
        assert!(cpu.registers[0] == 0x7f);
        assert!(cpu.registers[FLAG_REGISTER] == 1);

        // LD V1, 2
        // test VF == 0
        cpu.execute_instruction(0x6102).unwrap();
        assert!(cpu.registers[1] == 0x02);
        cpu.execute_instruction(0x8016).unwrap();
        assert!(cpu.registers[0] == 0x1);
        assert!(cpu.registers[FLAG_REGISTER] == 0);
    }

    #[test]
    fn test_subn_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V0, 0xff
        assert!(cpu.registers[0] == 0);
        cpu.execute_instruction(0x60ff).unwrap();
        assert!(cpu.registers[0] == 0xff);

        // LD V1, 0x05
        assert!(cpu.registers[1] == 0);
        cpu.execute_instruction(0x6105).unwrap();
        assert!(cpu.registers[1] == 0x05);

        // SUBN V1, V0
        // V1 = V0 - V1 = 255 - 5 = 250
        // VF register should equal 0x1
        cpu.execute_instruction(0x8107).unwrap();
        assert!(cpu.registers[1] == 0xfa);
        assert!(cpu.registers[FLAG_REGISTER] == 1);

        // set V0 < V1 for next test
        cpu.execute_instruction(0x6005).unwrap();
        assert!(cpu.registers[0] == 0x5);

        // SUB V1, V0
        // V1 = V0 - V1 = 5 - 250 = 11
        // VF register should equal 0x0
        cpu.execute_instruction(0x8107).unwrap();
        assert!(cpu.registers[1] == 0xb);
        assert!(cpu.registers[FLAG_REGISTER] == 0);
    }

    #[test]
    fn test_shl_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        assert!(cpu.registers[1] == 0);
        cpu.execute_instruction(0x61ff).unwrap();
        assert!(cpu.registers[1] == 0xff);

        // SHR v0, v1
        // test VF == 1
        assert!(cpu.registers[0] == 0);
        cpu.execute_instruction(0x801e).unwrap();
        assert!(cpu.registers[0] == 0xfe);
        assert!(cpu.registers[FLAG_REGISTER] == 1);

        // LD V1, 2
        // test VF == 0
        cpu.execute_instruction(0x6101).unwrap();
        assert!(cpu.registers[1] == 0x01);
        cpu.execute_instruction(0x801e).unwrap();
        assert!(cpu.registers[0] == 0x2);
        assert!(cpu.registers[FLAG_REGISTER] == 0);
    }

    #[test]
    fn test_sne_vx_vy() {
        let mut cpu = cpu::Cpu::new();

        // LD V1, 0xff
        // PC @ 0x200
        assert_eq!(cpu.registers[1], 0);
        cpu.execute_instruction(0x61ff).unwrap();
        assert_eq!(cpu.registers[1], 0xff);

        // LD V2, 0xff
        // PC @ 0x202
        assert_eq!(cpu.registers[2], 0);
        cpu.execute_instruction(0x62fe).unwrap();
        assert_eq!(cpu.registers[2], 0xfe);

        // test that following inst is skipped
        assert_eq!(cpu.program_counter, 0x204);
        cpu.execute_instruction(0x9120).unwrap();
        assert_eq!(cpu.program_counter, 0x208);

        // test taht the inst is not skipped
        // when equal
        cpu.execute_instruction(0x62ff).unwrap();
        assert_eq!(cpu.registers[2], 0xff);
        assert_eq!(cpu.registers[1], 0xff);
        assert_eq!(cpu.program_counter, 0x20a);
        cpu.execute_instruction(0x9120).unwrap();
        assert_eq!(cpu.program_counter, 0x20c);
    }

    #[test]
    fn test_ld_i_nnn() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.i_register, 0);
        cpu.execute_instruction(0xA562).unwrap();
        assert_eq!(cpu.i_register, 0x562);
    }

    #[test]
    fn test_jmp_v0_nnn() {
        let mut cpu = cpu::Cpu::new();

        // set V0
        assert_eq!(cpu.registers[0], 0);
        cpu.execute_instruction(0x60fe).unwrap();
        assert_eq!(cpu.registers[0], 0xfe);

        // jump
        cpu.execute_instruction(0xB208).unwrap();
        assert_eq!(cpu.program_counter, 0x208 + 0xfe);
    }

    #[test]
    fn test_rnd_vx_kk() {
        let mut cpu = cpu::Cpu::new();

        assert_eq!(cpu.registers[0], 0);

        cpu.execute_instruction(0xC0ff).unwrap();
        assert_ne!(cpu.registers[0], 0);

        // anything & 0 = 0
        cpu.execute_instruction(0xC000).unwrap();
        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_ld_vx_dt() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.registers[0], 0);
        cpu.delay_timer = 0x20;
        cpu.execute_instruction(0xF007).unwrap();
        assert_eq!(cpu.delay_timer, cpu.registers[0]);
    }

    #[test]
    fn test_ld_dt_vx() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.delay_timer, 0);
        cpu.registers[0] = 0x20;
        cpu.execute_instruction(0xF015).unwrap();
        assert_eq!(cpu.delay_timer, cpu.registers[0]);
    }

    #[test]
    fn test_ld_st_vx() {
        let mut cpu = cpu::Cpu::new();
        assert_eq!(cpu.sound_timer, 0);
        cpu.registers[0] = 0x20;
        cpu.execute_instruction(0xF018).unwrap();
        assert_eq!(cpu.sound_timer, cpu.registers[0]);
    }
}
