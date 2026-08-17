//! The static page's stylesheet, rendered from the ported values so the boot
//! page and the canvas cannot disagree about a color without one of them
//! saying so.

use super::{Length, scheme, space, typeface};

/// A design length as css writes it, in the em the reference wrote it in.
fn em(length: Length) -> String {
    format!("{}em", super::trimmed(length.drawn().count() / super::BASE))
}

/// The whole text of `jellium-web/boot.css`.
pub fn boot() -> String {
    format!(
        "#jellium-boot {{
  position: fixed;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-family: system-ui, sans-serif;
  line-height: {line_height};
  background: {background};
  color: {text};
  z-index: 4;
}}

#jellium-boot[data-state='failed'] {{
  color: {error};
}}

#jellium-boot[hidden] {{
  display: none;
}}

#jellium-boot-logo {{
  width: 30%;
  height: 30%;
  background-image: url('/branding/icon-transparent.png');
  background-position: center center;
  background-repeat: no-repeat;
  background-size: contain;
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}}

@media screen and (min-device-width: 992px) {{
  #jellium-boot-logo {{
    background-image: url('/branding/banner-light.png');
  }}
}}

#jellium-boot-retry {{
  margin-top: {block_gap};
  padding: {button_pad_block} {button_pad_inline};
  font: inherit;
  font-weight: 600;
  color: {on_raised};
  background: {raised};
  border: 0;
  border-radius: {radius};
  cursor: pointer;
}}

#jellium-boot-retry:focus {{
  background: {raised_focus};
}}

#jellium-boot-retry[hidden] {{
  display: none;
}}

canvas {{
  position: fixed;
  inset: 0;
  z-index: 2;
}}

[data-overlay] {{
  position: fixed;
  inset: 0;
  width: 100%;
  height: 100%;
  border: 0;
  background: {backdrop};
  object-fit: contain;
  pointer-events: none;
}}

[data-overlay][data-stack='below'] {{
  z-index: 0;
}}

[data-overlay][data-stack='above'] {{
  z-index: 3;
}}

[data-overlay][data-pointer] {{
  pointer-events: auto;
}}

[data-overlay][hidden] {{
  display: none;
}}

html[data-idle] canvas {{
  cursor: none;
}}
",
        line_height = super::trimmed(typeface::LINE_HEIGHT.factor()),
        background = scheme::BACKGROUND.css(),
        text = scheme::TEXT.css(),
        error = scheme::ERROR.css(),
        block_gap = em(space::BLOCK_GAP),
        button_pad_block = em(space::BUTTON_PAD.top),
        button_pad_inline = em(space::BUTTON_PAD.right),
        on_raised = scheme::ON_RAISED.css(),
        raised = scheme::RAISED.css(),
        radius = em(space::RADIUS),
        raised_focus = scheme::RAISED_FOCUS.css(),
        backdrop = scheme::DIALOG_BACKDROP.css(),
    )
}
