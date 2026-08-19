//! The modern layout's own controls, as the dashboard's react routes draw
//! them.
//!
//! These stand on `/dashboard` and every route beneath it, and on no other.

use std::borrow::Cow;

use iced::widget::{Space, button, column, container, stack, text_input};
use iced::{Element, Fill};

use super::{Choice, Showing, line, portion, pressed, prose, tinted};
use crate::app::Message;
use crate::icon::{self, Icon};
use crate::style::{self, Css, Layout, Share, Viewport, space, typeface};
use crate::text::{self as strings, Template, Text};

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
fn disc<'a>(glyph: Icon, layout: Layout) -> Element<'a, Message> {
    container(
        container(icon::tinted(
            glyph,
            typeface::CONTROL_GLYPH,
            style::on_avatar,
        ))
        .center_x(style::drawn(space::AVATAR.drawn(layout)))
        .center_y(style::drawn(space::AVATAR.drawn(layout)))
        .style(move |theme| style::avatar(theme, layout)),
    )
    .width(style::drawn(space::LIST_AVATAR_SLOT.drawn(layout)))
    .into()
}

/// The lines a row writes, at `MuiListItemText`'s own margins.
// reference: mui-list-item-text
// reference: task-last-ran
fn said<'a>(
    primary: Primary<'a>,
    beneath: Option<Beneath<'a>>,
    layout: Layout,
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
            Beneath::Running(share) => progress(share, layout),
        });
    }
    container(lines)
        .padding(style::inset(margin, layout))
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
pub fn row<'a>(row: Row<'a>, layout: Layout) -> Element<'a, Message> {
    let mut parts: Vec<Element<'a, Message>> = Vec::new();
    let mut pad = style::inset(space::LIST_ROW_PAD, layout);
    match row.lead {
        Some(Lead::Glyph(glyph)) => parts.push(
            container(icon::icon(glyph, typeface::LIST_ICON))
                .width(style::drawn(space::LIST_ICON_SLOT.drawn(layout)))
                .into(),
        ),
        Some(Lead::Avatar(glyph)) => parts.push(disc(glyph, layout)),
        Some(Lead::Nested) => {
            pad.left = style::drawn(space::DRAWER_NESTED.drawn(layout));
            parts.push(
                Space::new()
                    .width(style::drawn(space::LIST_ICON_SLOT.drawn(layout)))
                    .into(),
            );
        }
        None => {}
    }
    parts.push(said(row.primary, row.beneath, layout));
    if let Some(glyph) = row.within {
        parts.push(icon::icon(glyph, typeface::LIST_ICON));
    }
    if row.trailing.is_some() {
        pad.right += style::drawn(space::LIST_ROW_ACTION.drawn(layout));
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
        .padding(style::drawn(space::ICON_BUTTON_PAD.drawn(layout)))
        .style(move |theme, status| style::icon_button(theme, status, layout))
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
            .padding(
                iced::Padding::ZERO.right(style::drawn(space::LIST_ACTION_INSET.drawn(layout)))
            )
            .align_right(Fill)
            .center_y(Fill),
    ]
    .into()
}

/// `MuiList` on `background.paper`: its own padding around the rows it holds.
// reference: mui-list
pub fn listed<'a>(rows: impl IntoIterator<Item = Row<'a>>, layout: Layout) -> Element<'a, Message> {
    container(column(rows.into_iter().map(|held| row(held, layout))))
        .padding(style::inset(space::LIST_PAD, layout))
        .width(Fill)
        .style(style::list_surface)
        .into()
}

/// `MuiLinearProgress` at its own height: the bar filled to `share` on the
/// track MUI darkens out of the accent, and the track alone where nothing
/// reports a share.
// reference: mui-linear-progress
// reference: mui-linear-progress-bar
pub fn bar<'a>(share: Option<Share>, layout: Layout) -> Element<'a, Message> {
    let height = style::drawn(space::LINEAR_PROGRESS.drawn(layout));
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
pub fn progress<'a>(share: Option<Share>, layout: Layout) -> Element<'a, Message> {
    let mut held = iced::widget::row![
        container(bar(share, layout))
            .width(Fill)
            .center_y(style::drawn(space::TASK_PROGRESS_ROW.drawn())),
    ]
    .spacing(style::drawn(space::TASK_PROGRESS_GAP.drawn(layout)));
    if let Some(share) = share {
        held = held.push(prose(
            strings::format(Template::Percent, &[&format!("{:.0}", share.percent())]),
            typeface::BODY,
        ));
    }
    container(held.align_y(iced::Center))
        .width(style::drawn(space::TASK_PROGRESS_MIN.drawn(layout)))
        .padding(iced::Padding::ZERO.right(style::drawn(space::TASK_PROGRESS_TRAIL.drawn(layout))))
        .into()
}

/// `MuiPaper` at MUI's own default elevation, with `content` standing on it.
// reference: mui-paper
// reference: mui-paper-elevation
pub fn papered<'a>(content: Element<'a, Message>, layout: Layout) -> Element<'a, Message> {
    container(content)
        .style(move |theme| style::paper(theme, layout))
        .into()
}

/// What a card stands where its media goes: the image the server holds for it,
/// or the glyph it stands in with over the background its title picks.
#[derive(Debug, Clone)]
pub enum Media {
    Image(iced::widget::image::Handle),
    Glyph(Icon, style::Length),
}

/// One `BaseCard`: its media over the title, the line under that title, and
/// the overflow control at that line's trailing edge.
#[derive(Debug, Clone)]
pub struct Card<'a> {
    pub title: Cow<'a, str>,
    /// The line under the title, and nothing where the card writes none.
    pub text: Option<Cow<'a, str>>,
    pub media: Media,
    pub height: Css,
    /// What pressing the media sends, and nothing where the card reaches
    /// nothing.
    pub opens: Option<Message>,
    /// What the overflow control sends, and nothing where the card carries no
    /// such control.
    pub action: Option<Message>,
}

/// `BaseCard`: MUI's own paper at that height, the media filling what
/// `MuiCardContent` leaves it, the title elided to one line over
/// `gutterBottom`'s margin, and the second line in `body2` on the scheme's
/// secondary lettering.
// the media is cropped to its slot, which is what `backgroundSize: cover` does
// reference: base-card
// reference: mui-card
// reference: mui-card-content
// reference: mui-card-action-area
// reference: mui-card-media
// reference: mui-typography-gutter-bottom
// reference: mui-icon-button
pub fn card<'a>(card: Card<'a>, layout: Layout) -> Element<'a, Message> {
    let media: Element<'a, Message> = match card.media {
        Media::Image(handle) => container(
            iced::widget::image(handle)
                .width(Fill)
                .height(Fill)
                .content_fit(iced::ContentFit::Cover),
        )
        .width(Fill)
        .height(Fill)
        .into(),
        Media::Glyph(glyph, size) => {
            let background = style::scheme::card_background(&card.title);
            container(icon::icon(glyph, size))
                .center_x(Fill)
                .center_y(Fill)
                .style(move |theme| style::card_media(theme, background))
                .into()
        }
    };
    let lit = iced::widget::hover(
        media,
        container(Space::new())
            .width(Fill)
            .height(Fill)
            .style(style::card_highlight),
    );
    let area: Element<'a, Message> = match card.opens {
        None => lit,
        Some(press) => button(lit)
            .width(Fill)
            .height(Fill)
            .padding(iced::Padding::ZERO)
            .style(style::flat)
            .on_press(press)
            .into(),
    };

    let mut lines = column![
        container(line(
            card.title,
            typeface::BODY,
            typeface::Weight::Regular,
            typeface::BODY_1_LEADING,
        ))
        .padding(iced::Padding::ZERO.bottom(style::drawn(space::GUTTER_BOTTOM.drawn())))
    ];
    if let Some(text) = card.text {
        lines = lines.push(tinted(
            text,
            typeface::BODY_2,
            typeface::Weight::Regular,
            typeface::BODY_2_LEADING,
            style::muted,
        ));
    }
    let mut held = iced::widget::row![container(lines).width(Fill)];
    if let Some(press) = card.action {
        held = held.push(
            button(icon::icon(Icon::MoreVert, typeface::CONTROL_GLYPH))
                .padding(style::drawn(space::ICON_BUTTON_PAD.drawn(layout)))
                .style(move |theme, status| style::icon_button(theme, status, layout))
                .on_press(press),
        );
    }
    let content = container(stack![
        Space::new().height(style::drawn(space::CARD_CONTENT_MIN_INSIDE.drawn(layout))),
        held,
    ])
    .width(Fill)
    .padding(style::inset(space::CARD_CONTENT_PAD, layout));

    papered(
        column![container(area).width(Fill).height(Fill), content]
            .height(style::drawn(card.height.drawn(layout)))
            .into(),
        layout,
    )
}

/// `Grid container`: its cells laid across the count `cell`'s own ladder puts
/// in one row at this page, at the gutter the container's spacing leaves.
// reference: mui-grid
pub fn grid<'a>(
    cell: space::Cell,
    cells: impl IntoIterator<Item = Element<'a, Message>>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let across = cell.across(viewport).count();
    let gutter = style::drawn(space::CARD_GRID_GAP.drawn(viewport.layout()));
    let mut cells = cells.into_iter().peekable();
    column(std::iter::from_fn(move || {
        cells.peek()?;
        let mut laid: Vec<Element<'a, Message>> = cells.by_ref().take(across).collect();
        while laid.len() < across {
            laid.push(Space::new().width(Fill).into());
        }
        Some(iced::widget::row(laid).spacing(gutter).into())
    }))
    .spacing(gutter)
    .into()
}

/// Where a `MuiFormHelperText` stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Helper {
    /// Inside a filled control, where MUI insets it to that control's own
    /// text.
    Contained,
    /// Under a heading, where it stands at the page's own leading edge.
    Flush,
}

/// `MuiFormHelperText`: the sentence MUI writes under a control, in the
/// scheme's secondary lettering at `caption`'s line box and the size the
/// reference writes over it.
// reference: mui-form-helper-text
// reference: mui-theme-form-helper
// reference: mui-typography-caption
pub fn helper<'a>(sentence: Text, standing: Helper, layout: Layout) -> Element<'a, Message> {
    let margin = match standing {
        Helper::Contained => space::HELPER_CONTAINED_MARGIN,
        Helper::Flush => space::HELPER_MARGIN,
    };
    container(tinted(
        strings::lookup(sentence),
        typeface::HELPER,
        typeface::Weight::Regular,
        typeface::HELPER_LEADING,
        style::muted,
    ))
    .padding(style::inset(margin, layout))
    .into()
}

/// The control with `MuiFormHelperText`'s sentence under it, and the control
/// alone where the reference writes none.
fn beneath<'a>(
    control: Element<'a, Message>,
    helper: Option<Text>,
    layout: Layout,
) -> Element<'a, Message> {
    match helper {
        Some(sentence) => {
            column![control, self::helper(sentence, Helper::Contained, layout)].into()
        }
        None => control,
    }
}

/// A filled field's label shrunk into the head of its own face, and the rule
/// the field draws under its foot.
// reference: mui-filled-underline
// reference: mui-input-label
fn dressed<'a>(control: Element<'a, Message>, label: Text, layout: Layout) -> Element<'a, Message> {
    let shrunk = container(tinted(
        strings::lookup(label),
        typeface::FILLED_LABEL,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
        style::muted,
    ))
    .padding(style::inset(space::FILLED_LABEL_INSET, layout));
    column![
        stack![control, shrunk],
        container(Space::new())
            .width(Fill)
            .height(style::drawn(space::FILLED_RULE.drawn(layout)))
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
// reference: mui-input-adornment
// reference: mui-input-base
// reference: mui-input-label
pub fn field<'a>(
    label: Text,
    helper: Option<Text>,
    unit: Option<Text>,
    value: &str,
    edited: impl Fn(String) -> Message + 'a,
    layout: Layout,
) -> Element<'a, Message> {
    let typed = text_input("", value)
        .style(move |theme, status| style::filled(theme, status, layout))
        .size(style::drawn(typeface::BODY.drawn()))
        .line_height(style::leading(typeface::FILLED_LEADING))
        .padding(style::inset(space::FILLED_PAD, layout))
        .on_input(edited)
        .width(Fill);
    let control = match unit {
        Some(sentence) => stack![typed, adornment(sentence, layout)].into(),
        None => Element::from(typed),
    };
    beneath(dressed(control, label, layout), helper, layout)
}

/// `MuiInputAdornment` at a field's trailing edge: the unit the reference
/// writes beside the value, in the scheme's secondary lettering, at the margin
/// the adornment keeps from the value.
// reference: mui-input-adornment
// reference: scheme-secondary-text
fn adornment<'a>(sentence: Text, layout: Layout) -> Element<'a, Message> {
    container(tinted(
        strings::lookup(sentence),
        typeface::BODY,
        typeface::Weight::Regular,
        typeface::FILLED_LEADING,
        style::muted,
    ))
    .padding(
        style::inset(space::FILLED_PAD, layout)
            .right(style::drawn(space::FILLED_ADORNMENT_MARGIN.drawn(layout))),
    )
    .align_right(Fill)
    .center_y(Fill)
    .into()
}

/// The same field carrying the option standing rather than a typed value, the
/// chevron laid over its trailing edge, and the menu the options stand in.
// reference: mui-select-filled
// reference: mui-select-icon
// reference: mui-select-chevron
// reference: mui-menu-item
// reference: mui-menu-paper
pub fn chosen<'a, T>(
    label: Text,
    helper: Option<Text>,
    offered: Vec<Choice<T>>,
    held: &T,
    picked: impl Fn(T) -> Message + 'a,
    viewport: Viewport,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
{
    let layout = viewport.layout();
    let options = offered.len();
    for choice in &offered {
        crate::fonts::observed(&choice.label, typeface::Weight::Regular);
    }
    let standing = offered.iter().find(|choice| &choice.value == held).cloned();
    let field = iced::widget::pick_list(offered, standing, move |choice| picked(choice.value))
        .style(move |theme, status| style::filled_select(theme, status, layout))
        .menu_style(move |theme| style::filled_menu(theme, layout))
        .menu_height(style::drawn(space::menu_height(options, viewport)))
        .handle(iced::widget::pick_list::Handle::None)
        .font(style::font(typeface::Weight::Regular))
        .text_size(style::drawn(typeface::BODY.drawn()))
        .text_line_height(style::leading(typeface::FILLED_LEADING))
        .padding(style::inset(space::FILLED_SELECT_PAD, layout))
        .width(Fill);
    let chevron = container(icon::tinted(
        Icon::ArrowDropDown,
        typeface::CONTROL_GLYPH,
        style::chevron,
    ))
    .padding(iced::Padding::ZERO.right(style::drawn(space::FILLED_CHEVRON_INSET.drawn(layout))))
    .align_right(Fill)
    .center_y(Fill);
    beneath(
        dressed(stack![field, chevron].into(), label, layout),
        helper,
        layout,
    )
}

/// A box and the label beside it, at `MuiFormControlLabel`'s own margins and
/// `MuiCheckbox`'s own padding; the box is one glyph and carries no outline of
/// its own.
// reference: mui-checkbox
// reference: mui-switch-base
// reference: mui-form-control-label
// reference: mui-svg-icon
pub fn flag<'a>(
    label: Text,
    helper: Option<Text>,
    held: bool,
    toggled: impl Fn(bool) -> Message + 'a,
    layout: Layout,
) -> Element<'a, Message> {
    let face: fn(
        &iced::Theme,
        iced::widget::button::Status,
        Layout,
    ) -> iced::widget::button::Style = match held {
        true => style::check_ticked,
        false => style::check_blank,
    };
    let glyph = match held {
        true => Icon::CheckBox,
        false => Icon::CheckBoxOutlineBlank,
    };
    let ticked = button(icon::icon(glyph, typeface::CONTROL_GLYPH))
        .padding(style::drawn(space::CHECK_PAD.drawn(layout)))
        .style(move |theme, status| face(theme, status, layout))
        .on_press(toggled(!held));
    beneath(
        container(
            iced::widget::row![ticked, prose(strings::lookup(label), typeface::BODY)]
                .align_y(iced::Center),
        )
        .padding(style::inset(space::CHECK_LABEL_MARGIN, layout))
        .into(),
        helper,
        layout,
    )
}

/// `MuiButton` at `variant='contained'` and `size='large'`, no narrower than
/// the least width MUI draws one at.
// reference: mui-button
// reference: mui-button-large
// reference: mui-theme-button
pub fn contained<'a>(label: Text, press: Option<Message>, layout: Layout) -> Element<'a, Message> {
    let lettering = column![
        Space::new().width(style::drawn(space::CONTAINED_MIN_INSIDE.drawn(layout))),
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
        .padding(style::inset(space::CONTAINED_PAD, layout))
        .style(move |theme, status| style::contained(theme, status, layout));
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
pub fn succeeded<'a>(sentence: Text, layout: Layout) -> Element<'a, Message> {
    let glyph = container(icon::tinted(
        Icon::CheckCircleOutline,
        typeface::ALERT_GLYPH,
        style::alert_glyph,
    ))
    .padding(style::inset(space::ALERT_GLYPH_PAD, layout));
    let written = container(tinted(
        strings::lookup(sentence),
        typeface::BODY_2,
        typeface::Weight::Regular,
        typeface::BODY_2_LEADING,
        iced::widget::text::default,
    ))
    .padding(style::inset(space::ALERT_MESSAGE_PAD, layout));
    container(iced::widget::row![glyph, written].align_y(iced::Center))
        .padding(style::inset(space::ALERT_PAD, layout))
        .style(move |theme| style::alert_success(theme, layout))
        .into()
}
