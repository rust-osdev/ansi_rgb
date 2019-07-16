pub use colored::*;

fn main() {
    println!("{}, {}, {}, {}, {}, {}, {}, {}",
        "Red".foreground(Colors::black()).background(Colors::red()),
        "Orange".foreground(Colors::black()).background(Colors::orange()),
        "Yellow".foreground(Colors::black()).background(Colors::yellow()),
        "Green".foreground(Colors::black()).background(Colors::green()),
        "Cyan".foreground(Colors::black()).background(Colors::cyan()),
        "Blue".foreground(Colors::white()).background(Colors::blue()),
        "Purple".foreground(Colors::white()).background(Colors::purple()),
        "Violet".foreground(Colors::white()).background(Colors::violet())
    );
}