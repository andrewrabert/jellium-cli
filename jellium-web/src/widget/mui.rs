//! MUI's own controls as the dashboard's react routes draw them.

use std::borrow::Cow;

use iced::widget::{Space, button, column, container, row, stack, text_input};
use iced::{Element, Fill};

use super::{Choice, prose, tinted};
use crate::app::Message;
use crate::icon::{self, Icon};
use crate::style::{self, Band, Viewport, space, typeface};
use crate::text::{self as strings, Text};

/// One line of the heading ladder `DEFAULT_THEME_OPTIONS` sets: its size from
/// the theme, its line box and its weight from MUI's own variant.
// reference: mui-theme-typography
// reference: mui-typography
pub fn heading<'a>(rank: typeface::Rank, content: impl Into<Cow<'a, str>>) -> Element<'a, Message> {
    tinted(
        content,
        rank.size(),
        typeface::HEADING_WEIGHT,
        rank.leading(),
        iced::widget::text::default,
    )
}

/// A filled field's label shrunk into the head of its own face, and the rule
/// the field draws under its foot.
// reference: mui-filled-underline
// reference: mui-input-label
fn dressed<'a>(
    control: Element<'a, Message>,
    label: impl Into<Cow<'a, str>>,
    band: Band,
) -> Element<'a, Message> {
    let shrunk = container(tinted(
        label,
        typeface::FILLED_LABEL,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
        style::muted,
    ))
    .padding(style::inset(space::FILLED_LABEL_INSET, band));
    column![
        stack![control, shrunk],
        container(Space::new())
            .width(Fill)
            .height(style::drawn(space::FILLED_RULE.drawn(band)))
            .style(style::filled_rule),
    ]
    .into()
}

/// A filled field: its own face with the head rounded, its label shrunk into
/// that head, the value beneath at the field's own padding, and the rule the
/// field draws under itself.
// the label is drawn shrunk whether or not the field carries a value, where
// the reference floats it down over an empty field
// the rule stands at rest, the canvas telling what stacks it nothing of what
// carries the caret
// reference: mui-filled-root
// reference: mui-filled-input
// reference: mui-filled-underline
// reference: mui-input-base
// reference: mui-input-label
pub fn field<'a>(
    label: impl Into<Cow<'a, str>>,
    value: &str,
    edited: impl Fn(String) -> Message + 'a,
    band: Band,
) -> Element<'a, Message> {
    let typed = text_input("", value)
        .style(move |theme, status| style::filled(theme, status, band))
        .size(style::drawn(typeface::BODY.drawn()))
        .line_height(style::leading(typeface::FILLED_LEADING))
        .padding(style::inset(space::FILLED_PAD, band))
        .on_input(edited)
        .width(Fill);
    dressed(typed.into(), label, band)
}

/// The same field carrying the option standing rather than a typed value, the
/// chevron laid over its trailing edge, and the menu the options stand in.
// reference: mui-select-filled
// reference: mui-select-icon
// reference: mui-select-chevron
// reference: mui-menu-item
// reference: mui-menu-paper
pub fn chosen<'a, T>(
    label: impl Into<Cow<'a, str>>,
    offered: Vec<Choice<T>>,
    held: &T,
    picked: impl Fn(T) -> Message + 'a,
    viewport: Viewport,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
{
    let band = viewport.band();
    let options = offered.len();
    for choice in &offered {
        crate::fonts::observed(&choice.label, typeface::Weight::Regular);
    }
    let standing = offered.iter().find(|choice| &choice.value == held).cloned();
    let field = iced::widget::pick_list(offered, standing, move |choice| picked(choice.value))
        .style(move |theme, status| style::filled_select(theme, status, band))
        .menu_style(move |theme| style::filled_menu(theme, band))
        .menu_height(style::drawn(space::menu_height(options, viewport)))
        .handle(iced::widget::pick_list::Handle::None)
        .font(style::font(typeface::Weight::Regular))
        .text_size(style::drawn(typeface::BODY.drawn()))
        .text_line_height(style::leading(typeface::FILLED_LEADING))
        .padding(style::inset(space::FILLED_SELECT_PAD, band))
        .width(Fill);
    let chevron = container(icon::tinted(
        Icon::ArrowDropDown,
        typeface::CONTROL_GLYPH,
        style::chevron,
    ))
    .padding(iced::Padding::ZERO.right(style::drawn(space::FILLED_CHEVRON_INSET.drawn(band))))
    .align_right(Fill)
    .center_y(Fill);
    dressed(stack![field, chevron].into(), label, band)
}

/// A box and the label beside it, at `MuiFormControlLabel`'s own margins and
/// `MuiCheckbox`'s own padding; the box is one glyph and carries no outline of
/// its own.
// reference: mui-checkbox
// reference: mui-switch-base
// reference: mui-form-control-label
// reference: mui-svg-icon
pub fn flag<'a>(
    label: impl Into<Cow<'a, str>>,
    held: bool,
    toggled: impl Fn(bool) -> Message + 'a,
    band: Band,
) -> Element<'a, Message> {
    let face: fn(&iced::Theme, iced::widget::button::Status, Band) -> iced::widget::button::Style =
        match held {
            true => style::check_ticked,
            false => style::check_blank,
        };
    let glyph = match held {
        true => Icon::CheckBox,
        false => Icon::CheckBoxOutlineBlank,
    };
    let ticked = button(icon::icon(glyph, typeface::CONTROL_GLYPH))
        .padding(style::drawn(space::CHECK_PAD.drawn(band)))
        .style(move |theme, status| face(theme, status, band))
        .on_press(toggled(!held));
    container(row![ticked, prose(label, typeface::BODY)].align_y(iced::Center))
        .padding(style::inset(space::CHECK_LABEL_MARGIN, band))
        .into()
}

/// `MuiButton` at `variant='contained'` and `size='large'`, no narrower than
/// the least width MUI draws one at.
// reference: mui-button
// reference: mui-button-large
// reference: mui-theme-button
pub fn contained<'a>(label: Text, press: Option<Message>, band: Band) -> Element<'a, Message> {
    let lettering = column![
        Space::new().width(style::drawn(space::CONTAINED_MIN.drawn(band))),
        container(tinted(
            strings::lookup(label),
            typeface::CONTAINED,
            typeface::CONTAINED_WEIGHT,
            typeface::BUTTON_LEADING,
            iced::widget::text::default,
        ))
        .center_x(Fill),
    ];
    let mut control = button(lettering)
        .padding(style::inset(space::CONTAINED_PAD, band))
        .style(move |theme, status| style::contained(theme, status, band));
    if let Some(message) = press {
        control = control.on_press(message);
    }
    control.into()
}

/// `MuiAlert` at `severity='success'`: its glyph before the sentence, on the
/// face MUI darkens out of the severity's own light shade.
// the glyph is the icon font's `check_circle_outline`, standing for the path
// MUI names `SuccessOutlined`
// reference: mui-alert
// reference: mui-alert-parts
// reference: mui-alert-icons
// reference: mui-alert-dark
// reference: mui-palette-success
// reference: mui-color-green
pub fn succeeded<'a>(sentence: Text, band: Band) -> Element<'a, Message> {
    let glyph = container(icon::tinted(
        Icon::CheckCircleOutline,
        typeface::ALERT_GLYPH,
        style::alert_glyph,
    ))
    .padding(style::inset(space::ALERT_GLYPH_PAD, band));
    let written = container(tinted(
        strings::lookup(sentence),
        typeface::BODY_2,
        typeface::Weight::Regular,
        typeface::BODY_2_LEADING,
        iced::widget::text::default,
    ))
    .padding(style::inset(space::ALERT_MESSAGE_PAD, band));
    container(row![glyph, written].align_y(iced::Center))
        .padding(style::inset(space::ALERT_PAD, band))
        .style(move |theme| style::alert_success(theme, band))
        .into()
}
