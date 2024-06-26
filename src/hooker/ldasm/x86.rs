use super::read_byte;

// const OP_X86_NONE: u32          = 0x00;
const OP_X86_DATA_I8: u32       = 0x01;
// const OP_X86_DATA_I16: u32      = 0x02;
// const OP_X86_DATA_I32: u32      = 0x04;
const OP_X86_MODRM: u32         = 0x08;
const OP_X86_DATA_PRE66_67: u32 = 0x10;
const OP_X86_PREFIX: u32        = 0x20;
// const OP_X86_REL32: u32         = 0x40;
// const OP_X86_REL8: u32          = 0x80;
const OP_X86_EXTENDED: u32      = 0x100;

const PACKED_TABLE: [u8; 256] = [
    0x80, 0x84, 0x80, 0x84, 0x80, 0x84, 0x80, 0x84, 0x80, 0x88, 0x80, 0x88, 0x80, 0x88, 0x80, 0x88,
    0x8c, 0x8b, 0x8b, 0x8b, 0x8b, 0x8b, 0x8b, 0x8b, 0x90, 0x94, 0x98, 0x8b, 0x9c, 0x9c, 0x9c, 0x9c,
    0xa0, 0x80, 0x80, 0x80, 0x8b, 0x8b, 0xa4, 0x8b, 0xa8, 0x8b, 0x84, 0x8b, 0xac, 0xac, 0xa8, 0xa8,
    0xb0, 0xb4, 0xb8, 0xbc, 0x80, 0xc0, 0x80, 0x80, 0x9c, 0xac, 0xc4, 0x8b, 0xc8, 0x90, 0x8b, 0x90,
    0x80, 0x8b, 0x8b, 0xcc, 0x80, 0x80, 0xd0, 0x8b, 0x80, 0xd4, 0x80, 0x80, 0x8b, 0x8b, 0x8b, 0x8b,
    0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xd8, 0xdc, 0x8b, 0x80,
    0xe0, 0xe0, 0xe0, 0xe0, 0x80, 0x80, 0x80, 0x80, 0x8f, 0xcf, 0x8f, 0xdb, 0x80, 0x80, 0xe4, 0x80,
    0xe8, 0xd9, 0x8b, 0x8b, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xdc,
    0x08, 0x08, 0x08, 0x08, 0x01, 0x10, 0x00, 0x00, 0x01, 0x10, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x08, 0x20, 0x20, 0x20, 0x20, 0x10, 0x18, 0x01, 0x09, 0x81, 0x81, 0x81, 0x81,
    0x09, 0x18, 0x09, 0x09, 0x00, 0x00, 0x12, 0x00, 0x10, 0x10, 0x10, 0x10, 0x01, 0x01, 0x01, 0x01,
    0x09, 0x09, 0x02, 0x00, 0x08, 0x08, 0x09, 0x18, 0x03, 0x00, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x01, 0x01, 0x00, 0x00, 0x50, 0x50, 0x12, 0x81, 0x20, 0x00, 0x20, 0x20, 0x00, 0x08, 0x00, 0x09,
    0x08, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x09, 0x09, 0x09, 0x09, 0x08, 0x08, 0x08, 0x00,
    0x50, 0x50, 0x50, 0x50, 0x00, 0x00, 0x09, 0x08, 0x08, 0x08, 0x09, 0x08, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

fn get_opcode_flags_32(opcode: u32) -> u32 {
    PACKED_TABLE[(PACKED_TABLE[(opcode / 4) as usize] + (opcode % 4) as u8) as usize] as u32
}

pub fn get_opcode_size_32(buffer: &[u8]) -> usize {
    let mut code = buffer.as_ptr() as usize;
    // let code_end = code + buffer.len();

    let mut op1: u32;
    let op2: u32;
    let mut pfx66: u32 = 0;
    let mut pfx67: u32 = 0;
    let mut osize: usize = 0;
    let mut oflen: usize = 0;

    op1 = read_byte(code);

    while (get_opcode_flags_32(op1) & OP_X86_PREFIX) != 0 {
        match op1 {
            0x66 => pfx66 = 1,
            0x67 => pfx67 = 1,
            _ => {},
        }

        code += 1;
        osize += 1;
        op1 = read_byte(code);
    }

    code += 1;
    osize += 1;

    if op1 == 0x0F {
        op2 = read_byte(code) | OP_X86_EXTENDED;
        code += 1;
        osize += 1;

    } else {
        op2 = op1;

        /* pfx66 = pfx67 for opcodes A0 - A3 */
        if op2 >= 0xA0 && op2 <= 0xA3 {
            pfx66 = pfx67;
        }
    }

    let mut flags = get_opcode_flags_32(op2);

    let i_mod: u32;
    let i_rm: u32;
    let i_reg: u32;

    if flags & OP_X86_MODRM != 0 {
        let v = read_byte(code);
        code += 1;
        osize += 1;

        i_mod = v >> 6;
        i_reg = (v & 0x38) >> 3;
        i_rm = v & 7;

        match op1 {
            0xF6 if i_reg == 0 => flags |= OP_X86_DATA_I8,
            0xF7 if i_reg == 0 => flags |= OP_X86_DATA_PRE66_67,
            _ => {},
        }

        match i_mod {
            0 => {
                if pfx67 != 0 {
                    if i_rm == 6 {
                        oflen = 2;
                    }
                } else {
                    if i_rm == 5 {
                        oflen = 4;
                    }
                }
            },

            1 => {
                oflen = 1;
            },

            2 => {
                oflen = if pfx67 != 0 { 2 } else { 4 };
            },

            _ => {},
        }

        if pfx67 == 0  && i_rm == 4 && i_mod != 3 {
            if (read_byte(code) & 7) == 5 && i_mod != 1 {
                oflen = 4;
            }

            oflen += 1;
        }

        osize += oflen;
    }

    if flags & OP_X86_DATA_PRE66_67 != 0 {
        osize += 4 - ((pfx66 as usize) << 1);
    }

    osize + ((flags & 7) as usize)
}
