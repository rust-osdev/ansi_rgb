# ansi_rgb
Colorful terminal text using [ANSI escape sequences](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_parameters).

 * Very simple API
 * 3-, 4-, 8-, and 24-bit colors
 * Colors all the [formatting traits](https://doc.rust-lang.org/std/fmt/#formatting-traits)
 * Easy to add your own color types
 * `no_std` compliant

[![crates.io badge](https://img.shields.io/crates/v/ansi_rgb.svg)](https://crates.io/crates/ansi_rgb)<br/>
[![docs.rs badge](https://docs.rs/ansi_rgb/badge.svg)](https://docs.rs/ansi_rgb)<br/>
[![Downloads badge](https://img.shields.io/crates/d/ansi_rgb.svg)](https://crates.io/crates/ansi_rgb)

# Upcoming changes

Features:

* Make it easier to extend to other color types
* Add built-in support for 3-, 4-, and 8-bit colors
* Slim down when `no-default-features` is specified

Bug fixes:

* Format string parameters (like padding) now actually work

Breaking changes:

* Pretty much every trait is changing names