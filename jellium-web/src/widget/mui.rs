//! MUI's own controls as the dashboard's react routes draw them.

use std::borrow::Cow;

use iced::widget::{Space, button, column, container, stack, text_input};
use iced::{Element, Fill};

use super::{Choice, Showing, portion, pressed, prose, tinted};
use crate::app::Message;
use crate::icon::{self, Icon};
use crate::style::{self, Band, Share, Viewport, space, typeface};
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

/// What stands before a row's own text.
#[derive(Debug, Clone)]
pub enum Lead {
    /// `MuiListItemIcon`'s slot with the glyph standing in it.
    Glyph(Icon),
    /// `MuiListItemAvatar`'s slot with `MuiAvatar`'s disc and the glyph on it.
    Avatar(Icon),
    /// No glyph at all: the row stands its own leading edge at the inset a
    /// drawer group's rows stand at, and its text at `MuiListItemText`'s own
    /// `inset` inside that.
    Nested,
}

/// The line a row writes its title in.
#[derive(Debug, Clone)]
pub enum Primary<'a> {
    /// `MuiListItemText`'s own line, which MUI writes in `body1`.
    Said(Cow<'a, str>),
    /// The heading level a row asks for instead.
    Headed(typeface::Rank, Cow<'a, str>),
}

/// What a row writes under that title, which MUI writes in `text.secondary`.
#[derive(Debug, Clone)]
pub enum Beneath<'a> {
    /// A second `body1` line.
    Said(Cow<'a, str>),
    /// `TaskLastRan`'s own line, at the line box the reference writes it in.
    Ran(Cow<'a, str>),
    /// `TaskProgress`: the bar at how far a task has got with the reading
    /// beside it, and the bar alone where the task reports no share.
    Running(Option<Share>),
}

/// `MuiListItem`'s own `secondaryAction`, which stands outside the row's own
/// control at the list's own inset.
#[derive(Debug, Clone)]
pub struct Trailing {
    pub glyph: Icon,
    /// What the control names itself, and nothing where the reference writes
    /// no title over it.
    pub label: Option<Text>,
    pub press: Message,
}

/// One `MuiListItem`.
#[derive(Debug, Clone)]
pub struct Row<'a> {
    pub lead: Option<Lead>,
    pub primary: Primary<'a>,
    pub beneath: Option<Beneath<'a>>,
    /// The glyph the row's own control carries after its text.
    pub within: Option<Icon>,
    /// Whether the list is showing what this row names, and nothing where the
    /// row is not a control at all.
    pub showing: Option<Showing>,
    pub trailing: Option<Trailing>,
}

/// The disc `MuiListItemAvatar` stands in its own slot, with the glyph on it.
// reference: mui-avatar
// reference: mui-list-item-avatar
fn disc<'a>(glyph: Icon, band: Band) -> Element<'a, Message> {
    container(
        container(icon::tinted(
            glyph,
            typeface::CONTROL_GLYPH,
            style::on_avatar,
        ))
        .center_x(style::drawn(space::AVATAR.drawn(band)))
        .center_y(style::drawn(space::AVATAR.drawn(band)))
        .style(move |theme| style::avatar(theme, band)),
    )
    .width(style::drawn(space::LIST_AVATAR_SLOT.drawn(band)))
    .into()
}

/// The lines a row writes, at `MuiListItemText`'s own margins.
// reference: mui-list-item-text
// reference: task-last-ran
fn said<'a>(
    primary: Primary<'a>,
    beneath: Option<Beneath<'a>>,
    band: Band,
) -> Element<'a, Message> {
    let title = match primary {
        Primary::Said(content) => prose(content, typeface::BODY),
        Primary::Headed(rank, content) => heading(rank, content),
    };
    let margin = match beneath {
        Some(_) => space::LIST_TEXT_MARGIN_STACKED,
        None => space::LIST_TEXT_MARGIN,
    };
    let mut lines = column![title];
    if let Some(under) = beneath {
        lines = lines.push(match under {
            Beneath::Said(content) => tinted(
                content,
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::BODY_1_LEADING,
                style::muted,
            ),
            Beneath::Ran(content) => tinted(
                content,
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::TASK_LAST_RAN_LEADING,
                style::muted,
            ),
            Beneath::Running(share) => progress(share, band),
        });
    }
    container(lines)
        .padding(style::inset(margin, band))
        .width(Fill)
        .into()
}

/// One `MuiListItem` at `MuiListItemButton`'s own padding, with
/// `MuiListItemText`'s own margins around its lines and no rule under it: the
/// accent where the list is showing what it names, MUI's own overlay where it
/// is reached, and no face of its own where it is not a control.
// reference: mui-list-item
// reference: mui-list-item-button
// reference: mui-list-item-text
// reference: mui-list-item-icon
// reference: mui-list-item-avatar
// reference: mui-list-secondary-action
// reference: dashboard-list-icon-slot
// reference: drawer-server
pub fn row<'a>(row: Row<'a>, band: Band) -> Element<'a, Message> {
    let mut parts: Vec<Element<'a, Message>> = Vec::new();
    let mut pad = style::inset(space::LIST_ROW_PAD, band);
    match row.lead {
        Some(Lead::Glyph(glyph)) => parts.push(
            container(icon::icon(glyph, typeface::LIST_ICON))
                .width(style::drawn(space::LIST_ICON_SLOT.drawn(band)))
                .into(),
        ),
        Some(Lead::Avatar(glyph)) => parts.push(disc(glyph, band)),
        Some(Lead::Nested) => {
            pad.left = style::drawn(space::DRAWER_NESTED.drawn(band));
            parts.push(
                Space::new()
                    .width(style::drawn(space::LIST_ICON_SLOT.drawn(band)))
                    .into(),
            );
        }
        None => {}
    }
    parts.push(said(row.primary, row.beneath, band));
    if let Some(glyph) = row.within {
        parts.push(icon::icon(glyph, typeface::LIST_ICON));
    }
    if row.trailing.is_some() {
        pad.right += style::drawn(space::LIST_ROW_ACTION.drawn(band));
    }

    let held = iced::widget::row(parts).align_y(iced::Center);
    let standing: Element<'a, Message> = match row.showing {
        Some(showing) => {
            let face: fn(
                &iced::Theme,
                iced::widget::button::Status,
            ) -> iced::widget::button::Style = match showing {
                Showing::Shown => style::list_row_selected,
                Showing::Offered(_) => style::list_row,
            };
            button(held)
                .width(Fill)
                .padding(pad)
                .style(face)
                .on_press(pressed(showing))
                .into()
        }
        None => container(held).width(Fill).padding(pad).into(),
    };

    let Some(trailing) = row.trailing else {
        return standing;
    };
    let control = button(icon::icon(trailing.glyph, typeface::CONTROL_GLYPH))
        .padding(style::drawn(space::ICON_BUTTON_PAD.drawn(band)))
        .style(move |theme, status| style::icon_button(theme, status, band))
        .on_press(trailing.press);
    let named: Element<'a, Message> = match trailing.label {
        Some(label) => iced::widget::tooltip(
            control,
            prose(strings::lookup(label), typeface::BODY),
            iced::widget::tooltip::Position::Top,
        )
        .style(style::dialog)
        .into(),
        None => control.into(),
    };
    stack![
        standing,
        container(named)
            .padding(iced::Padding::ZERO.right(style::drawn(space::LIST_ACTION_INSET.drawn(band))))
            .align_right(Fill)
            .center_y(Fill),
    ]
    .into()
}

/// `MuiList` on `background.paper`: its own padding around the rows it holds.
// reference: mui-list
pub fn listed<'a>(rows: impl IntoIterator<Item = Row<'a>>, band: Band) -> Element<'a, Message> {
    container(column(rows.into_iter().map(|held| row(held, band))))
        .padding(style::inset(space::LIST_PAD, band))
        .width(Fill)
        .style(style::list_surface)
        .into()
}

/// `MuiLinearProgress` at its own height: the bar filled to `share` on the
/// track MUI darkens out of the accent, and the track alone where nothing
/// reports a share.
// reference: mui-linear-progress
// reference: mui-linear-progress-bar
pub fn bar<'a>(share: Option<Share>, band: Band) -> Element<'a, Message> {
    let height = style::drawn(space::LINEAR_PROGRESS.drawn(band));
    let standing: Element<'a, Message> = match share {
        Some(share) => iced::widget::row![
            container(Space::new())
                .width(iced::Length::FillPortion(portion(share)))
                .height(height)
                .style(style::progress_bar),
            Space::new().width(iced::Length::FillPortion(portion(Share::WHOLE.less(share)))),
        ]
        .into(),
        None => Space::new().height(height).into(),
    };
    container(standing)
        .width(Fill)
        .height(height)
        .style(style::progress_track)
        .into()
}

/// `TaskProgress`: that bar held to its own row and its least width, with the
/// reading beside it where a share is reported.
// reference: task-progress
pub fn progress<'a>(share: Option<Share>, band: Band) -> Element<'a, Message> {
    let mut held = iced::widget::row![
        container(bar(share, band))
            .width(Fill)
            .center_y(style::drawn(space::TASK_PROGRESS_ROW.drawn())),
    ]
    .spacing(style::drawn(space::TASK_PROGRESS_GAP.drawn(band)));
    if let Some(share) = share {
        held = held.push(prose(
            strings::format(Text::Percent, &[&format!("{:.0}", share.percent())]),
            typeface::BODY,
        ));
    }
    container(held.align_y(iced::Center))
        .width(style::drawn(space::TASK_PROGRESS_MIN.drawn(band)))
        .padding(iced::Padding::ZERO.right(style::drawn(space::TASK_PROGRESS_TRAIL.drawn(band))))
        .into()
}

/// `MuiPaper` at MUI's own default elevation, with `content` standing on it.
// reference: mui-paper
// reference: mui-paper-elevation
pub fn papered<'a>(content: Element<'a, Message>, band: Band) -> Element<'a, Message> {
    container(content)
        .style(move |theme| style::paper(theme, band))
        .into()
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
    container(iced::widget::row![ticked, prose(label, typeface::BODY)].align_y(iced::Center))
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
        Space::new().width(style::drawn(space::CONTAINED_MIN_INSIDE.drawn(band))),
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
    container(iced::widget::row![glyph, written].align_y(iced::Center))
        .padding(style::inset(space::ALERT_PAD, band))
        .style(move |theme| style::alert_success(theme, band))
        .into()
}
