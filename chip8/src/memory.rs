/// The amount of memory available to the CHIP-8
pub const CHIP8_MEMORY_SIZE_BYTES: usize = 4096;

/// The offset from the start of memory of the start of the default hex digit
/// sprites
pub const SPRITES_OFFSET_BYTES: usize = 0x0;

// See here for more info:
// devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.4
const DEFAULT_SPRITES: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// The offset from the start of memory that the program bytes should be loaded
pub const PROGRAM_OFFSET_BYTES: usize = 0x200;

/// Represents the memory (RAM) of the CHIP-8
pub struct Chip8Memory([u8; CHIP8_MEMORY_SIZE_BYTES]);

impl Chip8Memory {
    /// Create a CHIP-8 memory loaded with the default hex digit sprites and
    /// the given program
    pub fn new(program: &[u8]) -> Chip8Memory {
        let mut memory = Self([0; CHIP8_MEMORY_SIZE_BYTES]);
        memory.load_bytes(SPRITES_OFFSET_BYTES, &DEFAULT_SPRITES);
        memory.load_bytes(PROGRAM_OFFSET_BYTES, program);
        memory
    }
    
    pub fn get(&self, index: usize) -> u8 {
        *self.0.get(index).unwrap()
    }
    
    pub fn set(&mut self, index: usize, value: u8) {
        *self.0.get_mut(index).unwrap() = value;
    }
    
    pub fn get_bytes(&self, index: usize, len: usize) -> &[u8] { 
        &self.0[index..index + len]
    }
}

impl Chip8Memory {
    fn load_bytes(
        &mut self,
        offset: usize,
        bytes: &[u8],
    ) {
        if offset >= CHIP8_MEMORY_SIZE_BYTES {
            panic!("offset is greater than memory size")
        }

        if bytes.len() > CHIP8_MEMORY_SIZE_BYTES - offset {
            panic!("program is too long");
        }

        for (i, byte) in bytes.iter().enumerate() {
            self.0[i + 0x200] = *byte;
        }
    }
}

#[cfg(test)]
mod test {
    // TODO: tests for Chip8Memory::load_bytes_into_memory
}