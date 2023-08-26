use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::*,
    Frame,
    buffer::Buffer,
    style::Style,
};

use crate::game::Game;

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(area.left(), area.top(), "TOTOTOTOTTOT", Style::default());
    }
}