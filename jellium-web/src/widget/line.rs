//! The measured line behind [`crate::widget::line`].

use iced::advanced::text::{Paragraph, Shaping, Wrapping, paragraph};
use iced::advanced::widget::{Tree, tree};
use iced::advanced::{Layout, Widget, layout, mouse, renderer, text};
use iced::{Rectangle, Size};

use crate::style::{self, Length, typeface};

/// What stands in for the part of the content the width could not hold.
const ELLIPSIS: &str = "\u{2026}";

/// A single line of text, laid out with no wrapping and cut with an ellipsis at
/// the last character that fits the width the layout gives it, measured through
/// the renderer's own paragraph.
pub struct Line {
    content: String,
    size: Length,
    weight: typeface::Weight,
    leading: typeface::Leading,
}

impl Line {
    pub fn new(
        content: String,
        size: Length,
        weight: typeface::Weight,
        leading: typeface::Leading,
    ) -> Self {
        Self {
            content,
            size,
            weight,
            leading,
        }
    }

    /// The content as it is drawn at `width`: whole when it fits, and otherwise
    /// the longest prefix whose ellipsis still fits.
    fn fitted<P: Paragraph<Font = iced::Font>>(&self, width: f32) -> String {
        if self.measured::<P>(&self.content).width <= width {
            return self.content.clone();
        }
        let ends: Vec<usize> = self
            .content
            .char_indices()
            .map(|(at, character)| at + character.len_utf8())
            .collect();
        let mut kept = 0;
        let mut rejected = ends.len();
        while rejected - kept > 1 {
            let trying = kept + (rejected - kept) / 2;
            if self.measured::<P>(&self.cut(&ends, trying)).width > width {
                rejected = trying;
            } else {
                kept = trying;
            }
        }
        self.cut(&ends, kept)
    }

    /// The first `characters` characters of the content, with the ellipsis that
    /// stands for the rest.
    fn cut(&self, ends: &[usize], characters: usize) -> String {
        let taken = characters
            .checked_sub(1)
            .map(|last| ends[last])
            .unwrap_or(0);
        format!("{}{ELLIPSIS}", &self.content[..taken])
    }

    fn measured<P: Paragraph<Font = iced::Font>>(&self, content: &str) -> Size {
        P::with_text(self.laid(content, Size::INFINITE)).min_bounds()
    }

    fn laid<'a>(&self, content: &'a str, bounds: Size) -> text::Text<&'a str, iced::Font> {
        text::Text {
            content,
            bounds,
            size: style::drawn(self.size.drawn()).into(),
            line_height: style::leading(self.leading),
            font: style::font(self.weight),
            align_x: text::Alignment::Default,
            align_y: iced::alignment::Vertical::Top,
            shaping: Shaping::Advanced,
            wrapping: Wrapping::None,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Line
where
    Renderer: text::Renderer<Font = iced::Font>,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<paragraph::Plain<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(paragraph::Plain::<Renderer::Paragraph>::default())
    }

    fn size(&self) -> Size<iced::Length> {
        Size {
            width: iced::Length::Fill,
            height: iced::Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree
            .state
            .downcast_mut::<paragraph::Plain<Renderer::Paragraph>>();
        layout::sized(limits, iced::Length::Fill, iced::Length::Shrink, |limits| {
            let bounds = limits.max();
            let drawn = self.fitted::<Renderer::Paragraph>(bounds.width);
            let _ = state.update(self.laid(&drawn, bounds));
            state.min_bounds()
        })
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        defaults: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree
            .state
            .downcast_ref::<paragraph::Plain<Renderer::Paragraph>>();
        let bounds = layout.bounds();
        let anchor = bounds.anchor(
            state.min_bounds(),
            state.raw().align_x(),
            state.raw().align_y(),
        );
        renderer.fill_paragraph(state.raw(), anchor, defaults.text_color, *viewport);
    }
}

impl<'a, Message, Theme, Renderer> From<Line> for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: text::Renderer<Font = iced::Font> + 'a,
    Message: 'a,
    Theme: 'a,
{
    fn from(line: Line) -> Self {
        Self::new(line)
    }
}
