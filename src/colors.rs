use rgb::RGB8;

/// Makes <code style="color: black; background: #FFFFFF">white</code>
pub const fn white() -> RGB8 {
    RGB8::new(255, 255, 255)
}

/// Makes <code style="color: white; background: #000000">black</code>
pub const fn black() -> RGB8 {
    RGB8::new(0, 0, 0)
}

/// Makes <code style="color: black; background: #FF0000">red</code>
pub const fn red() -> RGB8 {
    RGB8::new(255, 0, 0)
}

/// Makes <code style="color: black; background: #FF8000">orange</code>
pub const fn orange() -> RGB8 {
    RGB8::new(255, 128, 0)
}

/// Makes <code style="color: black; background: #FFFF00">yellow</code>
pub const fn yellow() -> RGB8 {
    RGB8::new(255, 255, 0)
}

/// Makes <code style="color: black; background: #80FF00">yellow green</code>
pub const fn yellow_green() -> RGB8 {
    RGB8::new(128, 255, 0)
}

/// Makes <code style="color: black; background: #00FF00">green</code>
pub const fn green() -> RGB8 {
    RGB8::new(0, 255, 0)
}

/// Makes <code style="color: black; background: #00FF80">green cyan</code>
pub const fn green_cyan() -> RGB8 {
    RGB8::new(0, 255, 128)
}

/// Makes <code style="color: black; background: #00FFFF">cyan</code>
pub const fn cyan() -> RGB8 {
    RGB8::new(0, 255, 255)
}

/// Makes <code style="color: white; background: #0080FF">cyan blue</code>
pub const fn cyan_blue() -> RGB8 {
    RGB8::new(0, 128, 255)
}

/// Makes <code style="color: white; background: #0000FF">blue</code>
pub const fn blue() -> RGB8 {
    RGB8::new(0, 0, 255)
}

/// Makes <code style="color: white; background: #8000FF">blue magenta</code>
pub const fn blue_magenta() -> RGB8 {
    RGB8::new(128, 0, 255)
}

/// Makes <code style="color: black; background: #FF00FF">magenta</code>
pub const fn magenta() -> RGB8 {
    RGB8::new(255, 0, 255)
}

/// Makes <code style="color: black; background: #FF0080">magenta pink</code>
pub const fn magenta_pink() -> RGB8 {
    RGB8::new(255, 0, 128)
}