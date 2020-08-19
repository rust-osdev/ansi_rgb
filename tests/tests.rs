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
        "\u{1b}[48;2;0;0;255m\u{1b}[38;2;255;0;0mHello, world\u{1b}[39m\u{1b}[49m",
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
    assert_eq!("\u{1b}[34mHello, world\u{1b}[39m", format!("{}", formatted));
}

#[test]
fn color4() {
    let hello_world = "Hello, world";
    let formatted = hello_world.fg(Color4::BLACK).bg(Color4::WHITE);
    assert_eq!(
        "\u{1b}[107m\u{1b}[30mHello, world\u{1b}[39m\u{1b}[49m",
        format!("{}", formatted)
    );
}

#[test]
fn color8_background() {
    assert_eq!(
        "\u{1b}[48;5;0mHello, world\u{1b}[49m",
        format!("{}", "Hello, world".bg(Color8::new(0)))
    );
}

#[test]
fn color8_foreground() {
    assert_eq!(
        "\u{1b}[38;5;0mHello, world\u{1b}[39m",
        format!("{}", "Hello, world".fg(Color8::new(0)))
    );
}

#[test]
fn convert_color3_to_color4() {
    assert_eq!(
        Color4::new(Color3::RED, false),
        Color3::RED.into()
    )
}

#[test]
fn convert_color3_to_color8() {
    assert_eq!(
        Color8::new(0),
        Color3::BLACK.into()
    )
}

#[test]
fn convert_color4_to_color8() {
    assert_eq!(
        Color8::new(15),
        Color4::WHITE.into()
    )
}

#[test]
fn format_padding() {
    assert_eq!(
        "\u{1b}[38;2;0;0;0mX    \u{1b}[39m",
        format!("{:5}", "X".fg(black()))
    )
}

#[test]
fn convert_rgb_to_color8() {
    assert_eq!(Color8::new_rgb(5, 2, 0).unwrap(), Color8::new(208));
}

#[test]
fn convert_gray_to_color8() {
    assert_eq!(Color8::new_gray(23).unwrap(), Color8::new(255));
}

#[test]
fn color8_new_rgb() {
    assert!(Color8::new_rgb(6, 5, 5).is_err());
    assert!(Color8::new_rgb(5, 6, 5).is_err());
    assert!(Color8::new_rgb(5, 5, 6).is_err());
    assert!(Color8::new_rgb(5, 5, 5).is_ok());
}
