use std::iter::zip;

use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;

/// A struct representing an equalizer with multiple frequency bands.
///
/// The equalizer is a vertical bar chart where each band represents a frequency range.
///
/// # Example
///
/// ```
/// use tui_equalizer::{Band, Equalizer};
///
/// # let area = ratatui::layout::Rect::default();
/// # let mut buf = ratatui::buffer::Buffer::empty(area);
/// let equalizer = Equalizer {
///     bands: vec![
///         Band::from(0.5),
///         Band::from(0.8),
///         Band::from(0.3),
///     ],
/// };
/// equalizer.render(area, buf);
/// ```
#[derive(Debug)]
pub struct Equalizer {
    /// A vector of `Band` structs representing each frequency band.
    pub bands: Vec<Band>,
}

/// A struct representing a single frequency band in the equalizer.
#[derive(Debug, Clone)]
pub struct Band {
    /// The normalized value of the band, where the maximum is 1.0.
    pub value: f64,
}

impl From<f64> for Band {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl Widget for Equalizer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let areas = Layout::horizontal(vec![Constraint::Length(2); self.bands.len()]).split(area);
        for (band, area) in zip(self.bands, areas.iter()) {
            band.render(*area, buf);
        }
    }
}

impl Widget for Band {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let value = self.value.clamp(0.0, 1.0);
        let height = (value * area.height as f64) as u16;

        // Calculate the color gradient step
        let color_step = 1.0 / area.height as f32;

        // Iterate over each segment and render it with the corresponding color
        for i in 0..height {
            // Green to Yellow to Red gradient
            let v = i as f32 * color_step;
            let vv = 1.0 - v;
            let r = if v < 0.5 { v * 2.0 * 255.0 } else { 255.0 } as u8;
            let g = if v < 0.5 { 255.0 } else { vv * 2.0 * 255.0 } as u8;
            let b = 0;
            let color = Color::Rgb(r, g, b);
            buf[(area.left(), area.bottom().saturating_sub(i + 1))]
                .set_fg(color)
                .set_symbol("⯀");
        }
    }
}
