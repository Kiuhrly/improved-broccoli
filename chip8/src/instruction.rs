use core::fmt;

/// Represents valid CHIP-8 instructions.
///
/// The documentation comments for fields of this enum are excerpts from
/// [CHIP‐8 Instruction Set](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set)
/// by Matthew Mikolay which is licensed under
/// [Creative Commons Attribution Share Alike 4.0](https://creativecommons.org/licenses/by-sa/4.0/).
#[derive(Debug)]
pub enum Inst {
    /// `0NNN`: Execute machine language subroutine at address `NNN`
    Exe { nnn: u16 },
    /// `00E0`: Clear the screen
    Clear,
    /// `00EE`: Return from a subroutine
    Return,
    /// `1NNN`: Jump to a subroutine
    Jump { nnn: u16 },
    /// `2NNN`: Execute subroutine starting at address `NNN`
    Call { nnn: u16 },
    /// `3XNN`: Skip the following instruction if the value of register `VX`
    /// equals `NN`
    SkipEqualValue { vx: u8, nn: u8 },
    /// `4XNN`: Skip the following instruction if the value of register `VX` is
    /// not equal to `NN`
    SkipNotEqualValue { vx: u8, nn: u8 },
    /// `5XY0`: Skip the following instruction if the value of register `VX` is
    /// equal to the value of register `VY`
    SkipEqualRegister { vx: u8, vy: u8 },
    /// `6XNN`: Store number `NN` in register `VX`
    LoadValue { vx: u8, nn: u8 },
    /// `7XNN`: Add the value `NN` to register `VX`
    AddValue { vx: u8, nn: u8 },
    /// `8XY0`: Store the value of register `VY` in register `VX`
    LoadRegister { vx: u8, vy: u8 },
    /// `8XY1`: Set `VX` to `VX` OR `VY`
    Or { vx: u8, vy: u8 },
    /// `8XY2`: Set `VX` to `VX` AND `VY`
    And { vx: u8, vy: u8 },
    /// `8XY3` Set `VX` to `VX` XOR `VY`
    Xor { vx: u8, vy: u8 },
    /// `8XY4`: Add the value of register `VY` to register `VX`.
    /// Set `VF` to `01` if a carry occurs.
    /// Set `VF` to `00` if a carry does not occur
    AddRegister { vx: u8, vy: u8 },
    /// `8XY5`: Subtract the value of register `VY` from register `VX`.
    /// Set `VF` to `00` if a borrow occurs.
    /// Set `VF` to `01` if a borrow does not occur.
    SubRegisterXY { vx: u8, vy: u8 },
    /// `8XY6`: Store the value of register `VY` shifted right one bit in register
    /// `VX`.
    /// Set register `VF` to the least significant bit prior to the shift.
    /// `VY` is unchanged.
    ShiftRight { vx: u8, vy: u8 },
    /// `8XY7`: Set register `VX` to the value of `VY` minus `VX`.
    /// Set `VF` to `00` if a borrow occurs.
    /// Set `VF` to `01` if a borrow does not occur
    SubRegisterYX { vx: u8, vy: u8 },
    /// `8XYE`: Store the value of register `VY` shifted left one bit in register
    /// `VX`.
    /// Set register `VF` to the most significant bit prior to the shift.
    /// `VY` is unchanged.
    ShiftLeft { vx: u8, vy: u8 },
    /// `9XY0`: Skip the following instruction if the value of register `VX` is
    /// not equal to the value of register `VY`
    SkipNotEqualRegister { vx: u8, vy: u8 },
    /// `ANNN`: Store memory address `NNN` in register `I`
    LoadIntoI { nnn: u16 },
    /// `BNNN`: Jump to address `NNN + V0`
    JumpAdd { nnn: u16 },
    /// `CXNN`: Set `VX` to a random number with a mask of `NN`
    LoadRandom { vx: u8, nn: u8 },
    /// `DXYN`: Draw a sprite at position `VX`, `VY` with `N` bytes of sprite data
    /// starting at the address stored in `I`.
    /// Set `VF` to `01` if any set pixels are changed to unset, and `00` otherwise.
    DrawSprite { vx: u8, vy: u8, n: u8 },
    /// `EX9E`: Skip the following instruction if the key corresponding to the hex
    /// value currently stored in register `VX` is pressed
    SkipIfKey { vx: u8 },
    /// `EXA1`: Skip the following instruction if the key corresponding to the hex
    /// value currently stored in register `VX` is not pressed.
    SkipIfNotKey { vx: u8 },
    /// `FX07`: Store the current value of the delay timer in register `VX`
    LoadDelay { vx: u8 },
    /// `FX0A`: Wait for a keypress and store the result in register `VX`
    WaitForKey { vx: u8 },
    /// `FX15`: Set the delay timer to the value of register `VX`
    SetDelay { vx: u8 },
    /// `FX18`: Set the sound timer to the value of register `VX`
    SetSound { vx: u8 },
    /// `FX1E`: Add the value stored in register `VX` to register `I`
    AddToI { vx: u8 },
    /// `FX29`: Set `I` to the memory address of the sprite data corresponding to
    /// the hexadecimal digit stored in register `VX`
    LoadDigitSpriteAddrIntoI { vx: u8 },
    /// `FX33`: Store the [binary-coded decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal)
    /// equivalent of the value stored in register VX at addresses `I`, `I + 1`,
    /// and `I + 2`
    StoreBCD { vx: u8 },
    /// `FX55`: Store the values of registers `V0` to `VX` inclusive in memory
    /// starting at address `I`.
    /// `I` is set to `I + X + 1` after operation.
    StoreRegisters { vx: u8 },
    /// `FX65`: Fill registers `V0` to `VX` inclusive with the values stored in
    /// memory starting at address `I`.
    /// `I` is set to `I + X + 1` after operation.
    LoadRegisters { vx: u8 },
}

/// Decode a u16 into an Instruction. Returns an error when attempting to
/// decode an invalid instruction.
pub fn decode(inst: u16) -> Result<Inst, DecodeError> {
    // common values decoded from instructions
    let vx = ((inst & 0x0f00) >> 8) as u8;
    let vy = ((inst & 0x00f0) >> 4) as u8;
    let n = (inst & 0x000f) as u8;
    let nn = (inst & 0x00ff) as u8;
    let nnn = inst & 0x0fff;

    match inst & 0xf000 {
        0x0000 => match inst {
            0x00e0 => Ok(Inst::Clear),
            0x00ee => Ok(Inst::Return),
            _ => Ok(Inst::Exe { nnn }),
        },
        0x1000 => Ok(Inst::Jump { nnn }),
        0x2000 => Ok(Inst::Call { nnn }),
        0x3000 => Ok(Inst::SkipEqualValue { vx, nn }),
        0x4000 => Ok(Inst::SkipNotEqualValue { vx, nn }),
        0x5000 => {
            if inst & 0x000f == 0 {
                Ok(Inst::SkipEqualRegister { vx, vy })
            } else {
                Err(DecodeError::UnknownInstruction { inst })
            }
        }
        0x6000 => Ok(Inst::LoadValue { vx, nn }),
        0x7000 => Ok(Inst::AddValue { vx, nn }),
        0x8000 => match inst & 0x000f {
            0x0000 => Ok(Inst::LoadRegister { vx, vy }),
            0x0001 => Ok(Inst::Or { vx, vy }),
            0x0002 => Ok(Inst::And { vx, vy }),
            0x0003 => Ok(Inst::Xor { vx, vy }),
            0x0004 => Ok(Inst::AddRegister { vx, vy }),
            0x0005 => Ok(Inst::SubRegisterXY { vx, vy }),
            0x0006 => Ok(Inst::ShiftRight { vx, vy }),
            0x0007 => Ok(Inst::SubRegisterYX { vx, vy }),
            0x000E => Ok(Inst::ShiftLeft { vx, vy }),
            _ => Err(DecodeError::UnknownInstruction { inst }),
        },
        0x9000 => Ok(Inst::SkipNotEqualRegister { vx, vy }),
        0xa000 => Ok(Inst::LoadIntoI { nnn }),
        0xb000 => Ok(Inst::JumpAdd { nnn }),
        0xc000 => Ok(Inst::LoadRandom { vx, nn }),
        0xd000 => Ok(Inst::DrawSprite { vx, vy, n }),
        0xe000 => match inst & 0x00FF {
            0x009E => Ok(Inst::SkipIfKey { vx }),
            0x00A1 => Ok(Inst::SkipIfNotKey { vx }),
            _ => Err(DecodeError::UnknownInstruction { inst }),
        },
        0xf000 => match inst & 0x00ff {
            0x0007 => Ok(Inst::LoadDelay { vx }),
            0x000A => Ok(Inst::WaitForKey { vx }),
            0x0015 => Ok(Inst::SetDelay { vx }),
            0x0018 => Ok(Inst::SetSound { vx }),
            0x001E => Ok(Inst::AddToI { vx }),
            0x0029 => Ok(Inst::LoadDigitSpriteAddrIntoI { vx }),
            0x0033 => Ok(Inst::StoreBCD { vx }),
            0x0055 => Ok(Inst::StoreRegisters { vx }),
            0x0065 => Ok(Inst::LoadRegisters { vx }),
            _ => Err(DecodeError::UnknownInstruction { inst }),
        },
        _ => unreachable!(),
    }
}

/// Error type for `decode()`.
///
/// Note that this doesn't implement the `Error` trait, for reasons specified
/// in the readme section titled "Why not implement the `Error` trait on error
/// types?".
#[derive(Debug)]
pub enum DecodeError {
    UnknownInstruction { inst: u16 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::UnknownInstruction { inst: inst_bytes } => {
                write!(f, "unknown instruction: 0x{:04x}", inst_bytes)
            }
        }
    }
}
