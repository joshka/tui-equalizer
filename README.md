# tui-equalizer

<!-- cargo-rdme start -->

An equalizer widget for [Ratatui] with multiple frequency bands.

The equalizer is a vertical bar chart where each band represents a frequency range. Each band
can display a value from 0.0 to 1.0, where 1.0 is the maximum value.

![Made with VHS](https://vhs.charm.sh/vhs-FiRQkkDAUEnH2BrPbUx5i.gif)

This demo can be found in the examples folder in the git repo.

```shell
cargo run --example demo
```

Inspired by [a comment in the ratatui
repo](https://github.com/ratatui/ratatui/issues/1325#issuecomment-2335095486).

## Example

```rust
use tui_equalizer::{Band, Equalizer};

let equalizer = Equalizer {
    bands: vec![
        Band::from(0.5),
        Band::from(0.8),
        Band::from(0.3),
    ],
    brightness: 1.0,
};
equalizer.render(area, &mut buf);
```

## License

Copyright (c) Josh McKinney

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: ./LICENSE-APACHE
[LICENSE-MIT]: ./LICENSE-MIT


[Ratatui]: https://crates.io/crates/ratatui

<!-- cargo-rdme end -->
