use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::*,
};

use crate::game::{Game, Outcome};

const HORIZONTAL_MARGIN: u16 = 5;
const VERTICAL_MARGIN: u16 = 2;
const MIDDLE_MARGIN: u16 = 2;

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // styles
        let bold_style = Style::default().add_modifier(Modifier::BOLD);
        let red_bold_style = Style::default().patch(bold_style).fg(Color::Red);
        let green_bold_style = Style::default().patch(bold_style).fg(Color::Green);
        let dim_bold_style = Style::default()
            .patch(bold_style)
            .add_modifier(Modifier::DIM);
        let underlined_dim_bold_style = Style::default()
            .patch(dim_bold_style)
            .add_modifier(Modifier::UNDERLINED);
        
        match !self.finished {
            true =>  {
                let max_chars_per_line = (area.width - (HORIZONTAL_MARGIN * 2)) /2; // divide by 2 because chinese characters are double width
                let mut prompt_zh_occupied_lines =
                            ((self.prompt_zh.chars().count() as f64 / max_chars_per_line as f64).ceil() + 1.0) as u16;
                if self.prompt_zh.chars().count() <= max_chars_per_line as usize {
                    prompt_zh_occupied_lines = 1;
                }

                let mut prompt_zy_occupied_lines =
                            ((self.prompt_zy.chars().count() as f64 / max_chars_per_line as f64).ceil() + 1.0) as u16;
                if self.prompt_zy.chars().count() <= max_chars_per_line as usize {
                    prompt_zy_occupied_lines = 1;
                }
                
                let min_height = prompt_zh_occupied_lines + prompt_zy_occupied_lines + MIDDLE_MARGIN + 2 * VERTICAL_MARGIN;

                if area.height < min_height {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .horizontal_margin(HORIZONTAL_MARGIN)
                        .vertical_margin(VERTICAL_MARGIN)
                        .constraints(
                            [
                                Constraint::Length(1),
                            ]
                            .as_ref(),
                        )
                        .split(area);
                    let widget = Paragraph::new(Span::styled(
                        format!("Please make your terminal taller"),
                        bold_style,
                    ))
                    .alignment(Alignment::Center);
                    widget.render(chunks[0], buf);
                    return;
                }

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(HORIZONTAL_MARGIN)
                    .vertical_margin(VERTICAL_MARGIN)
                    .constraints(
                        [
                            Constraint::Length(
                                ((area.height as f64 - min_height as f64) / 2.0) as u16,
                            ),
                            Constraint::Min(prompt_zh_occupied_lines),
                            Constraint::Min(MIDDLE_MARGIN),
                            Constraint::Min(prompt_zy_occupied_lines),
                            Constraint::Length(
                                ((area.height as f64 - min_height as f64) / 2.0) as u16,
                            ),
                        ]
                        .as_ref(),
                    )
                    .split(area);

                
                let test: String = format!("{} {} {} {}", self.prompt_zy.chars().count(), area.height as f64, prompt_zh_occupied_lines as f64, prompt_zy_occupied_lines as f64);
                let widget = Paragraph::new(Span::styled(
                    test,
                    bold_style,
                ))
                .alignment(Alignment::Center);

                widget.render(chunks[0], buf);

                let mut spans = self
                    .input
                    .iter()
                    .enumerate()
                    .map(|(idx, input)| {
                        let expected = self.prompt_zy.chars().nth(idx).unwrap().to_string();

                        match input.outcome {
                            Outcome::Incorrect => Span::styled(
                                match expected.as_str() {
                                    " " => "·".to_owned(),
                                    _ => expected,
                                },
                                red_bold_style,
                            ),
                            Outcome::Correct => Span::styled(expected, green_bold_style),
                        }
                    })
                    .collect::<Vec<Span>>();

                spans.push(Span::styled(
                    self.prompt_zy
                        .chars()
                        .nth(self.cursor_pos)
                        .unwrap()
                        .to_string(),
                    underlined_dim_bold_style,
                ));

                for idx in self.cursor_pos + 1..self.prompt.len() {
                    spans.push(Span::styled(
                        self.prompt_zy.chars().nth(idx).unwrap().to_string(),
                        dim_bold_style,
                    ));
                }

                let widget = Paragraph::new(Line::from(spans))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false });

                widget.render(chunks[3], buf);

                // let widget = Paragraph::new(self.prompt.clone())
                //     .alignment(Alignment::Center)
                //     .wrap(Wrap { trim: true });

                // widget.render(chunks[2], buf);

                let widget = Paragraph::new(self.prompt_zh.clone())
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });

                widget.render(chunks[1], buf);
            }

            false => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .horizontal_margin(HORIZONTAL_MARGIN)
                    .vertical_margin(VERTICAL_MARGIN)
                    .constraints(
                        [
                            Constraint::Min(1),
                            Constraint::Length(1),
                            Constraint::Length(1), // for padding
                            Constraint::Length(1),
                        ]
                        .as_ref(),
                    )
                    .split(area);

                let stats = Paragraph::new(Span::styled(
                    format!(
                        "You did it! Press r to restart or q to quit"
                    ),
                    bold_style,
                ))
                .alignment(Alignment::Center);

                stats.render(chunks[1], buf);
            }
        }

    }
}
