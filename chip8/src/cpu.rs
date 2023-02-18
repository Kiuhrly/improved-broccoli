use crate::{
    instruction::{self, decode, Inst},
    memory::{self, Chip8Memory, CHIP8_MEMORY_SIZE_BYTES, PROGRAM_OFFSET_BYTES},
    screen::Chip8Screen,
};
use core::fmt;

const STACK_SIZE: usize = 12;

pub struct Chip8 {
    memory: Chip8Memory,
    screen: Chip8Screen,
    /// general purpose registers
    v_reg: [u8; 16],
    /// memory address register
    i_reg: u16,

    stack: [u16; STACK_SIZE],
    /// stack pointer
    stack_ptr: u8,
    /// program counter
    pc: u16,

    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    #[must_use]
    pub fn new(program: &[u8]) -> Chip8 {
        Chip8 {
            memory: Chip8Memory::new(program),
            screen: Chip8Screen::new(),
            v_reg: [0; 16],
            i_reg: 0,
            stack: [0; STACK_SIZE],
            stack_ptr: 0,
            pc: PROGRAM_OFFSET_BYTES as u16,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    /// Advance the CHIP-8 by one cycle using the inputs given.
    ///
    /// Don't forget to call `update_timers()` 60 times per realtime second.
    pub fn cycle(
        &mut self,
        keyboard_state: &[bool; 16],
        previous_keyboard_state: &[bool; 16],
    ) -> Result<(), CycleError> {
        // Get instruction at program counter
        let instruction_bytes = self.get_instruction();
        let instruction = match decode(instruction_bytes) {
            Ok(inst) => inst,
            Err(err) => return Err(CycleError::DecodeError(err)),
        };
        match self.execute_instruction(instruction, keyboard_state, previous_keyboard_state) {
            Err(err) => Err(CycleError::ExecuteError(err)),
            Ok(_) => Ok(()),
        }
    }

    /// Update the delay timer and sound timer. This should be called 60 times
    /// per realtime second
    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// Whether the CHIP-8 speaker is playing
    pub fn is_sound_playing(&self) -> bool {
        self.sound_timer >= 2
    }

    pub fn get_screen(&self) -> &Chip8Screen {
        &self.screen
    }
}

impl Chip8 {
    #[must_use]
    fn get_instruction(&self) -> u16 {
        let pc = self.pc as usize;
        ((self.memory.get(pc) as u16) << 8) | (self.memory.get(pc + 1) as u16)
    }

    fn execute_instruction(
        &mut self,
        instruction: Inst,
        keyboard_state: &[bool; 16],
        previous_keyboard_state: &[bool; 16],
    ) -> Result<(), ExecuteError> {
        let mut increment_pc = true;
        let mut skip_next_instruction = false;
        match instruction {
            Inst::Exe { nnn } => return Err(ExecuteError::UnknownMachineSubroutine { nnn }),
            Inst::Clear => self.screen.clear(),
            Inst::Return => {
                if self.stack_ptr == 0 {
                    return Err(ExecuteError::EmptyStackReturn);
                }

                self.pc = self.stack[self.stack_ptr as usize - 1];
                self.stack_ptr -= 1;
            }
            Inst::Jump { nnn } => {
                self.pc = nnn;
                increment_pc = false;
            }
            Inst::Call { nnn } => {
                increment_pc = false;
                // TODO stack overflow
                self.stack[self.stack_ptr as usize] = self.pc;
                self.stack_ptr += 1;
                self.pc = nnn;
            }
            Inst::SkipEqualValue { vx, nn } => {
                skip_next_instruction = self.v_reg[vx as usize] == nn
            }
            Inst::SkipNotEqualValue { vx, nn } => {
                skip_next_instruction = self.v_reg[vx as usize] != nn
            }
            Inst::SkipEqualRegister { vx, vy } => {
                skip_next_instruction = self.v_reg[vx as usize] == self.v_reg[vy as usize]
            }
            Inst::LoadValue { vx, nn } => self.v_reg[vx as usize] = nn,
            Inst::AddValue { vx, nn } => {
                self.v_reg[vx as usize] = self.v_reg[vx as usize].wrapping_add(nn)
            }
            Inst::LoadRegister { vx, vy } => self.v_reg[vx as usize] = self.v_reg[vy as usize],
            Inst::Or { vx, vy } => self.v_reg[vx as usize] |= self.v_reg[vy as usize],
            Inst::And { vx, vy } => self.v_reg[vx as usize] &= self.v_reg[vy as usize],
            Inst::Xor { vx, vy } => self.v_reg[vx as usize] ^= self.v_reg[vy as usize],
            Inst::AddRegister { vx, vy } => {
                let vx = vx as usize;
                let vy = vy as usize;
                self.v_reg[vx] = if let Some(result) = self.v_reg[vx].checked_add(self.v_reg[vy]) {
                    self.v_reg[0xf] = 0;
                    result
                } else {
                    let result = self.v_reg[vx].wrapping_add(self.v_reg[vy]);
                    self.v_reg[0xf] = 1;
                    result
                };
            }
            Inst::SubRegisterXY { vx, vy } => {
                let vx = vx as usize;
                let vy = vy as usize;
                self.v_reg[vx] = if let Some(result) = self.v_reg[vx].checked_sub(self.v_reg[vy]) {
                    self.v_reg[0xf] = 1;
                    result
                } else {
                    let result = self.v_reg[vx].wrapping_sub(self.v_reg[vy]);
                    self.v_reg[0xf] = 0;
                    result
                };
            }
            Inst::ShiftRight { vx, vy } => {
                let flag = self.v_reg[vy as usize] & 0b00000001;
                self.v_reg[vx as usize] = self.v_reg[vy as usize] >> 1;
                self.v_reg[0xf] = flag;
            }
            Inst::SubRegisterYX { vx, vy } => {
                let vx = vx as usize;
                let vy = vy as usize;
                self.v_reg[vx] = if let Some(result) = self.v_reg[vy].checked_sub(self.v_reg[vx]) {
                    self.v_reg[0xf] = 1;
                    result
                } else {
                    let result = self.v_reg[vy].wrapping_sub(self.v_reg[vx]);
                    self.v_reg[0xf] = 0;
                    result
                };
            }
            Inst::ShiftLeft { vx, vy } => {
                let flag = (self.v_reg[vy as usize] & 0b10000000) >> 7;
                self.v_reg[vx as usize] = self.v_reg[vy as usize] << 1;
                self.v_reg[0xf] = flag;
            }
            Inst::SkipNotEqualRegister { vx, vy } => {
                skip_next_instruction = self.v_reg[vx as usize] != self.v_reg[vy as usize]
            }
            Inst::LoadIntoI { nnn } => self.i_reg = nnn,
            Inst::JumpAdd { nnn } => {
                // TODO: bounds check
                self.pc = nnn + (self.v_reg[0] as u16);
                increment_pc = false;
            }
            Inst::LoadRandom { vx, nn } => {
                // return Err(ExecuteError::UnimplementedInstruction { inst: instruction })
                // TODO implement this
                self.v_reg[vx as usize] = 123 & nn;
            }
            Inst::DrawSprite { vx, vy, n } => {
                // TODO: find out what the correct behavior is here
                if self.i_reg as usize + n as usize > CHIP8_MEMORY_SIZE_BYTES {
                    return Err(ExecuteError::SpriteMemoryOverflow {
                        index: self.i_reg,
                        len: n,
                    });
                }

                let sprite = self.memory.get_bytes(self.i_reg as usize, n as usize);
                self.v_reg[0xf] = self.screen.draw_sprite(
                    self.v_reg[vx as usize],
                    self.v_reg[vy as usize],
                    sprite,
                ) as u8
            }
            Inst::SkipIfKey { vx } => {
                skip_next_instruction = keyboard_state[self.v_reg[vx as usize] as usize]
            }
            Inst::SkipIfNotKey { vx } => {
                skip_next_instruction = !keyboard_state[self.v_reg[vx as usize] as usize]
            }
            Inst::LoadDelay { vx } => self.v_reg[vx as usize] = self.delay_timer,
            Inst::WaitForKey { vx } => {
                increment_pc = false;
                for i in 0..16 {
                    if previous_keyboard_state[i] && !keyboard_state[i] {
                        self.v_reg[vx as usize] = i as u8;
                        increment_pc = true;
                        break;
                    }
                }
            }
            Inst::SetDelay { vx } => self.delay_timer = self.v_reg[vx as usize],
            Inst::SetSound { vx } => self.sound_timer = self.v_reg[vx as usize],
            Inst::AddToI { vx } => {
                // TODO: bounds check/wrapping?
                self.i_reg += self.v_reg[vx as usize] as u16
            }
            Inst::LoadDigitSpriteAddrIntoI { vx } => {
                self.i_reg = memory::SPRITES_OFFSET_BYTES as u16 + vx as u16;
            }
            Inst::StoreBCD { vx } => {
                let value = self.v_reg[vx as usize];
                let ones = value % 10;
                let tens = (value / 10) % 10;
                let hundreds = (value / 100) % 10;
                self.memory.set(self.i_reg as usize, hundreds);
                self.memory.set(self.i_reg as usize + 1, tens);
                self.memory.set(self.i_reg as usize + 2, ones);
            }
            Inst::StoreRegisters { vx } => {
                for i in 0..=vx {
                    self.memory
                        .set(self.i_reg as usize + i as usize, self.v_reg[i as usize]);
                }
                self.i_reg += vx as u16 + 1;
            }
            Inst::LoadRegisters { vx } => {
                for i in 0..=vx {
                    self.v_reg[i as usize] = self.memory.get(self.i_reg as usize + i as usize);
                }
                self.i_reg += vx as u16 + 1;
            }
        };
        if increment_pc {
            self.pc += 2;
        }
        if skip_next_instruction {
            self.pc += 2;
        }
        Ok(())
    }
}

/// Error type for `execute_instruction()`.
///
/// Note that this doesn't implement the `Error` trait, for reasons specified
/// in the readme section titled "Why not implement the `Error` trait on error
/// types?".
#[derive(Debug)]
pub enum ExecuteError {
    /// This instruction is valid but not implemented
    UnimplementedInstruction { inst: Inst },
    /// A 0NNN instruction has attempted to call into an unknown machine code
    /// subroutine
    UnknownMachineSubroutine { nnn: u16 },
    /// Attempted to `Return` when the stack was empty
    EmptyStackReturn,

    /// A `DrawSprite` instruction attempted to read bytes beyond the end of memory
    SpriteMemoryOverflow { index: u16, len: u8 },
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecuteError::UnimplementedInstruction { inst } => {
                write!(f, "unimplemented instruction: {:?}", inst)
            }
            ExecuteError::UnknownMachineSubroutine { nnn } => {
                write!(f, "unknown machine code subroutine: 0x{:03x}", nnn)
            }
            ExecuteError::EmptyStackReturn => {
                write!(
                    f,
                    "attempted to return from a subroutine when the stack is empty"
                )
            }
            ExecuteError::SpriteMemoryOverflow { index, len } => {
                write!(
                    f,
                    "a draw sprite instruction attempted to read data beyond the end of memory at index {index} with length {len}"
                )
            }
        }
    }
}

/// Error type for `cycle()`.
///
/// Note that this doesn't implement the `Error` trait, for reasons specified
/// in the readme section titled "Why not implement the `Error` trait on error
/// types?".
#[derive(Debug)]
pub enum CycleError {
    DecodeError(instruction::DecodeError),
    ExecuteError(ExecuteError),
}

impl fmt::Display for CycleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CycleError::DecodeError(inner) => inner.fmt(f),
            CycleError::ExecuteError(inner) => inner.fmt(f),
        }
    }
}
