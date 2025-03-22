use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

macro_rules! left_cell {
    ($text:expr) => {
        Cell::from(Line::from($text).alignment(Alignment::Left))
    };
}

macro_rules! right_cell {
    ($text:expr) => {
        Cell::from(Line::from($text).alignment(Alignment::Right))
    };
}
#[cfg(not(target_os = "macos"))]
macro_rules! alt_keypress {
    ($key:literal) => {
        concat!("Alt+", $key)
    };
}

#[cfg(target_os = "macos")]
macro_rules! alt_keypress {
    ($key:literal) => {
        concat!("Opt+", $key)
    };
}

const CELLS: [[&str; 2]; 13] = [
    ["Search", alt_keypress!("s")],
    ["Champ Select", alt_keypress!("c")],
    ["Mode Select", alt_keypress!("m")],
    ["Role Select", alt_keypress!("r")],
    ["Version Select", alt_keypress!("v")],
    ["Region Select", alt_keypress!("w")],
    ["Build Select", alt_keypress!("b")],
    ["Log Viewer", alt_keypress!("l")],
    ["Exit Log Viewer", "Q"],
    ["Back", "Esc"],
    ["Send", "Enter"],
    ["Quit", "Ctrl+Q"],
    ["Help", "?"],
];

#[allow(clippy::cast_possible_truncation)]
const fn left_size() -> u16 {
    let mut max = 0u16;
    let mut idx = 0usize;

    while idx < CELLS.len() {
        let len = CELLS[idx][0].len() as u16;
        if len > max {
            max = len;
        }
        idx += 1;
    }

    max + 1
}

#[allow(clippy::cast_possible_truncation)]
const fn right_size() -> u16 {
    let mut max = 0u16;
    let mut idx = 0usize;

    while idx < CELLS.len() {
        let len = CELLS[idx][1].len() as u16;
        if len > max {
            max = len;
        }
        idx += 1;
    }

    max
}

const CONSTRAINTS: [Constraint; 2] = [
    Constraint::Length(left_size()),
    Constraint::Length(right_size()),
];

#[allow(clippy::cast_possible_truncation)]
pub fn make() -> (impl Widget, Rect) {
    (
        Table::new(
            CELLS
                .iter()
                .map(|row| Row::new(vec![left_cell!(row[0]), right_cell!(row[1])])),
            CONSTRAINTS,
        )
        .column_spacing(2)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().bold())
                .borders(Borders::ALL),
        ),
        Rect::new(0, 0, left_size() + right_size() + 3, CELLS.len() as u16 + 1),
    )
}
