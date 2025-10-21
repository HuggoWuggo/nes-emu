#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Mem};

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.status.bits() & 0b1000_0000 == 0);
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
}
