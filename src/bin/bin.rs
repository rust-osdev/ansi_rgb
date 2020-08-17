use ansi_rgb::*;
use rgb::RGB8;


struct XorShiftState (u32);

impl XorShiftState {
    fn new(seed: u32) -> Self {
        Self(seed)
    }

    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

fn create_colors(num: usize) -> Vec<RGB8> {
    let mut xor_shift = XorShiftState::new(0x12345678);
    let mut colors : Vec<RGB8> = Vec::with_capacity(num);
    for _ in 0..num {
        let next = xor_shift.next();
        let color = RGB8::new(
            (next & 0xFF) as u8,
            ((next >> 8) & 0xFF) as u8,
            ((next >> 16) & 0xFF) as u8
        );
        colors.push(color);
    }
    colors
}

fn main() {
    let colors = create_colors(80);
    let greeting = "Hello, world! This is a test of the emergency broadcast system. Do not be alarmed. This is only a test.";
    let mut color_index = 0usize;
    const BACKGROUND: RGB8 = black();
    for letter in greeting.chars() {
        let fg = colors[color_index];
        print!("{}", letter.fg(fg).bg(BACKGROUND));
        color_index = (color_index + 1) % colors.len();
    }
    println!();

    println!("{}", "Hello again!".fg(Color4::new(Color3::RED, false)));
    println!("{}", "Hello again!".fg(Color3::RED));
    println!("{}", "Hello again!".fg(Color4::new(Color3::RED, true)));
    println!("{}", "Hello again!".bg(Color4::new(Color3::RED, false)));
    println!("{}", "Hello again!".bg(Color3::RED));
    println!("{}", "Hello again!".bg(Color4::new(Color3::RED, true)));
}