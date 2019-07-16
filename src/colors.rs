#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Rgb {
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

pub struct Colors();

impl Colors {
    pub fn white() -> Rgb {
        Rgb::new(255, 255, 255)
    }

    pub fn black() -> Rgb {
        Default::default()
    }

    pub fn red() -> Rgb {
        Rgb::new(255, 0, 0)
    }

    pub fn orange() -> Rgb {
        Rgb::new(255, 127, 0)
    }

    pub fn yellow() -> Rgb {
        Rgb::new(255, 255, 0)
    }

    pub fn green() -> Rgb {
        Rgb::new(0, 255, 0)
    }

    pub fn cyan() -> Rgb {
        Rgb::new(0, 255, 255)
    }

    pub fn blue() -> Rgb {
        Rgb::new(0, 0, 255)
    }

    pub fn purple() -> Rgb {
        Rgb::new(127, 0, 127)
    }

    pub fn violet() -> Rgb {
        Rgb::new(127, 0, 255)
    }
}