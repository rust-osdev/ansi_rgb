pub use colored::*;

fn main() {
    println!("{}, {}, {}, {}, {}, {}, {}, {}",
        "Red".foreground(Colors::red()),
        "Orange".foreground(Colors::orange()).italic(),
        "Yellow".foreground(Colors::yellow()),
        "Green".foreground(Colors::green()).underline(),
        "Cyan".foreground(Colors::cyan()),
        "Blue".foreground(Colors::blue()).bold(),
        "Purple".foreground(Colors::purple()),
        "Violet".foreground(Colors::violet()).blink()
    );
}