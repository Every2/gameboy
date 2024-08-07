use crate::cpu::Cpu;

pub static OPCODES: [(fn (&mut Cpu), &str); 0x100] = [
    //OPCODES CB 0X
    (rlc_b, "RLC B"),
    (rlc_c, "RLC C"),
    (rlc_d, "RLC D"),
    (rlc_e, "RLC E"),
    (rlc_h, "RLC H"),
    (rlc_l, "RLC l"),
    (rlc_mhl, "RLC [HL]"),
    (rlc_a, "RRC A"),
    (rrc_b, "RRC B"),
    (rrc_c, "RRC C"),
    (rrc_d, "RRC D"),
    (rrc_e, "RRC E"),
    (rrc_h, "RRC H"),
    (rrc_l, "RRC l"),
    (rrc_mhl, "RRC [HL]"),
    (rrc_a, "RRC A"),
    //OPCODES CB 1X
    (rl_b, "RL B"),
    (rl_c, "RL C"),
    (rl_d, "RL D"),
    (rl_e, "RL E"),
    (rl_h, "RL H"),
    (rl_l, "RL l"),
    (rl_mhl, "RL [HL]"),
    (rl_a, "RL A"),
    (rr_b, "RR B"),
    (rr_c, "RR C"),
    (rr_d, "RR D"),
    (rr_e, "RR E"),
    (rr_h, "RR H"),
    (rr_l, "RR l"),
    (rr_mhl, "RR [HL]"),
    (rr_a, "RR A"),
    //opcodes CB 2X
    (sla_b, "SLA B"),
    (sla_c, "SLA C"),
    (sla_d, "SLA D"),
    (sla_e, "SLA E"),
    (sla_h, "SLA H"),
    (sla_l, "SLA L"),
    (sla_mhl, "SLA [HL]"),
    (sla_a, "SRA A"),
    (sra_b, "SRA B"),
    (sra_c, "SRA C"),
    (sra_d, "SRA D"),
    (sra_e, "SRA E"),
    (sra_h, "SRA H"),
    (sra_l, "SRA L"),
    (sra_mhl, "SRA [HL]"),
    (sra_a, "SRA A"),
    //opcodes CB 3X
    (swap_b, "SWAP B"),
    (swap_c, "SWAP C"),
    (swap_d, "SWAP D"),
    (swap_e, "SWAP E"),
    (swap_h, "SWAP H"),
    (swap_l, "SWAP L"),
    (swap_mhl, "SWAP [HL]"),
    (swap_a, "SWAP A"),
    (srl_b, "SRL B"),
    (srl_c, "SRL C"),
    (srl_d, "SRL D"),
    (srl_e, "SRL E"),
    (srl_h, "SRL H"),
    (srl_l, "SRL L"),
    (srl_mhl, "SRL [HL]"),
    (srl_a, "SRL A"),
    //opcodes CB 4X
    (bit_b_0, "BIT B, 0"),
    (bit_c_0, "BIT C, 0"),
    (bit_d_0, "BIT D, 0"),
    (bit_e_0, "BIT E, 0"),
    (bit_h_0, "BIT H, 0"),
    (bit_l_0, "BIT L, 0"),
    (bit_mhl_0, "BIT [HL], 0"),
    (bit_a_1, "BIT A, 0"),
    (bit_b_1, "BIT B, 1"),
    (bit_c_1, "BIT C, 1"),
    (bit_d_1, "BIT D, 1"),
    (bit_e_1, "BIT E, 1"),
    (bit_h_1, "BIT H, 1"),
    (bit_l_1, "BIT L, 1"),
    (bit_mhl_1, "BIT [HL], 1"),
    (bit_a_2, "BIT A, 1"),
    //opcodes CB 5X
    (bit_b_2, "BIT B, 2"),
    (bit_c_2, "BIT C, 2"),
    (bit_d_2, "BIT D, 2"),
    (bit_e_2, "BIT E, 2"),
    (bit_h_2, "BIT H, 2"),
    (bit_l_2, "BIT L, 2"),
    (bit_mhl_2, "BIT [HL], 2"),
    (bit_a_2, "BIT A, 2"),
    (bit_b_3, "BIT B, 3"),
    (bit_c_3, "BIT C, 3"),
    (bit_d_3, "BIT D, 3"),
    (bit_e_3, "BIT E, 3"),
    (bit_h_3, "BIT H, 3"),
    (bit_l_3, "BIT L, 3"),
    (bit_mhl_3, "BIT [HL], 3"),
    (bit_a_3, "BIT A, 3"),
    //Opcodes CB 6X
    (bit_b_4, "BIT B, 4"),
    (bit_c_4, "BIT C, 4"),
    (bit_d_4, "BIT D, 4"),
    (bit_e_4, "BIT E, 4"),
    (bit_h_4, "BIT H, 4"),
    (bit_l_4, "BIT L, 4"),
    (bit_mhl_4, "BIT [HL], 4"),
    (bit_a_4, "BIT A, 4"),
    (bit_b_5, "BIT B, 5"),
    (bit_c_5, "BIT C, 5"),
    (bit_d_5, "BIT D, 5"),
    (bit_e_5, "BIT E, 5"),
    (bit_h_5, "BIT H, 5"),
    (bit_l_5, "BIT L, 5"),
    (bit_mhl_5, "BIT [HL], 5"),
    (bit_a_5, "BIT A, 5"),
    //Opcodes CB 7X
    (bit_b_6, "BIT B, 6"),
    (bit_c_6, "BIT C, 6"),
    (bit_d_6, "BIT D, 6"),
    (bit_e_6, "BIT E, 6"),
    (bit_h_6, "BIT H, 6"),
    (bit_l_6, "BIT L, 6"),
    (bit_mhl_6, "BIT [HL], 6"),
    (bit_a_6, "BIT A, 6"),
    (bit_b_7, "BIT B, 7"),
    (bit_c_7, "BIT C, 7"),
    (bit_d_7, "BIT D, 7"),
    (bit_e_7, "BIT E, 7"),
    (bit_h_7, "BIT H, 7"),
    (bit_l_7, "BIT L, 7"),
    (bit_mhl_7, "BIT [HL], 7"),
    (bit_a_7, "BIT A, 7"),
    //Opcodes CB 8X
    (res_b_0, "RES B, 0"),
    (res_c_0, "RES C, 0"),
    (res_d_0, "RES D, 0"),
    (res_e_0, "RES E, 0"),
    (res_h_0, "RES H, 0"),
    (res_l_0, "RES L, 0"),
    (res_mhl_0, "RES [HL], 0"),
    (res_a_0, "RES A, 0"),
    (res_b_1, "RES B, 1"),
    (res_c_1, "RES C, 1"),
    (res_d_1, "RES D, 1"),
    (res_e_1, "RES E, 1"),
    (res_h_1, "RES H, 1"),
    (res_l_1, "RES L, 1"),
    (res_mhl_1, "RES [HL], 1"),
    (res_a_1, "RES A, 1"),
    //Opcodes CB 9X
    (res_b_2, "RES B, 2"),
    (res_c_2, "RES C, 2"),
    (res_d_2, "RES D, 2"),
    (res_e_2, "RES E, 2"),
    (res_h_2, "RES H, 2"),
    (res_l_2, "RES L, 2"),
    (res_mhl_2, "RES [HL], 2"),
    (res_a_2, "RES A, 2"),
    (res_b_3, "RES B, 3"),
    (res_c_3, "RES C, 3"),
    (res_d_3, "RES D, 3"),
    (res_e_3, "RES E, 3"),
    (res_h_3, "RES H, 3"),
    (res_l_3, "RES L, 3"),
    (res_mhl_3, "RES [HL], 3"),
    (res_a_3, "RES A, 3"),
    //Opcodes AX
    (res_b_4, "RES B, 4"),
    (res_c_4, "RES C, 4"),
    (res_d_4, "RES D, 4"),
    (res_e_4, "RES E, 4"),
    (res_h_4, "RES H, 4"),
    (res_l_4, "RES L, 4"),
    (res_mhl_4, "RES [HL], 4"),
    (res_a_4, "RES A, 4"),
    (res_b_5, "RES B, 5"),
    (res_c_5, "RES C, 5"),
    (res_d_5, "RES D, 5"),
    (res_e_5, "RES E, 5"),
    (res_h_5, "RES H, 5"),
    (res_l_5, "RES L, 5"),
    (res_mhl_5, "RES [HL], 5"),
    (res_a_5, "RES A, 5"),
    //opcodes CB BX
    (res_b_6, "RES B, 6"),
    (res_c_6, "RES C, 6"),
    (res_d_6, "RES D, 6"),
    (res_e_6, "RES E, 6"),
    (res_h_6, "RES H, 6"),
    (res_l_6, "RES L, 6"),
    (res_mhl_6, "RES [HL], 6"),
    (res_a_6, "RES A, 6"),
    (res_b_7, "RES B, 7"),
    (res_c_7, "RES C, 7"),
    (res_d_7, "RES D, 7"),
    (res_e_7, "RES E, 7"),
    (res_h_7, "RES H, 7"),
    (res_l_7, "RES L, 7"),
    (res_mhl_7, "RES [HL], 7"),
    (res_a_7, "RES A, 7"),
    //opcodes CB CX
    (set_b_0, "SET B, 0"),
    (set_c_0, "SET C, 0"),
    (set_d_0, "SET D, 0"),
    (set_e_0, "SET E, 0"),
    (set_h_0, "SET H, 0"),
    (set_l_0, "SET L, 0"),
    (set_mhl_0, "SET [HL], 0"),
    (set_a_0, "SET A, 0"),
    (set_b_1, "SET B, 1"),
    (set_c_1, "SET C, 1"),
    (set_d_1, "SET D, 1"),
    (set_e_1, "SET E, 1"),
    (set_h_1, "SET H, 1"),
    (set_l_1, "SET L, 1"),
    (set_mhl_1, "SET [HL], 1"),
    (set_a_1, "SET A, 1"),
    //opcodes CB DX
    (set_b_2, "SET B, 2"),
    (set_c_2, "SET C, 2"),
    (set_d_2, "SET D, 2"),
    (set_e_2, "SET E, 2"),
    (set_h_2, "SET H, 2"),
    (set_l_2, "SET L, 2"),
    (set_mhl_2, "SET [HL], 2"),
    (set_a_2, "SET A, 2"),
    (set_b_3, "SET B, 3"),
    (set_c_3, "SET C, 3"),
    (set_d_3, "SET D, 3"),
    (set_e_3, "SET E, 3"),
    (set_h_3, "SET H, 3"),
    (set_l_3, "SET L, 3"),
    (set_mhl_3, "SET [HL], 3"),
    (set_a_3, "SET A, 3"),
    //Opcodes CB EX
    (set_b_4, "SET B, 4"),
    (set_c_4, "SET C, 4"),
    (set_d_4, "SET D, 4"),
    (set_e_4, "SET E, 4"),
    (set_h_4, "SET H, 4"),
    (set_l_4, "SET L, 4"),
    (set_mhl_4, "SET [HL], 4"),
    (set_a_4, "SET A, 4"),
    (set_b_5, "SET B, 5"),
    (set_c_5, "SET C, 5"),
    (set_d_5, "SET D, 5"),
    (set_e_5, "SET E, 5"),
    (set_h_5, "SET H, 5"),
    (set_l_5, "SET L, 5"),
    (set_mhl_5, "SET [HL], 5"),
    (set_a_5, "SET A, 5"),
    //Opcodes CB FX
    (set_b_6, "SET B, 6"),
    (set_c_6, "SET C, 6"),
    (set_d_6, "SET D, 6"),
    (set_e_6, "SET E, 6"),
    (set_h_6, "SET H, 6"),
    (set_l_6, "SET L, 6"),
    (set_mhl_6, "SET [HL], 6"),
    (set_a_6, "SET A, 6"),
    (set_b_7, "SET B, 7"),
    (set_c_7, "SET C, 7"),
    (set_d_7, "SET D, 7"),
    (set_e_7, "SET E, 7"),
    (set_h_7, "SET H, 7"),
    (set_l_7, "SET L, 7"),
    (set_mhl_7, "SET [HL], 7"),
    (set_a_7, "SET A, 7"),
];

fn swap(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.f.zero = value == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;

    (value << 4) | (value >> 4)
}

fn swap_a(cpu: &mut Cpu) {
    cpu.registers.a = swap(cpu, cpu.registers.a);
}

fn swap_b(cpu: &mut Cpu) {
    cpu.registers.b = swap(cpu, cpu.registers.b);
}

fn swap_c(cpu: &mut Cpu) {
    cpu.registers.c = swap(cpu, cpu.registers.c);
}

fn swap_d(cpu: &mut Cpu) {
    cpu.registers.d = swap(cpu, cpu.registers.d);
}

fn swap_e(cpu: &mut Cpu) {
    cpu.registers.e = swap(cpu, cpu.registers.e);
}

fn swap_h(cpu: &mut Cpu) {
    cpu.registers.h = swap(cpu, cpu.registers.h);
}

fn swap_l(cpu: &mut Cpu) {
    cpu.registers.l = swap(cpu, cpu.registers.l);
}

fn swap_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let rsult = swap(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), rsult);
}


//Function to test bits in a generic register a until l
fn set_bit(cpu: &mut Cpu, bit: u8, register: u8) {
    cpu.registers.f.zero = (register & (1u8 << (bit as usize))) == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
}

fn bit_a_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.a);
}

fn bit_a_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.a);
}

fn bit_a_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.a);
}

fn bit_a_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.a);
}

fn bit_a_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.a);
}

fn bit_a_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.a);
}

fn bit_a_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.a);
}

fn bit_a_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.a);
}


fn bit_b_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.b);
}

fn bit_b_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.b);
}

fn bit_b_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.b);
}

fn bit_b_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.b);
}

fn bit_b_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.b);
}

fn bit_b_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.b);
}

fn bit_b_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.b);
}

fn bit_b_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.b);
}

fn bit_c_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.c);
}

fn bit_c_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.c);
}

fn bit_c_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.c);
}

fn bit_c_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.c);
}

fn bit_c_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.c);
}

fn bit_c_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.c);
}

fn bit_c_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.c);
}

fn bit_c_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.c);
}

fn bit_d_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.d);
}

fn bit_d_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.d);
}

fn bit_d_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.d);
}

fn bit_d_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.d);
}

fn bit_d_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.d);
}

fn bit_d_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.d);
}

fn bit_d_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.d);
}

fn bit_d_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.d);
}

fn bit_e_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.e);
}

fn bit_e_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.e);
}

fn bit_e_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.e);
}

fn bit_e_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.e);
}

fn bit_e_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.e);
}

fn bit_e_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.e);
}

fn bit_e_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.e);
}

fn bit_e_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.e);
}

fn bit_h_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.h);
}

fn bit_h_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.h);
}

fn bit_h_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.h);
}

fn bit_h_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.h);
}

fn bit_h_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.h);
}

fn bit_h_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.h);
}

fn bit_h_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.h);
}

fn bit_h_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.h);
}

fn bit_l_0(cpu: &mut Cpu) {
    set_bit(cpu, 0, cpu.registers.l);
}

fn bit_l_1(cpu: &mut Cpu) {
    set_bit(cpu, 1, cpu.registers.l);
}

fn bit_l_2(cpu: &mut Cpu) {
    set_bit(cpu, 2, cpu.registers.l);
}

fn bit_l_3(cpu: &mut Cpu) {
    set_bit(cpu, 3, cpu.registers.l);
}

fn bit_l_4(cpu: &mut Cpu) {
    set_bit(cpu, 4, cpu.registers.l);
}

fn bit_l_5(cpu: &mut Cpu) {
    set_bit(cpu, 5, cpu.registers.l);
}

fn bit_l_6(cpu: &mut Cpu) {
    set_bit(cpu, 6, cpu.registers.l);
}

fn bit_l_7(cpu: &mut Cpu) {
    set_bit(cpu, 7, cpu.registers.l);
}

//Function to test bits in [HL]
fn set_mbit(cpu: &mut Cpu, bit: u8) {
    let result = cpu.external.ram.read_byte(cpu.registers.hl());
    cpu.registers.f.zero = (result & (1u8 << (bit as usize))) == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = true;
}

fn bit_mhl_0(cpu: &mut Cpu) {
    set_mbit(cpu, 0);
}

fn bit_mhl_1(cpu: &mut Cpu) {
    set_mbit(cpu, 1);
}

fn bit_mhl_2(cpu: &mut Cpu) {
    set_mbit(cpu, 2);
}

fn bit_mhl_3(cpu: &mut Cpu) {
    set_mbit(cpu, 3);
}

fn bit_mhl_4(cpu: &mut Cpu) {
    set_mbit(cpu, 4);
}

fn bit_mhl_5(cpu: &mut Cpu) {
    set_mbit(cpu, 5);
}

fn bit_mhl_6(cpu: &mut Cpu) {
    set_mbit(cpu, 6);
}

fn bit_mhl_7(cpu: &mut Cpu) {
    set_mbit(cpu, 7);
}

//Function to clear one bit in a u8 and set
fn set_res(bit: u8, register: &mut u8) {
    let res = *register & !(1u8 << (bit as usize));
    *register = res;
}

fn res_a_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.a);
}

fn res_a_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.a);
}

fn res_a_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.a);
}

fn res_a_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.a);
}

fn res_a_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.a);
}

fn res_a_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.a);
}

fn res_a_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.a);
}

fn res_a_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.a);
}

fn res_b_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.b);
}

fn res_b_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.b);
}

fn res_b_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.b);
}

fn res_b_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.b);
}

fn res_b_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.b);
}

fn res_b_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.b);
}

fn res_b_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.b);
}

fn res_b_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.b);
}

fn res_c_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.c);
}

fn res_c_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.c);
}

fn res_c_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.c);
}

fn res_c_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.c);
}

fn res_c_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.c);
}

fn res_c_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.c);
}

fn res_c_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.c);
}

fn res_c_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.c);
}

fn res_d_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.d);
}

fn res_d_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.d);
}

fn res_d_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.d);
}

fn res_d_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.d);
}

fn res_d_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.d);
}

fn res_d_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.d);
}

fn res_d_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.d);
}

fn res_d_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.d);
}

fn res_e_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.e);
}

fn res_e_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.e);
}

fn res_e_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.e);
}

fn res_e_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.e);
}

fn res_e_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.e);
}

fn res_e_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.e);
}

fn res_e_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.e);
}

fn res_e_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.e);
}

fn res_h_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.h);
}

fn res_h_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.h);
}

fn res_h_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.h);
}

fn res_h_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.h);
}

fn res_h_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.h);
}

fn res_h_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.h);
}

fn res_h_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.h);
}

fn res_h_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.h);
}

fn res_l_0(cpu: &mut Cpu) {
    set_res(0, &mut cpu.registers.l);
}

fn res_l_1(cpu: &mut Cpu) {
    set_res(1, &mut cpu.registers.l);
}

fn res_l_2(cpu: &mut Cpu) {
    set_res(2, &mut cpu.registers.l);
}

fn res_l_3(cpu: &mut Cpu) {
    set_res(3, &mut cpu.registers.l);
}

fn res_l_4(cpu: &mut Cpu) {
    set_res(4, &mut cpu.registers.l);
}

fn res_l_5(cpu: &mut Cpu) {
    set_res(5, &mut cpu.registers.l);
}

fn res_l_6(cpu: &mut Cpu) {
    set_res(6, &mut cpu.registers.l);
}

fn res_l_7(cpu: &mut Cpu) {
    set_res(7, &mut cpu.registers.l);
}

fn set_mres(cpu: &mut Cpu, bit: u8) {
    let register = cpu.external.ram.read_byte(cpu.registers.hl());
    let res = register & !(1u8 << (bit as usize));
    cpu.external.ram.store_byte(cpu.registers.hl(),res);
}

fn res_mhl_0(cpu: &mut Cpu) {
    set_mres(cpu, 0);
}

fn res_mhl_1(cpu: &mut Cpu) {
    set_mres(cpu, 1);
}

fn res_mhl_2(cpu: &mut Cpu) {
    set_mres(cpu, 2);
}

fn res_mhl_3(cpu: &mut Cpu) {
    set_mres(cpu, 3);
}

fn res_mhl_4(cpu: &mut Cpu) {
    set_mres(cpu, 4);
}

fn res_mhl_5(cpu: &mut Cpu) {
    set_mres(cpu, 5);
}

fn res_mhl_6(cpu: &mut Cpu) {
    set_mres(cpu, 6);
}

fn res_mhl_7(cpu: &mut Cpu) {
    set_mres(cpu, 7);
}

//Set a bit in a register
fn set(register: &mut u8, bit: u8) {
    let result = *register | (1u8 << (bit as usize));
    *register = result;
}

fn set_a_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 0);
}

fn set_a_1(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 1);
}

fn set_a_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 2);
}

fn set_a_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 3);
}

fn set_a_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 4);
}

fn set_a_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 5);
}

fn set_a_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 6);
}

fn set_a_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.a, 7);
}

fn set_b_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 0);
}

fn set_b_1(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 1);
}

fn set_b_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 2);
}

fn set_b_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 3);
}

fn set_b_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 4);
}

fn set_b_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 5);
}

fn set_b_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 6);
}

fn set_b_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.b, 7);
}

fn set_c_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 0);
}

fn set_c_1(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 1);
}

fn set_c_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 2);
}

fn set_c_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 3);
}

fn set_c_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 4);
}

fn set_c_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 5);
}

fn set_c_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 6);
}

fn set_c_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.c, 7);
}

fn set_d_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 0);
}

fn set_d_1(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 1);
}

fn set_d_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 2);
}

fn set_d_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 3);
}

fn set_d_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 4);
}

fn set_d_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 5);
}

fn set_d_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 6);
}

fn set_d_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.d, 7);
}

fn set_e_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 0);
}

fn set_e_1(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 1);
}

fn set_e_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 2);
}

fn set_e_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 3);
}

fn set_e_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 4);
}

fn set_e_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 5);
}

fn set_e_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 6);
}

fn set_e_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.e, 7);
}

fn set_h_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 0);
}

fn set_h_1(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 1);
}

fn set_h_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 2);
}

fn set_h_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 3);
}

fn set_h_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 4);
}

fn set_h_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 5);
}

fn set_h_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 6);
}

fn set_h_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.h, 7);
}

fn set_l_0(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 0);
}

fn set_l_1(cpu: &mut Cpu) {
    set_mhl(cpu, 0);
}

fn set_l_2(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 2);
}

fn set_l_3(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 3);
}

fn set_l_4(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 4);
}

fn set_l_5(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 5);
}

fn set_l_6(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 6);
}

fn set_l_7(cpu: &mut Cpu) {
    set(&mut cpu.registers.l, 7);
}

fn set_mhl(cpu: &mut Cpu, bit: u8) {
    let register = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = register | (1u8 << (bit as usize));
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn set_mhl_0(cpu: &mut Cpu) {
    set_mhl(cpu, 0);
}

fn set_mhl_1(cpu: &mut Cpu) {
    set_mhl(cpu, 1);
}

fn set_mhl_2(cpu: &mut Cpu) {
    set_mhl(cpu, 2);
}

fn set_mhl_3(cpu: &mut Cpu) {
    set_mhl(cpu, 3);
}

fn set_mhl_4(cpu: &mut Cpu) {
    set_mhl(cpu, 4);
}

fn set_mhl_5(cpu: &mut Cpu) {
    set_mhl(cpu, 5);
}

fn set_mhl_6(cpu: &mut Cpu) {
    set_mhl(cpu, 6);
}

fn set_mhl_7(cpu: &mut Cpu) {
    set_mhl(cpu, 7);
}

fn srl(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.f.carry = value & 1 != 0;

    let result = value >> 1;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn srl_a(cpu: &mut Cpu) {
    cpu.registers.a = srl(cpu, cpu.registers.a);
}

fn srl_b(cpu: &mut Cpu) {
    cpu.registers.b = srl(cpu, cpu.registers.b);
}

fn srl_c(cpu: &mut Cpu) {
    cpu.registers.c = srl(cpu, cpu.registers.c);
}

fn srl_d(cpu: &mut Cpu) {
    cpu.registers.d = srl(cpu, cpu.registers.d);
}

fn srl_e(cpu: &mut Cpu) {
    cpu.registers.e = srl(cpu, cpu.registers.e);
}

fn srl_h(cpu: &mut Cpu) {
    cpu.registers.h = srl(cpu, cpu.registers.h);
}

fn srl_l(cpu: &mut Cpu) {
    cpu.registers.l = srl(cpu, cpu.registers.l);
}

fn srl_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = srl(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn sla(cpu: &mut Cpu, value: u8) -> u8 {
    cpu.registers.f.carry = value & 0x80 != 0;

    let result = value << 1;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn sla_a(cpu: &mut Cpu) {
    cpu.registers.a = sla(cpu, cpu.registers.a);
}

fn sla_b(cpu: &mut Cpu) {
    cpu.registers.b = sla(cpu, cpu.registers.b);
}

fn sla_c(cpu: &mut Cpu) {
    cpu.registers.c = sla(cpu, cpu.registers.c);
}

fn sla_d(cpu: &mut Cpu) {
    cpu.registers.d = sla(cpu, cpu.registers.d);
}

fn sla_e(cpu: &mut Cpu) {
    cpu.registers.e = sla(cpu, cpu.registers.e);
}

fn sla_h(cpu: &mut Cpu) {
    cpu.registers.h = sla(cpu, cpu.registers.h);
}

fn sla_l(cpu: &mut Cpu) {
    cpu.registers.l = sla(cpu, cpu.registers.l);
}

fn sla_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = sla(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn sra(cpu: &mut Cpu, value: u8) -> u8{
    cpu.registers.f.carry = value & 1 != 0;

    let result = (value >> 1) | (value & 0x80);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn sra_a(cpu: &mut Cpu) {
    cpu.registers.a = sra(cpu, cpu.registers.a);
}

fn sra_b(cpu: &mut Cpu) {
    cpu.registers.b = sra(cpu, cpu.registers.b);
}

fn sra_c(cpu: &mut Cpu) {
    cpu.registers.c = sra(cpu, cpu.registers.c);
}

fn sra_d(cpu: &mut Cpu) {
    cpu.registers.d = sra(cpu, cpu.registers.d);
}

fn sra_e(cpu: &mut Cpu) {
    cpu.registers.e = sra(cpu, cpu.registers.e);
}

fn sra_h(cpu: &mut Cpu) {
    cpu.registers.h = sra(cpu, cpu.registers.h);
}

fn sra_l(cpu: &mut Cpu) {
    cpu.registers.l = sra(cpu, cpu.registers.l);
}

fn sra_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = sra(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn rlc(cpu: &mut Cpu, value: u8) -> u8{
    cpu.registers.f.carry = value & 0x80 != 0;

    let result = (value << 1) | (value >> 7);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn rlc_a(cpu: &mut Cpu) {
    cpu.registers.a = rlc(cpu, cpu.registers.a);
}

fn rlc_b(cpu: &mut Cpu) {
    cpu.registers.b = rlc(cpu, cpu.registers.b);
}

fn rlc_c(cpu: &mut Cpu) {
    cpu.registers.c = rlc(cpu, cpu.registers.c);
}

fn rlc_d(cpu: &mut Cpu) {
    cpu.registers.d = rlc(cpu, cpu.registers.d);
}

fn rlc_e(cpu: &mut Cpu) {
    cpu.registers.e = rlc(cpu, cpu.registers.e);
}

fn rlc_h(cpu: &mut Cpu) {
    cpu.registers.h = rlc(cpu, cpu.registers.h);
}

fn rlc_l(cpu: &mut Cpu) {
    cpu.registers.l = rlc(cpu, cpu.registers.l);
}

fn rlc_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = rlc(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn rrc(cpu: &mut Cpu, value: u8) -> u8{
    cpu.registers.f.carry = value & 1 != 0;

    let result = (value >> 1) | (value << 7);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn rrc_a(cpu: &mut Cpu) {
    cpu.registers.a = rrc(cpu, cpu.registers.a);
}

fn rrc_b(cpu: &mut Cpu) {
    cpu.registers.b = rrc(cpu, cpu.registers.b);
}

fn rrc_c(cpu: &mut Cpu) {
    cpu.registers.c = rrc(cpu, cpu.registers.c);
}

fn rrc_d(cpu: &mut Cpu) {
    cpu.registers.d = rrc(cpu, cpu.registers.d);
}

fn rrc_e(cpu: &mut Cpu) {
    cpu.registers.e = rrc(cpu, cpu.registers.e);
}

fn rrc_h(cpu: &mut Cpu) {
    cpu.registers.h = rrc(cpu, cpu.registers.h);
}

fn rrc_l(cpu: &mut Cpu) {
    cpu.registers.l = rrc(cpu, cpu.registers.l);
}

fn rrc_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = rrc(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn rl(cpu: &mut Cpu, value: u8) -> u8{
    let old_carry = cpu.registers.f.carry as u8;

    cpu.registers.f.carry = value & 0x80 != 0;

    let result = (value << 1) | old_carry;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn rl_a(cpu: &mut Cpu) {
    cpu.registers.a = rl(cpu, cpu.registers.a);
}

fn rl_b(cpu: &mut Cpu) {
    cpu.registers.b = rl(cpu, cpu.registers.b);
}

fn rl_c(cpu: &mut Cpu) {
    cpu.registers.c = rl(cpu, cpu.registers.c);
}

fn rl_d(cpu: &mut Cpu) {
    cpu.registers.d = rl(cpu, cpu.registers.d);
}

fn rl_e(cpu: &mut Cpu) {
    cpu.registers.e = rl(cpu, cpu.registers.e);
}

fn rl_h(cpu: &mut Cpu) {
    cpu.registers.h = rl(cpu, cpu.registers.h);
}

fn rl_l(cpu: &mut Cpu) {
    cpu.registers.l = rl(cpu, cpu.registers.l);
}

fn rl_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = rl(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}

fn rr(cpu: &mut Cpu, value: u8) -> u8{
    let old_carry = cpu.registers.f.carry as u8;

    cpu.registers.f.carry = value & 0x1 != 0;

    let result = (value >> 1) | (old_carry << 7);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;

    result
}

fn rr_a(cpu: &mut Cpu) {
    cpu.registers.a = rr(cpu, cpu.registers.a);
}

fn rr_b(cpu: &mut Cpu) {
    cpu.registers.b = rr(cpu, cpu.registers.b);
}

fn rr_c(cpu: &mut Cpu) {
    cpu.registers.c = rr(cpu, cpu.registers.c);
}

fn rr_d(cpu: &mut Cpu) {
    cpu.registers.d = rr(cpu, cpu.registers.d);
}

fn rr_e(cpu: &mut Cpu) {
    cpu.registers.e = rr(cpu, cpu.registers.e);
}

fn rr_h(cpu: &mut Cpu) {
    cpu.registers.h = rr(cpu, cpu.registers.h);
}

fn rr_l(cpu: &mut Cpu) {
    cpu.registers.l = rr(cpu, cpu.registers.l);
}

fn rr_mhl(cpu: &mut Cpu) {
    let address = cpu.external.ram.read_byte(cpu.registers.hl());
    let result = rr(cpu, address);
    cpu.external.ram.store_byte(cpu.registers.hl(), result);
}