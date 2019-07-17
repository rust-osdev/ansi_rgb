# ansi_rgb
Colorful console text using [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_parameters).

 * Very simple API
 * Full color (using the [`rgb` crate](https://crates.io/crates/rgb))
 * Colors all the [formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits)
 * `no_std` compliant

[![crates.io badge](https://img.shields.io/crates/v/ansi_rgb.svg)](https://crates.io/crates/ansi_rgb)<br/>
[![docs.rs badge](https://docs.rs/ansi_rgb/badge.svg)](https://docs.rs/ansi_rgb)<br/>
[![Downloads badge](https://img.shields.io/crates/d/ansi_rgb.svg)](https://crates.io/crates/ansi_rgb)

Cargo.toml:
```toml
ansi_rgb = "0.2.0"
```

# Foreground colors

```rust
use ansi_rgb::{ Foreground, red };

println!("{}", "Hello, world!".fg(red()));
```

Output:

<code style="color: red">Hello, world!</code>

# Background colors

```rust
use ansi_rgb::{ Background, red };

println!("{}", "Hello, world!".bg(red()));
```

Output:

<code style="background: red">Hello, world!</code>

# Mix and match

```toml
# Cargo.toml
[dependencies]
rbg = "0.8"
```

```rust
use ansi_rgb::{ Foreground, Background };
use rgb::RGB8;

let fg = RGB8::new(123, 231, 111);
let bg = RGB8::new(10, 100, 20);
println!("{}", "Yuck".fg(fg).bg(bg));
```

Output:

<code style="color: #7BE76F; background: #0A6414">Yuck</code>

# Anything formattable

```rust
#[derive(Debug)]
struct Foo(i32, i32);

let foo = Foo(1, 2);
println!("{:?}", foo.fg(green()));
```

Output:

<code style="color: #00FF00">Foo(1, 2)</code>

# Windows users

You need to [set your console mode](https://docs.microsoft.com/en-us/windows/console/console-modes). Otherwise you'll get garbage like this:

`�[48;2;159;114;0m �[0m`