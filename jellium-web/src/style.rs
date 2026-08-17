//! The one place the ported appearance values cross into iced.
//!
//! Every length below is a canvas length, because the canvas carries the band's
//! root size as its scale and so resolves every em once for the whole surface.

pub use jellium_model::appearance::{
    Band, Css, Dialog, Drawn, Length, Letters, Ratio, Screen, Share, Viewport, card, scheme, space,
    typeface,
};

/// The one site a ported length becomes a number iced takes.
pub fn drawn(length: Drawn) -> f32 {
    length.count()
}

pub fn color(color: scheme::Color) -> iced::Color {
    iced::Color::from_rgba8(
        color.red(),
        color.green(),
        color.blue(),
        color.alpha().fraction(),
    )
}

pub fn padding(padding: space::Padding) -> iced::Padding {
    iced::Padding {
        top: drawn(padding.top.drawn()),
        right: drawn(padding.right.drawn()),
        bottom: drawn(padding.bottom.drawn()),
        left: drawn(padding.left.drawn()),
    }
}

/// A padding the reference writes in css pixels, which the band's root
/// resolves once for the whole surface.
pub fn inset(inset: space::Inset, band: Band) -> iced::Padding {
    iced::Padding {
        top: drawn(inset.top.drawn(band)),
        right: drawn(inset.right.drawn(band)),
        bottom: drawn(inset.bottom.drawn(band)),
        left: drawn(inset.left.drawn(band)),
    }
}

/// The ported shadow as iced draws one; iced's own shadow carries no spread,
/// which is the one part of `space::Shadow` that does not cross.
pub fn shadow(shadow: space::Shadow) -> iced::Shadow {
    iced::Shadow {
        color: color(shadow.color),
        offset: iced::Vector::new(0.0, drawn(shadow.drop.drawn())),
        blur_radius: drawn(shadow.blur.drawn()),
    }
}

pub fn radius() -> iced::border::Radius {
    iced::border::Radius::new(drawn(space::RADIUS.drawn()))
}

/// The dark scheme as iced's own palette, whose six slots the reference fills
/// five of: it declares no success color, so the accent stands in that slot and
/// nothing this client draws reads it.
pub fn theme() -> iced::Theme {
    iced::Theme::Custom(std::sync::Arc::new(iced::theme::Custom::new(
        crate::text::lookup(crate::text::Text::AppName).to_owned(),
        iced::theme::Palette {
            background: color(scheme::BACKGROUND),
            text: color(scheme::TEXT),
            primary: color(scheme::ACCENT),
            success: color(scheme::ACCENT),
            warning: color(scheme::STAR),
            danger: color(scheme::ERROR),
        },
    )))
}

/// The family the reference's own base faces register under. Private because
/// `iced::font::Family::Name` is the foreign boundary that can carry only a
/// string and `font` is the one site that crosses it.
const FAMILY: &str = "Noto Sans";

pub fn font(weight: typeface::Weight) -> iced::Font {
    iced::Font {
        weight: match weight {
            typeface::Weight::Regular => iced::font::Weight::Normal,
            typeface::Weight::Bold => iced::font::Weight::Bold,
        },
        ..iced::Font::with_name(FAMILY)
    }
}

/// The family the reference's own icon face registers under. Private for the
/// reason `FAMILY` is: `ICONS` is the one site that crosses into iced with it.
// reference: icon-family
const ICON_FAMILY: &str = "Material Icons";

/// The face a glyph is drawn in.
pub const ICONS: iced::Font = iced::Font::with_name(ICON_FAMILY);

/// The line box a run of text stands in, which iced takes as a factor of the
/// size the run is drawn at.
pub fn leading(leading: typeface::Leading) -> iced::widget::text::LineHeight {
    match leading {
        typeface::Leading::Factor(factor) => {
            iced::widget::text::LineHeight::Relative(factor.factor())
        }
        typeface::Leading::Length(length) => {
            iced::widget::text::LineHeight::Absolute(drawn(length.drawn()).into())
        }
    }
}

/// The canvas scale the band draws at, which is what resolves every em.
pub fn scale(band: Band) -> f32 {
    band.root().factor()
}

/// The page itself, which the reference paints in the scheme's background and
/// draws every screen inside.
// reference: page-standalone
pub fn page(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::BACKGROUND))
        .color(color(scheme::TEXT))
}

/// The slot at the top of a page, which the reference leaves transparent over
/// the page's own background.
// reference: scheme-header-transparent
pub fn header(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::HEADER))
}

/// The surface a dialog stands on.
// reference: dialog-fullscreen
pub fn dialog(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::SURFACE))
        .color(color(scheme::TEXT))
        .border(iced::Border {
            radius: radius(),
            ..iced::Border::default()
        })
        .shadow(shadow(space::SHADOW))
}

/// The reference's `.toast`: its own surface, rounded tighter than a control
/// and carrying the shadow a raised surface carries.
// reference: scheme-toast
// reference: toast-face
pub fn toast(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::TOAST))
        .color(color(scheme::ON_TOAST))
        .border(iced::Border {
            radius: iced::border::Radius::new(drawn(space::TOAST_RADIUS.drawn())),
            ..iced::Border::default()
        })
        .shadow(shadow(space::SHADOW))
}

/// Every scrollbar the client draws: the reference's own thumb on its own
/// track, in one face at rest, under the pointer and under a drag.
/// The reference sets `scrollbar-color` and `scrollbar-width` on every
/// element, and both engines this client runs on drop the
/// `::-webkit-scrollbar-thumb` rule beside them where either standard property
/// is set, so the thumb is `#3b3b3b` on `#202020` and its corners are square.
// reference: scheme-scrollbar
pub fn scrollbar(
    _theme: &iced::Theme,
    _status: iced::widget::scrollable::Status,
) -> iced::widget::scrollable::Style {
    let rail = iced::widget::scrollable::Rail {
        background: Some(iced::Background::Color(color(scheme::SCROLLBAR_TRACK))),
        border: iced::Border::default(),
        scroller: iced::widget::scrollable::Scroller {
            background: iced::Background::Color(color(scheme::SCROLLBAR_THUMB)),
            border: iced::Border::default(),
        },
    };
    iced::widget::scrollable::Style {
        container: iced::widget::container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: iced::widget::scrollable::AutoScroll {
            background: iced::Background::Color(iced::Color::TRANSPARENT),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            icon: iced::Color::TRANSPARENT,
        },
    }
}

/// What a dialog is drawn over, which the reference paints black behind its
/// own backdrop.
// reference: scheme-dialog-backdrop
pub fn scrim(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::DIALOG_BACKDROP))
}

/// The background a screen drawn over the video element carries.
pub fn over_video(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::BACKGROUND))
}

/// `.videoOsdBottom`'s scrim, which fades upward from the page's own
/// background into nothing.
// reference: osd-bottom
pub fn osd_bottom(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(fading(iced::Degrees(0.0)))
        .color(color(scheme::ON_OSD))
}

/// `.osdHeader`'s scrim, the same gradient the other way up.
// reference: osd-header
pub fn osd_header(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(fading(iced::Degrees(180.0)))
        .color(color(scheme::ON_OSD_HEADER))
}

/// A scrim's own gradient: the near end at the angle given, fading into the
/// same color at no opacity.
// reference: osd-bottom
fn fading(angle: iced::Degrees) -> iced::Background {
    iced::Background::Gradient(iced::Gradient::Linear(
        iced::gradient::Linear::new(angle)
            .add_stop(0.0, color(scheme::SCRIM))
            .add_stop(1.0, color(scheme::SCRIM.at(scheme::Alpha::CLEAR))),
    ))
}

/// Whether a control is drawing its resting face or the one the reference gives
/// it under the pointer, under the keyboard focus ring, or pressed.
fn lit(status: iced::widget::button::Status) -> bool {
    match status {
        iced::widget::button::Status::Active | iced::widget::button::Status::Disabled => false,
        iced::widget::button::Status::Hovered | iced::widget::button::Status::Pressed => true,
    }
}

/// A face in the scheme's own colors, rounded the way every control here is.
fn faced(background: scheme::Color, text: scheme::Color) -> iced::widget::button::Style {
    iced::widget::button::Style {
        background: Some(iced::Background::Color(color(background))),
        text_color: color(text),
        border: iced::Border {
            radius: radius(),
            ..iced::Border::default()
        },
        ..iced::widget::button::Style::default()
    }
}

/// The reference's `.raised`, whose focus face is the accent.
// reference: scheme-raised
// reference: scheme-focus
pub fn raised(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::ACCENT, scheme::ON_ACCENT);
    }
    faced(scheme::RAISED, scheme::ON_RAISED)
}

/// The reference's `.button-submit`.
// reference: scheme-submit
pub fn submit(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::ACCENT_FOCUS, scheme::ON_ACCENT);
    }
    faced(scheme::ACCENT, scheme::ON_ACCENT)
}

/// The reference's `.button-delete`, which it gives the control that removes
/// something and lights no differently where it is reached.
// reference: scheme-delete
pub fn destructive(
    _theme: &iced::Theme,
    _status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    faced(scheme::DELETE, scheme::ON_DELETE)
}

/// A control carrying no face of its own until it is reached, which is what a
/// card, a row and an icon button are.
// reference: scheme-list-state
pub fn flat(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::LIST_HOVER, scheme::TEXT);
    }
    iced::widget::button::Style {
        text_color: color(scheme::TEXT),
        ..iced::widget::button::Style::default()
    }
}

/// The reference's `.paper-icon-button-light`: a disc carrying a glyph and no
/// face of its own until it is reached.
// reference: control-icon-button
// reference: control-icon-glyph
// reference: scheme-list-state
pub fn icon_control(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    let disc = iced::border::Radius::new(drawn(
        typeface::ICON_BUTTON
            .plus(space::ICON_BUTTON_PAD)
            .plus(space::ICON_BUTTON_PAD)
            .times(Ratio::thousandths(500))
            .drawn(),
    ));
    let face = match lit(status) {
        true => faced(scheme::LIST_HOVER, scheme::TEXT),
        false => iced::widget::button::Style {
            text_color: color(scheme::TEXT),
            ..iced::widget::button::Style::default()
        },
    };
    iced::widget::button::Style {
        border: iced::Border {
            radius: disc,
            ..face.border
        },
        ..face
    }
}

/// A control that reads as one of the reference's anchors: its accent lettering
/// on no face of its own, brightening where it is reached.
// reference: scheme-anchors
pub fn link(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: match lit(status) {
            true => color(scheme::ACCENT_FOCUS),
            false => color(scheme::ACCENT),
        },
        ..iced::widget::button::Style::default()
    }
}

/// The surface the navigation drawer stands on, which is the scheme's own
/// paper.
// reference: drawer-paper
// reference: scheme-anchors
pub fn drawer(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::SURFACE))
        .color(color(scheme::TEXT))
}

/// A drawer entry whose screen the dashboard is not showing: no face of its
/// own until it is reached.
// reference: drawer-entry
// reference: mui-dark-action
pub fn drawer_offered(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::ACTION_HOVER, scheme::TEXT);
    }
    iced::widget::button::Style {
        text_color: color(scheme::TEXT),
        ..iced::widget::button::Style::default()
    }
}

/// The entry whose screen the dashboard is showing: the accent at a fifth of
/// its opacity, and at twenty-eight hundredths where it is reached.
// reference: drawer-entry
pub fn drawer_shown(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::DRAWER_SHOWN_HOVER, scheme::TEXT);
    }
    faced(scheme::DRAWER_SHOWN, scheme::TEXT)
}

/// A tab the strip is not showing: the reference's own grey lettering on no
/// face of its own, in the accent where it is reached.
// reference: scheme-tab
pub fn tab_offered(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: match lit(status) {
            true => color(scheme::ACCENT),
            false => color(scheme::TAB_OFFERED),
        },
        ..iced::widget::button::Style::default()
    }
}

/// The tab whose body the strip is showing: the reference's white lettering,
/// in the accent where it is reached, its hover outranking its shown face.
// reference: scheme-tab
pub fn tab_shown(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        text_color: match lit(status) {
            true => color(scheme::ACCENT),
            false => color(scheme::TAB_SHOWN),
        },
        ..iced::widget::button::Style::default()
    }
}

/// Which end of a `.localnav` group a control stands at, which is where the
/// group's own radius falls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ends {
    Leading,
    Trailing,
    Both,
    Neither,
}

/// The group's radius where `ends` puts it and nothing anywhere else.
// reference: control-localnav-group
// reference: mui-toggle-group
fn rounded(ends: Ends, corner: Drawn) -> iced::border::Radius {
    let corner = drawn(corner);
    let (leading, trailing) = match ends {
        Ends::Leading => (corner, 0.0),
        Ends::Trailing => (0.0, corner),
        Ends::Both => (corner, corner),
        Ends::Neither => (0.0, 0.0),
    };
    iced::border::Radius::default()
        .top_left(leading)
        .bottom_left(leading)
        .top_right(trailing)
        .bottom_right(trailing)
}

/// A control of a `.localnav` group whose screen the group is not showing.
/// The reference gives it no face under the pointer.
// reference: control-localnav
// reference: control-localnav-group
pub fn localnav_offered(
    _theme: &iced::Theme,
    _status: iced::widget::button::Status,
    ends: Ends,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        border: iced::Border {
            radius: rounded(ends, space::LOCALNAV_RADIUS.drawn()),
            ..iced::Border::default()
        },
        ..faced(scheme::LOCALNAV, scheme::TEXT)
    }
}

/// The control whose screen the group is showing, which the reference does not
/// light either.
// reference: control-localnav-group
// reference: scheme-localnav-active
pub fn localnav_shown(
    _theme: &iced::Theme,
    _status: iced::widget::button::Status,
    ends: Ends,
) -> iced::widget::button::Style {
    iced::widget::button::Style {
        border: iced::Border {
            radius: rounded(ends, space::LOCALNAV_RADIUS.drawn()),
            ..iced::Border::default()
        },
        ..faced(scheme::LOCALNAV_SHOWN, scheme::ON_LOCALNAV_SHOWN)
    }
}

/// The surface `MRT_TablePaper` stands the table on, which is the scheme's own
/// paper, with the lettering MUI writes on it.
// reference: table-paper
// reference: table-activity-face
pub fn table(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::SURFACE))
        .color(color(scheme::ON_SURFACE))
}

/// The rule every table cell draws under itself.
// reference: mui-table-cell
pub fn table_rule(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::TABLE_RULE))
}

/// The edge one segment of the toolbar's group carries, and the radius the
/// group carries at its two ends alone. The radius is written in css pixels,
/// so the band resolves it.
// reference: mui-toggle-button
// reference: mui-toggle-group
fn toggle_edge(ends: Ends, band: Band) -> iced::Border {
    iced::Border {
        color: color(scheme::DIVIDER),
        width: drawn(space::TOGGLE_BORDER.drawn(band)),
        radius: rounded(ends, space::TOGGLE_RADIUS.drawn(band)),
    }
}

/// A segment of the toolbar's group whose view the screen is not showing: no
/// face of its own until it is reached, inside MUI's own divider.
// reference: mui-toggle-button
// reference: mui-toggle-group
pub fn toggle_offered(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
    ends: Ends,
    band: Band,
) -> iced::widget::button::Style {
    let face = match lit(status) {
        true => faced(scheme::ACTION_HOVER, scheme::ON_SURFACE),
        false => iced::widget::button::Style {
            text_color: color(scheme::ON_SURFACE),
            ..iced::widget::button::Style::default()
        },
    };
    iced::widget::button::Style {
        border: toggle_edge(ends, band),
        ..face
    }
}

/// The segment whose view it is showing: white at a fifth of its opacity, and
/// at twenty-eight hundredths where it is reached.
// reference: mui-toggle-button
// reference: mui-toggle-group
pub fn toggle_shown(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
    ends: Ends,
    band: Band,
) -> iced::widget::button::Style {
    let face = match lit(status) {
        true => faced(scheme::TOGGLE_SHOWN_HOVER, scheme::ON_SURFACE),
        false => faced(scheme::TOGGLE_SHOWN, scheme::ON_SURFACE),
    };
    iced::widget::button::Style {
        border: toggle_edge(ends, band),
        ..face
    }
}

/// The rule `.listItem-border` draws under a row.
// reference: scheme-list-border
pub fn list_rule(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::LIST_RULE))
}

/// A node of the metadata sidebar the manager is not showing: no face of its
/// own until it is reached, and the reference's own blue with white lettering
/// there.
// reference: metadata-tree
pub fn tree_offered(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    if lit(status) {
        return faced(scheme::TREE_HOVER, scheme::ON_TREE_HOVER);
    }
    iced::widget::button::Style {
        text_color: color(scheme::TEXT),
        ..iced::widget::button::Style::default()
    }
}

/// The node whose part the manager is showing, which the reference paints in
/// the accent whether or not it is reached.
// reference: metadata-tree
pub fn tree_shown(
    _theme: &iced::Theme,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    match lit(status) {
        true => faced(scheme::TREE_SHOWN, scheme::ON_TREE_HOVER),
        false => faced(scheme::TREE_SHOWN, scheme::TEXT),
    }
}

/// The rule down the metadata sidebar's trailing edge.
// reference: metadata-sidebar
pub fn editor_rule(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().background(color(scheme::EDITOR_RULE))
}

/// The frame a card's image sits in, which the reference fills with
/// `.cardPadder`'s own color behind an image that has not arrived.
// reference: card-container
pub fn card_padder(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::CARD_PADDER))
        .border(iced::Border {
            radius: radius(),
            ..iced::Border::default()
        })
        .shadow(shadow(space::SHADOW))
}

/// A card drawing no image, which the reference gives one of five backgrounds
/// chosen from the item's own name.
// reference: card-container
// reference: card-shadow
pub fn card_face(
    _theme: &iced::Theme,
    background: scheme::Color,
) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(background))
        .border(iced::Border {
            radius: radius(),
            ..iced::Border::default()
        })
        .shadow(shadow(space::SHADOW))
}

/// What a card writes under its image.
// reference: card-footer
pub fn card_footer(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default().color(color(scheme::TEXT))
}

/// The mark laid on a control that is narrowing what is shown: the reference's
/// own face under a radius of the mark's own diameter, which is the circle its
/// `border-radius: 100em` draws.
// reference: filter-indicator
// reference: filter-indicator-face
pub fn indicator(_theme: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color(scheme::INDICATOR))
        .color(color(scheme::ON_ACCENT))
        .border(iced::Border {
            radius: iced::border::Radius::new(drawn(space::INDICATOR.drawn())),
            ..iced::Border::default()
        })
}

/// The label written above a field.
// reference: scheme-label
pub fn label(_theme: &iced::Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(color(scheme::LABEL)),
    }
}

/// The description written under a field.
// reference: scheme-secondary-text
pub fn description(_theme: &iced::Theme) -> iced::widget::text::Style {
    iced::widget::text::Style {
        color: Some(color(scheme::TEXT_SECONDARY)),
    }
}

/// The reference's `.emby-select-withcolor`: its own face inside its own edge,
/// that edge in the accent while the field is reached or open.
// reference: scheme-select
// reference: scheme-select-focus
// reference: control-select-withcolor
pub fn select(
    _theme: &iced::Theme,
    status: iced::widget::pick_list::Status,
) -> iced::widget::pick_list::Style {
    let edge = match status {
        iced::widget::pick_list::Status::Active => scheme::SELECT,
        iced::widget::pick_list::Status::Hovered
        | iced::widget::pick_list::Status::Opened { .. } => scheme::ACCENT,
    };
    iced::widget::pick_list::Style {
        text_color: color(scheme::TEXT),
        placeholder_color: color(scheme::TEXT_SECONDARY),
        handle_color: color(scheme::TEXT),
        background: iced::Background::Color(color(scheme::SELECT)),
        border: iced::Border {
            color: color(edge),
            width: drawn(space::SELECT_BORDER.drawn()),
            radius: radius(),
        },
    }
}

// the reference hands its options to the browser's own popup, which this
// canvas cannot raise, so the list is drawn here on `option`'s own face
// the row under the pointer takes the face the reference gives a reached
// `.listItem`, the browser deciding that face in the reference
// reference: scheme-select-option
// reference: scheme-list-state
pub fn menu(_theme: &iced::Theme) -> iced::widget::overlay::menu::Style {
    iced::widget::overlay::menu::Style {
        background: iced::Background::Color(color(scheme::SELECT_OPTION)),
        border: iced::Border {
            color: color(scheme::SELECT),
            width: drawn(space::SELECT_BORDER.drawn()),
            radius: radius(),
        },
        text_color: color(scheme::TEXT),
        selected_text_color: color(scheme::TEXT),
        selected_background: iced::Background::Color(color(scheme::LIST_HOVER)),
        shadow: iced::Shadow::default(),
    }
}

/// The reference's `.checkboxOutline`: an edge in the page's own lettering
/// while it is unchecked, the accent filling it when it is, and white at its
/// edge where a checked box is reached.
// reference: scheme-checkbox
// reference: scheme-checkbox-outline
// reference: control-checkbox
pub fn checkbox(
    _theme: &iced::Theme,
    status: iced::widget::checkbox::Status,
) -> iced::widget::checkbox::Style {
    let (filled, edge) = match status {
        iced::widget::checkbox::Status::Active { is_checked: true }
        | iced::widget::checkbox::Status::Disabled { is_checked: true } => {
            (Some(scheme::ACCENT), scheme::ACCENT)
        }
        iced::widget::checkbox::Status::Active { is_checked: false }
        | iced::widget::checkbox::Status::Disabled { is_checked: false } => (None, scheme::TEXT),
        iced::widget::checkbox::Status::Hovered { is_checked: true } => {
            (Some(scheme::ACCENT), scheme::CHECKBOX_FOCUS)
        }
        iced::widget::checkbox::Status::Hovered { is_checked: false } => (None, scheme::ACCENT),
    };
    iced::widget::checkbox::Style {
        background: match filled {
            Some(face) => iced::Background::Color(color(face)),
            None => iced::Background::Color(iced::Color::TRANSPARENT),
        },
        icon_color: color(scheme::ON_CHECKBOX),
        border: iced::Border {
            color: color(edge),
            width: drawn(space::CHECKBOX_BORDER.drawn()),
            radius: iced::border::Radius::new(drawn(space::CHECKBOX_RADIUS.drawn())),
        },
        text_color: Some(color(scheme::TEXT)),
    }
}

// reference: scheme-input
pub fn input(
    _theme: &iced::Theme,
    status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let edge = match status {
        iced::widget::text_input::Status::Focused { .. } => scheme::ACCENT,
        iced::widget::text_input::Status::Active
        | iced::widget::text_input::Status::Hovered
        | iced::widget::text_input::Status::Disabled => scheme::INPUT,
    };
    iced::widget::text_input::Style {
        background: iced::Background::Color(color(scheme::INPUT)),
        border: iced::Border {
            color: color(edge),
            width: drawn(space::INPUT_BORDER.drawn()),
            radius: radius(),
        },
        icon: color(scheme::TEXT_SECONDARY),
        placeholder: color(scheme::TEXT_SECONDARY),
        value: color(scheme::TEXT),
        selection: color(scheme::ACCENT),
    }
}
