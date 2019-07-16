use colored::*;

fn main() {
    println!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "Red".fg(red()),
        "Orange".fg(orange()),
        "Yellow".fg(yellow()),
        "Yellow green".fg(yellow_green()),
        "Green".fg(green()),
        "Green cyan".fg(green_cyan()),
        "Cyan".fg(cyan()),
        "Cyan blue".fg(white()).bg(cyan_blue()),
        "Blue".fg(white()).bg(blue()),
        "Blue magenta".fg(white()).bg(blue_magenta()),
        "Magenta".fg(magenta()),
        "Magenta pink".fg(magenta_pink()),
        "Custom color".fg(Rgb::new(123, 231, 111)).bg(Rgb::new(10, 100, 20))
    );
}