use crate::cpu::Cpu;

pub static OPCODES: [(fn(&mut Cpu), &str); 0x100] = [
    //Opcodes 0X
    (nop, "NOP"),
    (ld_bc_nn, "LD BC, NN"),
    (ld_mbc_a, "LD [BC], A"),
    (inc_bc, "INC BC"),
    (inc_b, "INC B"),
    (dec_b, "DEC B"),
    (ld_b_n, "LD B, N"),
    (rlca, "RLCA"),
    (ld_mnn_sp, "LD [NN], SP"),
    (add_hl_bc, "ADD HL, BC"),
    (ld_a_mbc, "LD A, [BC]"),
    (dec_bc, "DEC BC"),
    (inc_c, "INC C"),
    (dec_c, "DEC C"),
    (ld_c_n, "LD C, N"),
    (rrca, "RRCA"),
    //Opcodes 1X
    (stop, "STOP"),
    (ld_de_nn, "LD DE, NN"),
    (ld_mde_a, "LD [DE], A"),
    (inc_de, "INC DE"),
    (inc_d, "INC D"),
    (dec_d, "DEC D"),
    (ld_d_n, "LD D, N"),
    (rla, "RLA"),
    (jr_sn, "JR SN"),
    (add_hl_de, "ADD HL, DE"),
    (ld_a_mde, "LD A, [DE]"),
    (dec_de, "DEC DE"),
    (inc_e, "INC E"),
    (dec_e, "DEC E"),
    (ld_e_n, "LD E, N"),
    (rra, "RRA"),
    (jr_nz_sn, "JR NZ, SN"),
    (ld_hl_nn, "LD HL, NN"),
    (ldi_mhl_a, "LDI [HL], A"),
    (inc_hl, "INC HL"),
    (inc_h, "INC H"),
    (dec_h, "DEC H"),
    (ld_h_n, "LD H, N"),
    (daa, "DAA"),
    (jr_z_sn, "JR Z, SN"),
    (add_hl_hl, "ADD HL, HL"),
    (ldi_a_mhl, "LDI A, [HL]"),
    (dec_hl, "DEC HL"),
    (inc_l, "INC L"),
    (dec_l, "DEC L"),
    (ld_l_n, "LD L, N"),
    (cpl, "CPL"),
    //Opcodes 3x
    (jr_nc_sn, "JR NC, SN"),
    (ld_sp_nn, "LD SP, NN"),
    (ldd_mhl_a, "LDD [HL], A"),
    (inc_sp, "INC SP"),
    (inc_mhl, "INC [HL]"),
    (dec_mhl, "DEC [HL]"),
    (ld_mhl_n, "LD [HL], N"),
    (scf, "SCF"),
    (jr_c_sn, "JR C, SN"),
    (add_hl_sp, "ADD HL, SP"),
    (ldd_a_mhl, "LLD A, [HL]"),
    (dec_sp, "DEC SP"),
    (inc_a, "INC A"),
    (dec_a, "DEC A"),
    (ld_a_n, "LD A, N"),
    (ccf, "CCF"),
    // Opcodes 4X
    (ld_b_b, "LD B, B"),
    (ld_b_c, "LD B, C"),
    (ld_b_d, "LD B, D"),
    (ld_b_e, "LD B, E"),
    (ld_b_h, "LD B, H"),
    (ld_b_l, "LD B, L"),
    (ld_b_mhl, "LD B, [HL]"),
    (ld_b_a, "LD B, A"),
    (ld_c_b, "LD C, B"),
    (ld_c_c, "LD C, C"),
    (ld_c_d, "LD C, D"),
    (ld_c_e, "LD C, E"),
    (ld_c_h, "LD C, H"),
    (ld_c_l, "LD C, L"),
    (ld_c_mhl, "LD C, [HL]"),
    (ld_c_a, "LD C, A"),
    // Opcodes 5X
    (ld_d_b, "LD D, B"),
    (ld_d_c, "LD D, C"),
    (ld_d_d, "LD D, D"),
    (ld_d_e, "LD D, E"),
    (ld_d_h, "LD D, H"),
    (ld_d_l, "LD D, L"),
    (ld_d_mhl, "LD D, [HL]"),
    (ld_d_a, "LD D, A"),
    (ld_e_b, "LD E, B"),
    (ld_e_c, "LD E, C"),
    (ld_e_d, "LD E, D"),
    (ld_e_e, "LD E, E"),
    (ld_e_h, "LD E, H"),
    (ld_e_l, "LD E, L"),
    (ld_e_mhl, "LD E, [HL]"),
    (ld_e_a, "LD E, A"),
    // Opcodes 6X
    (ld_h_b, "LD H, B"),
    (ld_h_c, "LD H, C"),
    (ld_h_d, "LD H, D"),
    (ld_h_e, "LD H, E"),
    (ld_h_h, "LD H, H"),
    (ld_h_l, "LD H, L"),
    (ld_h_mhl, "LD H, [HL]"),
    (ld_h_a, "LD H, A"),
    (ld_l_b, "LD L, B"),
    (ld_l_c, "LD L, C"),
    (ld_l_d, "LD L, D"),
    (ld_l_e, "LD L, E"),
    (ld_l_h, "LD L, H"),
    (ld_l_l, "LD L, L"),
    (ld_l_mhl, "LD L, [HL]"),
    (ld_l_a, "LD L, A"),
    //Opcodes 7X
    (ld_mhl_b, "LD [HL], B"),
    (ld_mhl_c, "LD [HL], C"),
    (ld_mhl_d, "LD [HL], D"),
    (ld_mhl_e, "LD [HL], E"),
    (ld_mhl_h, "LD [HL], H"),
    (ld_mhl_l, "LD [HL], L"),
    (halt, "HALT"),
    (ld_mhl_a, "LD [HL], A"),
    (ld_a_b, "LD A, B"),
    (ld_a_c, "LD A, C"),
    (ld_a_d, "LD A, D"),
    (ld_a_e, "LD A, E"),
    (ld_a_h, "LD A, H"),
    (ld_a_l, "LD A, L"),
    (ld_a_mhl, "LD A, [HL]"),
    (ld_a_a, "LD A, A"),
    //Opcodes 8X
    (add_a_b, "ADD A, B"),
    (add_a_c, "ADD A, C"),
    (add_a_d, "ADD A, D"),
    (add_a_e, "ADD A, E"),
    (add_a_h, "ADD A, H"),
    (add_a_l, "ADD A, L"),
    (add_a_mhl, "ADD A, [HL]"),
    (add_a_a, "ADD A, A"),
    (adc_a_b, "ADC A, B"),
    (adc_a_c, "ADC A, C"),
    (adc_a_d, "ADC A, D"),
    (adc_a_e, "ADC A, E"),
    (adc_a_h, "ADC A, H"),
    (adc_a_l, "ADC A, L"),
    (adc_a_mhl, "ADC A, [HL]"),
    (adc_a_a, "ADC A, A"),
    //Opcodes 9X
    (sub_a_b, "SUB A, B"),
    (sub_a_c, "SUB A, C"),
    (sub_a_d, "SUB A, D"),
    (sub_a_e, "SUB A, E"),
    (sub_a_h, "SUB A, H"),
    (sub_a_l, "SUB A, L"),
    (sub_a_mhl, "SUB A, [HL]"),
    (sub_a_a, "SUB A, A"),
    (sbc_a_b, "SBC A, B"),
    (sbc_a_c, "SBC A, C"),
    (sbc_a_d, "SBC A, D"),
    (sbc_a_e, "SBC A, E"),
    (sbc_a_h, "SBC A, H"),
    (sbc_a_l, "SBC A, L"),
    (sbc_a_mhl, "SBC A, [HL]"),
    (sbc_a_a, "SBC A, A"),
    //Opcodes AX
    (and_a_b, "AND A, B"),
    (and_a_c, "AND A, C"),
    (and_a_d, "AND A, D"),
    (and_a_e, "AND A, E"),
    (and_a_h, "AND A, H"),
    (and_a_l, "AND A, L"),
    (and_a_mhl, "AND A, [HL]"),
    (and_a_a, "AND A, A"),
    (xor_a_b, "XOR A, B"),
    (xor_a_c, "XOR A, C"),
    (xor_a_d, "XOR A, D"),
    (xor_a_e, "XOR A, E"),
    (xor_a_h, "XOR A, H"),
    (xor_a_l, "XOR A, L"),
    (xor_a_mhl, "XOR A, [HL]"),
    (xor_a_a, "XOR A, A"),
    //Opcodes BX
    (or_a_b, "OR A, B"),
    (or_a_c, "OR A, C"),
    (or_a_d, "OR A, D"),
    (or_a_e, "OR A, E"),
    (or_a_h, "OR A, H"),
    (or_a_l, "OR A, L"),
    (or_a_mhl, "OR A, [HL]"),
    (or_a_a, "OR A, A"),
    (cp_a_b, "CP A, B"),
    (cp_a_c, "CP A, C"),
    (cp_a_d, "CP A, D"),
    (cp_a_e, "CP A, E"),
    (cp_a_h, "CP A, H"),
    (cp_a_l, "CP A, L"),
    (cp_a_mhl, "CP A, [HL]"),
    (cp_a_a, "CP A, A"),
    //Opcodes CX
    (ret_nz, "RET NZ"),
    (pop_bc, "POP BC"),
    (jp_nz_nn, "JP NZ, NN"),
    (jp_nn, "JP NN"),
    (call_nz_nn, "CALL NZ, NN"),
    (push_bc, "PUSH BC"),
    (add_a_n, "ADD A, N"),
    (rst_00, "RST 00"),
    (ret_z, "RET Z"),
    (ret, "RET"),
    (jp_z_nn, "JP Z, NN"),
    (undefined, "UNDEFINED"),
    (call_z_nn, "CALL Z, NN"),
    (call_nn, "CALL NN"),
    (adc_a_n, "ADC A, N"),
    (rst_08, "RST 08"),
    //Opcodes DX
    (ret_nc, "RET NC"),
    (pop_de, "POP DE"),
    (jp_nc_nn, "JP NC, NN"),
    (undefined, "UNDEFINED"),
    (call_nc_nn, "CALL NC, NN"),
    (push_de, "PUSH DE"),
    (sub_a_n, "SUB A, N"),
    (rst_10, "RST 10"),
    (ret_c, "RET C"),
    (reti, "RETI"),
    (jp_c_nn, "JP C, NN"),
    (undefined, "UNDEFINED"),
    (call_c_nn, "CALL C, NN"),
    (undefined, "UNDEFINED"),
    (sbc_a_n, "SBC A, N"),
    (rst_18, "RST 18"),
    //OPCODES EX
    (ldh_mn_a, "LDH [N], A"),
    (pop_hl, "POP HL"),
    (ldh_mc_a, "LDH [C], A"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (push_hl, "PUSH HL"),
    (and_a_n, "AND A, N"),
    (rst_20, "RST 20"),
    (add_sp_sn, "ADD SP, SN"),
    (jp_hl, "JP HL"),
    (ld_mnn_a, "LD [NN], A"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (xor_a_n, "XOR A, N"),
    (rst_28, "RST 28"),
    //Opcodes FX
    (ldh_a_mn, "LDH A, [N]"),
    (pop_af, "POP AF"),
    (ldh_a_mc, "LDH A, [C]"),
    (di, "DI"),
    (undefined, "UNDEFINED"),
    (push_af, "PUSH A, F"),
    (or_a_n, "OR A, N"),
    (rst_30, "RST 30"),
    (ld_hl_sp_sn, "LD HL, SP, SN"),
    (ld_sp_hl, "LD SP, HL"),
    (ld_a_mnn, "LD A, [NN]"),
    (ei, "EI"),
    (undefined, "UNDEFINED"),
    (undefined, "UNDEFINED"),
    (cp_a_n, "CP A, N"),
    (rst_38, "RST 38"),
];

fn undefined(cpu: &mut Cpu) {
    let pc = cpu.pc.wrapping_sub(1);

    println!("Invalid instruction called at 0x{:04x}. CPU stalled.", pc);

    cpu.set_pc(pc);
}

fn nop(_: &mut Cpu) {}

fn rlca(cpu: &mut Cpu) {
    let c = cpu.registers.a >> 7;

    cpu.registers.a = cpu.registers.a << 1 | c;
    
    cpu.registers.f.carry = c != 0;
    cpu.registers.f.zero = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
}

fn rla(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let new_carry = (a >> 7) != 0;
    let old_carry = cpu.registers.f.carry as u8;

    cpu.registers.a = (a << 1) | old_carry;

    cpu.registers.f.carry = new_carry;
    cpu.registers.f.zero = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
}

fn rrca(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    let c = a & 1;

    cpu.registers.a = (a >> 1) | (c << 7);

    cpu.registers.f.carry = c != 0;
    cpu.registers.f.zero = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
}

fn rra(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    let new_carry = (a & 1) != 0;
    let oldcarry = cpu.registers.f.carry as u8;

    cpu.registers.a = (a >> 1) | (oldcarry << 7);

    cpu.registers.f.carry = new_carry;
    cpu.registers.f.zero = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
}

fn cpl(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.a = !a;

    cpu.registers.f.subtract = true;
    cpu.registers.f.half_carry = true;
}

fn daa(cpu: &mut Cpu) {
    let mut adjust = 0;
    if cpu.registers.f.half_carry {
        adjust |= 0x06;
    }

    if cpu.registers.f.carry {
        adjust |= 0x60;
    }

    let res = if cpu.registers.f.subtract {
        cpu.registers.a.wrapping_sub(adjust)
    } else {
        if cpu.registers.a & 0x0F > 0x09 {
            adjust |= 0x06;
        }

        if cpu.registers.a > 0x99 {
            adjust |= 0x60;
        }

        cpu.registers.a.wrapping_add(adjust)
    };

    cpu.registers.a = res;

    cpu.registers.f.zero = res == 0;
    cpu.registers.f.carry = adjust & 0x60 != 0;
    cpu.registers.f.half_carry = false;
}

fn ccf(cpu: &mut Cpu) {
    let carry = cpu.registers.f.carry;

    cpu.registers.f.carry = !carry;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
}

fn scf(cpu: &mut Cpu) {
    cpu.registers.f.carry = true;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
}

fn ld_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.a = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_b_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.b = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_c_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.c = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_d_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.d = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_e_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.e = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_h_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.h = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_l_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    cpu.registers.l = cpu.external.ram.read_byte(cpu.pc);
}

fn ld_a_a(_: &mut Cpu) {}

fn ld_b_b(_: &mut Cpu) {}

fn ld_c_c(_: &mut Cpu) {}

fn ld_d_d(_: &mut Cpu) {}

fn ld_e_e(_: &mut Cpu) {}

fn ld_h_h(_: &mut Cpu) {}

fn ld_l_l(_: &mut Cpu) {}

fn ld_a_b(cpu: &mut Cpu) {
    let v = cpu.registers.b;

    cpu.registers.a = v;
}

fn ld_a_c(cpu: &mut Cpu) {
    let v = cpu.registers.c;

    cpu.registers.a = v;
}

fn ld_a_d(cpu: &mut Cpu) {
    let v = cpu.registers.d;

    cpu.registers.a = v;
}

fn ld_a_e(cpu: &mut Cpu) {
    let v = cpu.registers.e;

    cpu.registers.a = v;
}

fn ld_a_h(cpu: &mut Cpu) {
    let v = cpu.registers.h;

    cpu.registers.a = v;
}

fn ld_a_l(cpu: &mut Cpu) {
    let v = cpu.registers.l;

    cpu.registers.a = v;
}

fn ld_a_mbc(cpu: &mut Cpu) {
    let bc = cpu.registers.bc();
    cpu.registers.a = cpu.external.ram.read_byte(bc);
}

fn ld_a_mde(cpu: &mut Cpu) {
    let de = cpu.registers.de();
    cpu.registers.a = cpu.external.ram.read_byte(de);
}

fn ld_a_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.a = cpu.external.ram.read_byte(hl);
}

fn ld_b_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.b = cpu.external.ram.read_byte(hl);
}

fn ld_c_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.c = cpu.external.ram.read_byte(hl);
}

fn ld_d_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.d = cpu.external.ram.read_byte(hl);
}

fn ld_e_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.e = cpu.external.ram.read_byte(hl);
}

fn ld_h_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.h = cpu.external.ram.read_byte(hl);
}

fn ld_l_mhl(cpu: &mut Cpu) {
    let hl = cpu.registers.hl();
    cpu.registers.l = cpu.external.ram.read_byte(hl);
}

fn ld_a_mnn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    cpu.registers.a = cpu.external.ram.read_byte(address);
}

fn ld_b_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    cpu.registers.b = a;
}

fn ld_c_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.c = a;
}

fn ld_d_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.d = a;
}

fn ld_e_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.e = a;
}

fn ld_h_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.h = a;
}

fn ld_l_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.l = a;
}

fn ld_mbc_a(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.bc(), cpu.registers.a);
}

fn ld_mde_a(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.de(), cpu.registers.a);
}

fn ld_mhl_a(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.a);
}

fn ld_mhl_b(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.b);
}

fn ld_mhl_c(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.c);
}

fn ld_mhl_d(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.d);
}

fn ld_mhl_e(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.e);
}

fn ld_mhl_h(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.h);
}

fn ld_mhl_l(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.l);
}

fn ld_mhl_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let n = cpu.external.ram.read_byte(cpu.pc);
    cpu.external.ram.store_byte(cpu.registers.hl(), n);
}

fn ld_mnn_a(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    cpu.external.ram.store_byte(address, cpu.registers.a);
}

fn ld_mnn_sp(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    cpu.external.ram.store_byte(address, (cpu.sp & 0xFF) as u8);
    cpu.external.ram.store_byte(address + 1, (cpu.sp >> 8) as u8);
}

fn ld_bc_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8; 
    cpu.registers.set_bc(value);
}

fn ld_de_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8; 
    cpu.registers.set_de(value);
}

fn ld_hl_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8; 
    cpu.registers.set_hl(value);
}

fn ld_hl_sp_sn(cpu: &mut Cpu) {
    let sp = cpu.sp as i32;
    cpu.set_pc(cpu.pc + 1) ;
    let n = cpu.external.ram.read_byte(cpu.pc) as i8;
    let nn = n as i32;
    let result = sp.wrapping_add(nn);
    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = (sp ^ nn ^ result) & 0x03 != 0;
    cpu.registers.f.carry = (sp ^ nn ^ result) & 0x07 != 0;
    cpu.registers.f.zero = false;

    cpu.registers.set_hl(result as u16);
}

fn ld_sp_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    cpu.sp = value;
}

fn ld_sp_hl(cpu: &mut Cpu) {
    cpu.sp = cpu.registers.hl();
}

fn ld_b_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;

    cpu.registers.b = c;
}

fn ld_b_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;

    cpu.registers.b = d;
}

fn ld_b_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;

    cpu.registers.b = e;
}

fn ld_b_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;

    cpu.registers.b = h;
}

fn ld_b_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;

    cpu.registers.b = l;
}

fn ld_c_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;

    cpu.registers.c = b;
}

fn ld_c_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;

    cpu.registers.c = d;
}

fn ld_c_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;

    cpu.registers.c = e;
}

fn ld_c_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;

    cpu.registers.c = h;
}

fn ld_c_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;

    cpu.registers.c = l;
}

fn ld_d_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;

    cpu.registers.d = b;
}

fn ld_d_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;

    cpu.registers.d = c;
}

fn ld_d_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;

    cpu.registers.d = e;
}

fn ld_d_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;

    cpu.registers.d = h;
}

fn ld_d_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;

    cpu.registers.d = l;
}

fn ld_e_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;

    cpu.registers.e = b;
}

fn ld_e_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;

    cpu.registers.e = c;
}

fn ld_e_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;

    cpu.registers.e = d;
}

fn ld_e_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;

    cpu.registers.e = h;
}

fn ld_e_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;

    cpu.registers.e = l;
}

fn ld_h_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;

    cpu.registers.h = b;
}

fn ld_h_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;

    cpu.registers.h = c;
}

fn ld_h_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;

    cpu.registers.h = d;
}

fn ld_h_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;

    cpu.registers.h = e;
}

fn ld_l_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;

    cpu.registers.l = b;
}

fn ld_l_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;

    cpu.registers.l = c;
}

fn ld_l_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;

    cpu.registers.l = d;
}

fn ld_l_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;

    cpu.registers.l = e;
}

fn ld_l_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;

    cpu.registers.l = h;
}

fn ld_h_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;

    cpu.registers.h = l;
}

fn pop_af(cpu: &mut Cpu) {
    let value = cpu.pop_sp();

    cpu.registers.set_af(value);
}

fn pop_bc(cpu: &mut Cpu) {
    let value = cpu.pop_sp();

    cpu.registers.set_bc(value);
}

fn pop_de(cpu: &mut Cpu) {
    let value = cpu.pop_sp();

    cpu.registers.set_de(value);
}

fn pop_hl(cpu: &mut Cpu) {
    let value = cpu.pop_sp();

    cpu.registers.set_hl(value);
}

fn push_af(cpu: &mut Cpu) {
    cpu.push_sp(cpu.registers.af());
}

fn push_bc(cpu: &mut Cpu) {
    cpu.push_sp(cpu.registers.bc());
}

fn push_de(cpu: &mut Cpu) {
    cpu.push_sp(cpu.registers.de());
}

fn push_hl(cpu: &mut Cpu) {
    cpu.push_sp(cpu.registers.hl());
}

fn jp_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    cpu.load_pc(value);
}

fn jp_hl(cpu: &mut Cpu) {
    cpu.set_pc(cpu.registers.hl());
}

fn jp_nz_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    
    if !cpu.registers.f.zero {
        cpu.load_pc(value);
    }
}

fn jp_z_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;

    if cpu.registers.f.zero {
        cpu.load_pc(value);
    }
}

fn jp_nc_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;

    if !cpu.registers.f.carry {
        cpu.load_pc(value);
    }
}

fn jp_c_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut value = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    value |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    
    if cpu.registers.f.carry {
        cpu.load_pc(value);
    }
}

fn jr_sn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let offset = cpu.external.ram.read_byte(cpu.pc) as i8;
    let mut pc = cpu.pc as i16;
    pc += 1;
    pc += offset as i16;

    cpu.load_pc(pc as u16);
}

fn jr_nz_sn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let offset = cpu.external.ram.read_byte(cpu.pc) as i8;
    if !cpu.registers.f.zero {
        let mut pc = cpu.pc as i16;
        pc += 1;
        pc += offset as i16;

        cpu.load_pc(pc as u16);
    }
    
}

fn jr_z_sn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let offset = cpu.external.ram.read_byte(cpu.pc) as i8;
    if cpu.registers.f.zero {
        let mut pc = cpu.pc as i16;
        pc += 1;
        pc += offset as i16;

        cpu.load_pc(pc as u16);
    }
    
}

fn jr_nc_sn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let offset = cpu.external.ram.read_byte(cpu.pc) as i8;
    if !cpu.registers.f.carry {
        let mut pc = cpu.pc as i16;
        pc += 1;
        pc += offset as i16;

        cpu.load_pc(pc as u16);
    }
}

fn jr_c_sn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let offset = cpu.external.ram.read_byte(cpu.pc) as i8;
    if cpu.registers.f.zero {
        let mut pc = cpu.pc as i16;
        pc += 1;
        pc += offset as i16;

        cpu.load_pc(pc as u16);
    }
}

fn rst(cpu: &mut Cpu, addr: u16) {
    cpu.push_sp(cpu.pc);

    cpu.load_pc(addr);
}

fn rst_00(cpu: &mut Cpu) {
    rst(cpu, 0x00);
}

fn rst_08(cpu: &mut Cpu) {
    rst(cpu, 0x08);
}

fn rst_10(cpu: &mut Cpu) {
    rst(cpu, 0x10);
}

fn rst_18(cpu: &mut Cpu) {
    rst(cpu, 0x18);
}

fn rst_20(cpu: &mut Cpu) {
    rst(cpu, 0x20);
}


fn rst_28(cpu: &mut Cpu) {
    rst(cpu, 0x28);
}

fn rst_30(cpu: &mut Cpu) {
    rst(cpu, 0x30);
}

fn rst_38(cpu: &mut Cpu) {
    rst(cpu, 0x38);
}

fn call_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    cpu.push_sp(cpu.pc + 1);

    cpu.load_pc(address);
}

fn call_nz_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    if !cpu.registers.f.zero {
        cpu.push_sp(cpu.pc + 1);

        cpu.load_pc(address);
    }
}


fn call_z_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    if cpu.registers.f.zero {
        cpu.push_sp(cpu.pc + 1);

        cpu.load_pc(address);
    }
}

fn call_nc_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    if !cpu.registers.f.carry {
        cpu.push_sp(cpu.pc + 1);

        cpu.load_pc(address);
    }
}

fn call_c_nn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let mut address = cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.set_pc(cpu.pc + 1);
    address |= (cpu.external.ram.read_byte(cpu.pc) as u16) << 8;
    if cpu.registers.f.carry {
        cpu.push_sp(cpu.pc + 1);

        cpu.load_pc(address);
    }
}

fn ret(cpu: &mut Cpu) {
    let address = cpu.pop_sp();
    cpu.set_pc(address);
}

fn reti(cpu: &mut Cpu) {
    let address = cpu.pop_sp();
    cpu.set_pc(address);
    cpu.interrupts = true;
}

fn ret_nz(cpu: &mut Cpu) {
    if !cpu.registers.f.zero {
        let address = cpu.pop_sp();
        cpu.set_pc(address);
    }
}

fn ret_z(cpu: &mut Cpu) {
    if cpu.registers.f.zero {
        let address = cpu.pop_sp();
        cpu.set_pc(address);
    }
}

fn ret_nc(cpu: &mut Cpu) {
    if !cpu.registers.f.carry {
        let address = cpu.pop_sp();
        cpu.set_pc(address);
    }
}

fn ret_c(cpu: &mut Cpu) {
    if cpu.registers.f.carry {
        let address = cpu.pop_sp();
        cpu.set_pc(address);
    }
}

fn ldd_mhl_a(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.a);
    cpu.registers.set_hl(cpu.registers.hl() - 1);
}

fn ldd_a_mhl(cpu: &mut Cpu) {
    cpu.registers.a = cpu.external.ram.read_byte(cpu.registers.hl());
    cpu.registers.set_hl(cpu.registers.hl() - 1);
}

fn ldi_mhl_a(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(cpu.registers.hl(), cpu.registers.a);
    cpu.registers.set_hl(cpu.registers.hl() + 1);
}

fn ldi_a_mhl(cpu: &mut Cpu) {
    cpu.registers.a = cpu.external.ram.read_byte(cpu.registers.hl());
    cpu.registers.set_hl(cpu.registers.hl() + 1);
}

fn ldh_mn_a(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = 0xff00 | cpu.external.ram.read_byte(cpu.pc) as u16;
    cpu.external.ram.store_byte(address, cpu.registers.a);
}

fn ldh_mc_a(cpu: &mut Cpu) {
    cpu.external.ram.store_byte(0xff00 | (cpu.registers.c as u16), cpu.registers.a);
}

fn ldh_a_mn(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = 0xff00 | cpu.external.ram.read_byte(cpu.pc) as u16;
    let value = cpu.external.ram.read_byte(address);

    cpu.registers.a = value;
}

fn ldh_a_mc(cpu: &mut Cpu) {
    let address = 0xff00 | cpu.registers.c as u16;
    let value = cpu.external.ram.read_byte(address);

    cpu.registers.a = value;
}

fn dec_a(cpu: &mut Cpu) {
    let mut a = cpu.registers.a;

    cpu.registers.f.half_carry = a & 0xf == 0;

    a = a.wrapping_sub(1);

    cpu.registers.a = a;

    cpu.registers.f.zero = a == 0;
    cpu.registers.f.zero = true;
}

fn dec_b(cpu: &mut Cpu) {
    let mut b = cpu.registers.b;

    cpu.registers.f.half_carry = b & 0xf == 0;

    b = b.wrapping_sub(1);

    cpu.registers.b = b;

    cpu.registers.f.zero = b == 0;
    cpu.registers.f.zero = true;
}

fn dec_c(cpu: &mut Cpu) {
    let mut c = cpu.registers.c;

    cpu.registers.f.half_carry = c & 0xf == 0;

    c = c.wrapping_sub(1);

    cpu.registers.c = c;

    cpu.registers.f.zero = c == 0;
    cpu.registers.f.zero = true;
}

fn dec_d(cpu: &mut Cpu) {
    let mut d = cpu.registers.d;

    cpu.registers.f.half_carry = d & 0xf == 0;

    d = d.wrapping_sub(1);

    cpu.registers.d = d;

    cpu.registers.f.zero = d == 0;
    cpu.registers.f.zero = true;
}

fn dec_e(cpu: &mut Cpu) {
    let mut e = cpu.registers.e;

    cpu.registers.f.half_carry = e & 0xf == 0;

    e = e.wrapping_sub(1);

    cpu.registers.e = e;

    cpu.registers.f.zero = e == 0;
    cpu.registers.f.zero = true;
}

fn dec_h(cpu: &mut Cpu) {
    let mut h = cpu.registers.h;

    cpu.registers.f.half_carry = h & 0xf == 0;

    h = h.wrapping_sub(1);

    cpu.registers.h = h;

    cpu.registers.f.zero = h == 0;
    cpu.registers.f.zero = true;
}

fn dec_l(cpu: &mut Cpu) {
    let mut l = cpu.registers.l;

    cpu.registers.f.half_carry = l & 0xf == 0;

    l = l.wrapping_sub(1);

    cpu.registers.l = l;

    cpu.registers.f.zero = l == 0;
    cpu.registers.f.zero = true;
}

fn dec_mhl(cpu: &mut Cpu) {
    let mut n = cpu.external.ram.read_byte(cpu.registers.hl());
    cpu.registers.f.half_carry = n & 0xf == 0;
    n -= 1;

    cpu.external.ram.store_byte(cpu.registers.hl(), n);

    cpu.registers.f.zero = n == 0;
    cpu.registers.f.subtract = true;
}

fn inc_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    let r = a.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;
    cpu.registers.f.half_carry = a & 0xf == 0xf;

    cpu.registers.a = r;
}

fn inc_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;

    let r = b.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;
    cpu.registers.f.half_carry = b & 0xf == 0xf;

    cpu.registers.b = r;
}

fn inc_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;

    let r = c.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;
    cpu.registers.f.half_carry = c & 0xf == 0xf;

    cpu.registers.c = r;
}

fn inc_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;

    let r = d.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero =  r == 0;
    cpu.registers.f.half_carry = d & 0xf == 0xf;

    cpu.registers.d = r;
}

fn inc_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;

    let r = e.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;
    cpu.registers.f.half_carry = e & 0xf == 0xf;

    cpu.registers.e = r;
}

fn inc_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;

    let r = h.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;
    cpu.registers.f.half_carry = h & 0xf == 0xf;

    cpu.registers.h = r;
}

fn inc_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;

    let r = l.wrapping_add(1);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;
    cpu.registers.f.half_carry = l & 0xf == 0xf;

    cpu.registers.l = r;
}

fn inc_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let value = address + 1;
    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = value == 0;
    cpu.registers.f.half_carry = address & 0xf == 0xf;
    cpu.external.ram.store_byte(cpu.registers.hl(), value);
}

fn inc_bc(cpu: &mut Cpu) {
    cpu.registers.set_bc(cpu.registers.bc() + 1);
}

fn inc_de(cpu: &mut Cpu) {
    cpu.registers.set_de(cpu.registers.de() + 1);
}

fn inc_hl(cpu: &mut Cpu) {
    cpu.registers.set_hl(cpu.registers.hl() + 1);
}

fn inc_sp(cpu: &mut Cpu) {
    cpu.set_sp(cpu.sp + 1);
}

fn dec_bc(cpu: &mut Cpu) {
    cpu.registers.set_bc(cpu.registers.bc() - 1);
}

fn dec_de(cpu: &mut Cpu) {
    cpu.registers.set_de(cpu.registers.de() - 1);
}

fn dec_hl(cpu: &mut Cpu) {
    cpu.registers.set_hl(cpu.registers.hl() - 1);
}

fn dec_sp(cpu: &mut Cpu) {
    cpu.set_sp(cpu.sp - 1);
}

fn sub_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    let x = x as u32;
    let y = y as u32;

    let r = x.wrapping_sub(y);

    let rb = r as u8;

    cpu.registers.f.zero = rb == 0;
    cpu.registers.f.half_carry = (x ^ y ^  r) &  0x10 != 0;
    cpu.registers.f.half_carry =  r & 0x100 != 0;
    cpu.registers.f.subtract = true;

    rb
}

fn cp_a_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    sub_and_set_flags(cpu, a, a);
}

fn cp_a_b(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let b = cpu.registers.b;

    sub_and_set_flags(cpu, a, b);
}

fn cp_a_c(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let c = cpu.registers.c;

    sub_and_set_flags(cpu, a, c);
}

fn cp_a_d(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let d = cpu.registers.d;

    sub_and_set_flags(cpu, a, d);
}

fn cp_a_e(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let e = cpu.registers.e;

    sub_and_set_flags(cpu, a, e);
}

fn cp_a_h(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let h = cpu.registers.h;

    sub_and_set_flags(cpu, a, h);
}

fn cp_a_l(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let l = cpu.registers.l;

    sub_and_set_flags(cpu, a, l);
}

fn cp_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    sub_and_set_flags(cpu, cpu.registers.a, address);
}

fn cp_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    sub_and_set_flags(cpu, cpu.registers.a, address);
}

fn sub_a_a(cpu: &mut Cpu) {
    cpu.registers.f.zero = true;
    cpu.registers.f.subtract = true;
    cpu.registers.f.carry = false;
    cpu.registers.f.half_carry = false;

    cpu.registers.a = 0;
}

fn sub_a_b(cpu: &mut Cpu) {
    let r = sub_and_set_flags(cpu, cpu.registers.a, cpu.registers.b);
    cpu.registers.a = r;
}


fn sub_a_c(cpu: &mut Cpu) {
    let r = sub_and_set_flags(cpu, cpu.registers.a, cpu.registers.c);
    cpu.registers.a = r;
}

fn sub_a_d(cpu: &mut Cpu) {
    let r = sub_and_set_flags(cpu, cpu.registers.a, cpu.registers.d);
    cpu.registers.a = r;
}

fn sub_a_e(cpu: &mut Cpu) {
    let r = sub_and_set_flags(cpu, cpu.registers.a, cpu.registers.e);
    cpu.registers.a = r;
}

fn sub_a_h(cpu: &mut Cpu) {
    let r = sub_and_set_flags(cpu, cpu.registers.a, cpu.registers.h);
    cpu.registers.a = r;
}

fn sub_a_l(cpu: &mut Cpu) {
    let r = sub_and_set_flags(cpu, cpu.registers.a, cpu.registers.l);
    cpu.registers.a = r;
}

fn sub_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let value = sub_and_set_flags(cpu, cpu.registers.a, address);

    cpu.registers.a = value;
}

fn sub_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let value = sub_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn sub_with_carry_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    let x = x as u32;
    let y = y as u32;
    let carry = cpu.registers.f.carry as u32;

    let r = x.wrapping_sub(y).wrapping_sub(carry);

    let rb = r as u8;

    cpu.registers.f.zero = rb == 0;
    cpu.registers.f.half_carry = (x ^ y ^  r) &  0x10 != 0;
    cpu.registers.f.half_carry =  r & 0x100 != 0;
    cpu.registers.f.subtract = true;

    rb
}

fn sbc_a_a(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.a);
    cpu.registers.a = r;
}

fn sbc_a_b(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.b);
    cpu.registers.a = r;
}

fn sbc_a_c(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.c);
    cpu.registers.a = r;
}

fn sbc_a_d(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.d);
    cpu.registers.a = r;
}

fn sbc_a_e(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.e);
    cpu.registers.a = r;
}

fn sbc_a_h(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.h);
    cpu.registers.a = r;
}

fn sbc_a_l(cpu: &mut Cpu) {
    let r = sub_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.l);
    cpu.registers.a = r;
}

fn sbc_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let value = sub_with_carry_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn sbc_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let value = sub_with_carry_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn add_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    let x = x as u32;
    let y = y as u32;

    let r = x.wrapping_add(y);

    let rb = r as u8;

    cpu.registers.f.zero = rb == 0;
    cpu.registers.f.half_carry = (x ^ y ^  r) &  0x10 != 0;
    cpu.registers.f.half_carry =  r & 0x100 != 0;
    cpu.registers.f.subtract = false;

    rb
}

fn add_a_a(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.a);
    cpu.registers.a = r;
}

fn add_a_b(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.b);
    cpu.registers.a = r;
}

fn add_a_c(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.c);
    cpu.registers.a = r;
}

fn add_a_d(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.d);
    cpu.registers.a = r;
}

fn add_a_e(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.e);
    cpu.registers.a = r;
}

fn add_a_h(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.h);
    cpu.registers.a = r;
}

fn add_a_l(cpu: &mut Cpu) {
    let r = add_and_set_flags(cpu, cpu.registers.a, cpu.registers.l);
    cpu.registers.a = r;
}

fn add_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let value = add_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn add_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let value = add_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn add_with_carry_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    let x = x as u32;
    let y = y as u32;
    let carry = cpu.registers.f.carry as u32;

    let r = x.wrapping_add(y).wrapping_add(carry);

    let rb = r as u8;

    cpu.registers.f.zero = rb == 0;
    cpu.registers.f.half_carry = (x ^ y ^  r) &  0x10 != 0;
    cpu.registers.f.half_carry =  r & 0x100 != 0;
    cpu.registers.f.subtract = false;

    rb
}

fn adc_a_a(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.a);
    cpu.registers.a = r;
}

fn adc_a_b(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.b);
    cpu.registers.a = r;
}

fn adc_a_c(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.c);
    cpu.registers.a = r;
}

fn adc_a_d(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.d);
    cpu.registers.a = r;
}

fn adc_a_e(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.e);
    cpu.registers.a = r;
}

fn adc_a_h(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.h);
    cpu.registers.a = r;
}

fn adc_a_l(cpu: &mut Cpu) {
    let r = add_with_carry_and_set_flags(cpu, cpu.registers.a, cpu.registers.l);
    cpu.registers.a = r;
}

fn adc_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let value = add_with_carry_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn adc_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let value = add_with_carry_and_set_flags(cpu, cpu.registers.a, address);
    cpu.registers.a = value;
}

fn add_word_and_set_flags(cpu: &mut Cpu, x: u16, y: u16) -> u16 {
    let x = x as u32;
    let y = y as u32;
    let result = x + y;

    cpu.registers.f.subtract = false;
    cpu.registers.f.carry = result & 0x1000 != 0;
    cpu.registers.f.half_carry = (x ^ y ^ result) & 0x1000 != 0;

    result as u16
}

fn add_hl_bc(cpu: &mut Cpu) {
    let r = add_word_and_set_flags(cpu, cpu.registers.hl(), cpu.registers.bc());

    cpu.registers.set_hl(r);
}

fn add_hl_de(cpu: &mut Cpu) {
    let r = add_word_and_set_flags(cpu, cpu.registers.hl(), cpu.registers.de());

    cpu.registers.set_hl(r);
}

fn add_hl_hl(cpu: &mut Cpu) {
    let r = add_word_and_set_flags(cpu, cpu.registers.hl(), cpu.registers.hl());

    cpu.registers.set_hl(r);
}

fn add_hl_sp(cpu: &mut Cpu) {
    let r = add_word_and_set_flags(cpu, cpu.registers.hl(), cpu.sp);

    cpu.registers.set_hl(r);
}

fn add_sp_sn(cpu: &mut Cpu) {
    let sp = cpu.sp as i32;
    cpu.set_pc(cpu.pc + 1);
    let n = cpu.external.ram.read_byte(cpu.pc) as i8;
    let nn = n as i32;

    let result = sp + nn;

    cpu.registers.f.carry = (sp ^ nn ^ result) & 0x100 != 0;
    cpu.registers.f.half_carry = (sp ^ nn ^ result) & 0x100 != 0;
    cpu.set_sp(result as u16);

    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = false;
}

fn and_a_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.f.zero = a == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;
}

fn and_a_b(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let b = cpu.registers.b;

    let r = a & b;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn and_a_c(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let c = cpu.registers.b;

    let r = a & c;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn and_a_d(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let d = cpu.registers.d;

    let r = a & d;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn and_a_e(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let e = cpu.registers.e;

    let r = a & e;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn and_a_h(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let h = cpu.registers.h;

    let r = a & h;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn and_a_l(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let l = cpu.registers.l;

    let r = a & l;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn and_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = cpu.registers.a & address;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = result;
}

fn and_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let result = cpu.registers.a & address;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;

    cpu.registers.a = result;
}

fn or_a_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;

    cpu.registers.f.zero = a == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
}

fn or_a_b(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let b = cpu.registers.b;

    let r = a | b;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn or_a_c(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let c = cpu.registers.c;

    let r = a | c;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn or_a_d(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let d = cpu.registers.d;

    let r = a | d;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn or_a_e(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let e = cpu.registers.e;

    let r = a | e;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn or_a_h(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let h = cpu.registers.h;

    let r = a | h;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = r;
}

fn or_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = cpu.registers.a | address;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = result;
}

fn or_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let result = cpu.registers.a | address;
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    cpu.registers.a = result;
}

fn xor(cpu: &mut Cpu, n: u8, m: u8) -> u8 {
    let r = n ^ m;

    cpu.registers.f.carry = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = r == 0;

    r
}

fn xor_a_a(cpu: &mut Cpu) {
    cpu.registers.a = 0;

    cpu.registers.f.carry = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtract = false;
    cpu.registers.f.zero = true;
}

fn xor_a_b(cpu: &mut Cpu) {
    let r = xor(cpu, cpu.registers.a, cpu.registers.b);

    cpu.registers.a = r;
}

fn xor_a_c(cpu: &mut Cpu) {
    let r = xor(cpu, cpu.registers.a, cpu.registers.c);

    cpu.registers.a = r;
}

fn xor_a_d(cpu: &mut Cpu) {
    let r = xor(cpu, cpu.registers.a, cpu.registers.d);

    cpu.registers.a = r;
}

fn xor_a_e(cpu: &mut Cpu) {
    let r = xor(cpu, cpu.registers.a, cpu.registers.e);

    cpu.registers.a = r;
}

fn xor_a_h(cpu: &mut Cpu) {
    let r = xor(cpu, cpu.registers.a, cpu.registers.h);

    cpu.registers.a = r;
}

fn xor_a_l(cpu: &mut Cpu) {
    let r = xor(cpu, cpu.registers.a, cpu.registers.l);

    cpu.registers.a = r;
}

fn xor_a_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = xor(cpu, cpu.registers.a, address);

    cpu.registers.a = result;
}

fn xor_a_n(cpu: &mut Cpu) {
    cpu.set_pc(cpu.pc + 1);
    let address = cpu.external.ram.read_byte(cpu.pc);
    let result = xor(cpu, cpu.registers.a, address);

    cpu.registers.a = result;
}

fn or_a_l(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.l;

    cpu.registers.f.zero = r == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.zero = false;

    cpu.registers.a = r;
}

fn di(cpu: &mut Cpu) {
    cpu.interrupts = false;
}

fn ei(cpu: &mut Cpu) {
    cpu.interrupts = true;
}

fn halt(cpu: &mut Cpu) {
    todo!()
}

fn stop(cpu: &mut Cpu) {
    todo!()
}