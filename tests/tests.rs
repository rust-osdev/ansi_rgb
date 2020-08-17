use ansi_rgb::*;
use rgb::RGB8;

#[test]
fn use_all_colors() {
    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
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
        "Custom color"
            .fg(RGB8::new(123, 231, 111))
            .bg(RGB8::new(10, 100, 20))
    );
}

#[test]
fn temporary_value_dropped_while_borrowed() {
    // https://github.com/rust-osdev/ansi_rgb/issues/3
    let foo = "Hello, world!".fg(green()).bg(red());
    println!("{}", foo);
}

#[test]
fn formatting_of_pointers() {
    // https://github.com/rust-osdev/ansi_rgb/issues/2
    let hello_world = "Hello, world";
    let formatted = hello_world.fg(red()).bg(blue());
    assert_eq!(
        "\u{1b}[48;2;0;0;255m\u{1b}[38;2;255;0;0mHello, world\u{1b}[0m\u{1b}[0m",
        format!("{}", formatted)
    );
}

#[test]
fn foo() {
    use ansi_rgb::*;
    #[derive(Debug)]
    struct Foo(i32, i32);

    let foo = Foo(1, 2);
    println!("{:?}", foo.fg(green()));
}

#[test]
fn color3() {
    let hello_world = "Hello, world";
    let formatted = hello_world.fg(Color3::BLUE);
    assert_eq!("\u{1b}[34mHello, world\u{1b}[0m", format!("{}", formatted));
}

#[test]
fn color4() {
    let hello_world = "Hello, world";
    let formatted = hello_world.fg(Color4::BLACK).bg(Color4::WHITE);
    assert_eq!(
        "\u{1b}[107m\u{1b}[30mHello, world\u{1b}[0m\u{1b}[0m",
        format!("{}", formatted)
    );
}
