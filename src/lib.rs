#![no_std]
/*!
Colorful terminal text using [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_parameters).

 * Very simple API
 * 3-, 4-, 8-, and 24-bit colors
 * Colors all the [formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits)
 * `no_std` compliant

# Foreground colors

```rust
use ansi_rgb::{ Colorable, red };

println!("{}", "Hello, world!".fg(red()));
```

Output:

<code style="color: red">Hello, world!</code>

# Background colors

```rust
use ansi_rgb::{ Colorable, red };

println!("{}", "Hello, world!".bg(red()));
```

Output:

<code style="background: red">Hello, world!</code>

# Anything formattable

```rust
# use ansi_rgb::*;
#[derive(Debug)]
struct Foo(i32, i32);

let foo = Foo(1, 2);
println!("{:?}", foo.fg(green()));
```

Output:

<code style="color: #00FF00">Foo(1, 2)</code>

# 3-bit colors

```rust
use ansi_rgb::{ Colorable, Color3 };

println!("{}", "Hello, world!".fg(Color3::RED).bg(Color3::BLACK));
```

Output:

<code style="color: #800000; background: #000000">Hello, world!</code>

# 4-bit colors

```rust
use ansi_rgb::{ Colorable, Color4 };

println!("{}", "Hello, world!".fg(Color4::BRIGHT_RED).bg(Color4::BLACK));
```

Output:

<code style="color: #ff0000; background: #000000">Hello, world!</code>

# 8-bit colors

```rust
use ansi_rgb::{ Colorable, Color8 };

println!("{}", "Hello, world!".fg(Color8::new(160)).bg(Color8::new(0)));
```

Output:

<code style="color: #d70000; background: #000000">Hello, world!</code>

# 24-bit colors

```toml
# Cargo.toml
[dependencies]
rbg = "0.8"
```

```rust
use ansi_rgb::{ Colorable };
use rgb::RGB8;

let fg = RGB8::new(123, 231, 111);
let bg = RGB8::new(10, 100, 20);
println!("{}", "Yuck".fg(fg).bg(bg));
```

Output:

<code style="color: #7BE76F; background: #0A6414">Yuck</code>

# Windows users

You need to [set your console mode](https://docs.microsoft.com/en-us/windows/console/console-modes). Otherwise you'll get garbage like this:

`�[48;2;159;114;0m �[0m`
 */

mod color;
mod colors;

pub use color::*;
pub use colors::*;
