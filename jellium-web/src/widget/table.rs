//! The one construct the three dashboard table screens draw through.

use iced::widget::{Space, column, container, row};
use iced::{Element, Fill};

use crate::app::Message;
use crate::style::{self, Band, Css, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::line;
use crate::window::{self, Window};

/// Which kind of column MRT draws: one holding written values, padded on every
/// side, or the display column it stands a row's own controls in and pads at
/// the sides alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Holding {
    Written,
    Display,
}

impl Holding {
    /// The padding MRT gives a body cell of this kind at its compact density.
    // reference: table-body-cell
    fn padding(self) -> space::Padding {
        match self {
            Holding::Written => space::TABLE_CELL_PAD,
            Holding::Display => space::TABLE_DISPLAY_PAD,
        }
    }
}

/// One column: what its head reads, absent where the reference gives it no
/// header, the width that reference declares, and what it holds.
#[derive(Debug, Clone)]
pub struct Column {
    pub label: Option<Text>,
    pub width: Css,
    pub holding: Holding,
}

/// One table as a screen declares it: the controls its top toolbar leads with,
/// its columns, the window over its rows, how many rows there are, and one
/// row's cells in the order its columns stand.
pub struct Table<'a> {
    pub toolbar: Vec<Element<'a, Message>>,
    pub columns: Vec<Column>,
    pub window: Window,
    pub rows: usize,
    pub cells: Box<dyn Fn(usize) -> Vec<Element<'a, Message>> + 'a>,
}

/// The width MRT draws a column at, which is what its definition declares held
/// to the narrowest MRT will draw.
// reference: table-cell-width
fn width(declared: Css, band: Band) -> f32 {
    match declared > space::TABLE_COLUMN_FLOOR {
        true => style::drawn(declared.drawn(band)),
        false => style::drawn(space::TABLE_COLUMN_FLOOR.drawn(band)),
    }
}

/// The rule one cell draws under itself, as wide as that cell.
// reference: mui-table-cell
fn rule<'a>(across: f32, band: Band) -> Element<'a, Message> {
    container(
        Space::new()
            .width(across)
            .height(style::drawn(space::TABLE_CELL_RULE.drawn(band))),
    )
    .style(style::table_rule)
    .into()
}

/// One cell: its content at its column's own padding, over the rule the cell
/// draws, held to its column's own width.
fn cell<'a>(
    content: Element<'a, Message>,
    across: f32,
    padding: space::Padding,
    band: Band,
) -> Element<'a, Message> {
    column![
        container(content)
            .padding(style::padding(padding))
            .width(across)
            .height(Fill),
        rule(across, band),
    ]
    .into()
}

/// The head row, which does not scroll with the body under it.
// reference: table-head-cell
fn head<'a>(columns: &[Column], widths: &[f32], band: Band) -> Element<'a, Message> {
    let cells = columns
        .iter()
        .zip(widths.iter())
        .map(|(column, across)| {
            let content: Element<'a, Message> = match column.label {
                Some(label) => line(
                    strings::lookup(label),
                    typeface::BODY_2,
                    typeface::TABLE_HEAD_WEIGHT,
                    typeface::TABLE_HEAD_LEADING,
                ),
                None => Space::new().into(),
            };
            cell(content, *across, space::TABLE_HEAD_PAD, band)
        })
        .collect::<Vec<Element<'a, Message>>>();
    container(row(cells))
        .height(style::drawn(space::table_head(band)))
        .into()
}

/// One written cell, in the face MUI writes every cell of a table in.
// reference: mui-table-cell
pub fn written<'a>(content: String) -> Element<'a, Message> {
    line(
        content,
        typeface::BODY_2,
        typeface::Weight::Regular,
        typeface::BODY_2_LEADING,
    )
}

/// A time as the reference's own short date and time, in the zone the browser
/// stands in.
// reference: table-date-cell
// reference: date-fns-en-us
pub fn stamped<Zone: chrono::TimeZone>(at: chrono::DateTime<Zone>) -> String {
    at.with_timezone(&chrono::Local)
        .format("%m/%d/%Y, %-I:%M %p")
        .to_string()
}

/// The reference's `MaterialReactTable` at the compact density its three table
/// screens set: the toolbar's row over a head that does not scroll, over a
/// windowed body, on the paper the table stands on. Every cell is held to its
/// column's own width and its column's own padding, and every body row to the
/// one height `space::table_row` answers, which is the height its window
/// counts in.
// reference: table-paper
// reference: table-container
// reference: table-body-cell
// reference: table-head-cell
// reference: table-cell-width
pub fn drawn<'a>(table: Table<'a>, band: Band) -> Element<'a, Message> {
    let Table {
        toolbar,
        columns,
        window,
        rows,
        cells,
    } = table;
    let widths: Vec<f32> = columns
        .iter()
        .map(|column| width(column.width, band))
        .collect();
    let holdings: Vec<space::Padding> = columns
        .iter()
        .map(|column| column.holding.padding())
        .collect();

    let toolbar = container(row(toolbar).spacing(style::drawn(space::TABLE_TOOLBAR_GAP.drawn())))
        .padding(style::drawn(space::TABLE_TOOLBAR_PAD.drawn()))
        .height(style::drawn(space::TABLE_TOOLBAR.drawn()));

    let heading = head(&columns, &widths, band);

    let body = window::list(window, rows, move |index| {
        let held = cells(index)
            .into_iter()
            .zip(widths.iter().zip(holdings.iter()))
            .map(|(content, (across, padding))| cell(content, *across, *padding, band))
            .collect::<Vec<Element<'a, Message>>>();
        container(row(held))
            .height(style::drawn(space::table_row(band)))
            .into()
    });

    container(column![toolbar, heading, body])
        .style(style::table)
        .width(Fill)
        .height(Fill)
        .into()
}
