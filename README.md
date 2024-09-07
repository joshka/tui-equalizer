# tui-equalizer

Inspired by <https://github.com/ratatui/ratatui/issues/1325#issuecomment-2335095486>

```rust
use tui_equalizer::{Band, Equalizer};

let equalizer = Equalizer {
    bands: vec![
        Band::from(0.5),
        Band::from(0.8),
        Band::from(0.3),
    ],
};
equalizer.render(area, buf);
```

![Made with VHS](https://vhs.charm.sh/vhs-732McVor5Mxwa0IMDh7uP6.gif)
