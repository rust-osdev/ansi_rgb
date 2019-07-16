#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Rgb {
    pub const fn default() -> Self {
        Self::new(0, 0, 0)
    }

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b
        }
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Rgb::new(0, 0, 0)
    }
}

pub const fn white() -> Rgb {
    Rgb::new(255, 255, 255)
}

pub const fn black() -> Rgb {
    Rgb::default()
}

pub const fn red() -> Rgb {
    Rgb::new(255, 0, 0)
}

pub const fn orange() -> Rgb {
    Rgb::new(255, 128, 0)
}

pub const fn yellow() -> Rgb {
    Rgb::new(255, 255, 0)
}

pub const fn yellow_green() -> Rgb {
    Rgb::new(128, 255, 0)
}

pub const fn green() -> Rgb {
    Rgb::new(0, 255, 0)
}

pub const fn green_cyan() -> Rgb {
    Rgb::new(0, 255, 128)
}

pub const fn cyan() -> Rgb {
    Rgb::new(0, 255, 255)
}

pub const fn cyan_blue() -> Rgb {
    Rgb::new(0, 128, 255)
}

pub const fn blue() -> Rgb {
    Rgb::new(0, 0, 255)
}

pub const fn blue_magenta() -> Rgb {
    Rgb::new(128, 0, 255)
}

pub const fn magenta() -> Rgb {
    Rgb::new(255, 0, 255)
}

pub const fn magenta_pink() -> Rgb {
    Rgb::new(255, 0, 128)
}