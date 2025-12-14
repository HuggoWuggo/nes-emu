#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Mem};

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status.bits() & 0b1000_0010 == 0);
    }

    #[test]
    fn test_0xa2_ldx_immediate_load_data() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa2, 0x33, 0x00]);

        assert_eq!(cpu.register_x, 0x33);
    }

    #[test]
    fn test_0xa0_ldy_immediate_load_data() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa0, 0x23, 0x00]);

        assert_eq!(cpu.register_y, 0x23);
    }

    #[test]
    fn test_0x19_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);
        assert!(cpu.status.bits() & 0b1000_0000 == 0b1000_0000);
    }

    #[test]
    fn test_0x0a_carry_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0x0a, 0x00]);
        assert!(cpu.status.bits() & 0b0000_0001 == 0b0000_0001);
    }

    #[test]
    fn test_0x24_overflow_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9,
            0b0100_0000,
            0x85,
            0x1C,
            0xa9,
            0b0000_0001,
            0x24,
            0x1C,
            0x00,
        ]);

        assert!(cpu.status.bits() & 0b0100_0000 == 0b0100_0000);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_0xe8_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1);
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_sta_0x85_memory_storage() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x1E, 0x85, 0x10, 0x00]);
        let val = cpu.mem_read(0x10);

        assert_eq!(val, 0x1E);
    }

    #[test]
    fn test_and_0x29_and_gate() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x98, 0x29, 0x76, 0x00]);
        // 01110110
        // 10011000
        // 00010000
        assert_eq!(cpu.register_a, 0b0001_0000);
    }

    #[test]
    fn test_asl_0x0a_left_shift() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x4, 0x0a, 0x00]);
        println!("{}", cpu.program_counter);

        assert_eq!(cpu.register_a, 0b1000);
    }

    #[test]
    fn test_bcc_0x90_branch_carry_clear() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0x90, 0x2, 0x00, 0x00, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_bcs_0xb0_branch_carry_set() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x80, 0x0a, 0xB0, 0x2, 0x00, 0x00, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_beq_0xf0_branch_zero_set() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x00, 0xf0, 0x2, 0x00, 0x00, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_bit_0x24_test_bits_set() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9, 0x3c, 0x85, 0x1C, 0x24, 0x1C, 0xf0, 0x02, 0x00, 0x00, 0xe8, 0x00,
        ]);

        assert_eq!(cpu.register_x, 0);

        let val = cpu.mem_read(0x1C);
        assert_eq!(val, 0x3C);
    }

    #[test]
    fn test_bmi_0x30_branch_negativ_set() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0b1000_0000, 0x30, 0x2, 0x00, 0x00, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_bne_0xd0_branch_zero_clear() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x1, 0xf0, 0x2, 0xe8, 0x00, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_bpl_0x10_branch_negativ_clear() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0b0000_0011, 0x10, 0x2, 0x00, 0x00, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_bvc_0x50_branch_overflow_clear() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9,
            0b0010_0000,
            0x0a,
            0x0a,
            0x50,
            0x2,
            0x00,
            0x00,
            0xe8,
            0x00,
        ]);

        assert_eq!(cpu.register_x, 1);

        println!("{:?}", cpu.register_a);
    }

    #[test]
    fn test_bvs_0x70_branch_overflow_set() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9,
            0b0100_0000,
            0x85,
            0x1C,
            0xa9,
            0b0000_0001,
            0x24,
            0x1C,
            0x70,
            0x2,
            0x00,
            0x00,
            0xe8,
            0x00,
        ]);

        assert!(cpu.status.bits() & 0b0100_0000 == 0b0100_0000);
        assert_eq!(cpu.register_x, 1);
    }

    #[test]
    fn test_clc_0x18_clear_carry_flag() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x80, 0x0a, 0x18, 0x00]);

        assert!(cpu.status.bits() & 0b0000_0001 == 0b0000_0000);
    }

    #[test]
    fn test_cld_0xd8_clear_decimal_mode() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xd8, 0x00]);

        assert!(cpu.status.bits() & 0b0000_1000 == 0b0000_00000);
    }

    #[should_panic]
    #[test]
    fn test_cli_0x58_clear_interupt_disable() {
        todo!();
    }

    #[test]
    fn test_clv_0xb8_clear_overflow() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9,
            0b0100_0000,
            0x85,
            0x1C,
            0xa9,
            0b0000_0001,
            0x24,
            0x1C,
            0xb8,
            0x00,
        ]);

        assert!(cpu.status.bits() & 0b0100_0000 == 0b0000_0000);
    }

    #[test]
    fn test_cmp_0xc5_compare_accumlator() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x99, 0x85, 0x1C, 0xc5, 0x1C, 0x00]);

        assert!(cpu.status.bits() & 0b0000_0001 == 0b0000_0001)
    }

    #[test]
    fn test_cpx_0xe0_compare_x_register() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa2, 0x88, 0xe0, 0x88, 0x00]);

        assert!(cpu.status.bits() & 0b0000_0001 == 0b0000_0001)
    }

    #[test]
    fn test_cpy_0xc0_compare_y_register() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa0, 0x67, 0xc0, 0x67, 0x00]);

        assert!(cpu.status.bits() & 0b0000_0001 == 0b0000_0001)
    }

    #[test]
    fn test_dec_0xc6_decrement_memory() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0xA, 0x85, 0x1C, 0xc6, 0x1C, 0x00]);

        assert_eq!(cpu.mem_read(0x1C), 0x09);
    }

    #[test]
    fn test_dex_0xca_decrement_x_register() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa2, 0x07, 0xCA, 0x00]);

        assert_eq!(cpu.register_x, 0x06);
    }

    #[test]
    fn test_dey_0x88_decrement_y_register() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa0, 0x0B, 0x88, 0x00]);

        assert_eq!(cpu.register_y, 0x0A);
    }

    #[test]
    fn test_eor_0x49_0x45_eor_immediate() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x49, 0b1010_1010, 0x00]);

        assert_eq!(cpu.register_a, 0b0101_0101);

        cpu = CPU::new();

        cpu.load_and_run(vec![
            0xa9,
            0b1111_1111,
            0x85,
            0x1C,
            0xa9,
            0b1110_0000,
            0x45,
            0x1C,
            0x00,
        ]);

        assert_eq!(cpu.register_a, 0b0001_1111);
    }
}
