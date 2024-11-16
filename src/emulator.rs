use std::collections::VecDeque;

#[derive(Debug)]
pub struct Emulator {
    /// Talk about how a bool technically represents a bit but is physically represented using a byte.
    memory: [u8; 4096],
    display: [[u8; 64]; 32],
    program_counter: u16,
    index_register: u16,
    stack: VecDeque<u16>,
    delay_timer: u8,
    sound_timer: u8,

    registers: [u8; 16]
}

impl Emulator {
    pub fn write(&mut self, bytes: Vec<u8>, offset: u16) {
        if (bytes.len() + offset as usize) >= self.memory.len() { panic!("Memory buffer overflow!") }

        let mut i = 0;
        for byte in bytes {
            self.memory[(i + offset) as usize] = byte;
            i += 1;
        }
    }

    pub fn new() -> Self {
        let mut emulator = Self {
            memory: [0; 4096],
            display: [[0; 64]; 32],
            program_counter: 0,
            index_register: 0,
            stack: VecDeque::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16]
        };

        emulator.init();

        emulator
    }

    /// Initializes initial state for critical parts of Emulator (e.g. font).
    pub fn init(&mut self) {
        self.write(
            Vec::from([
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
            ]),
            0x50,
        )
    }
}
