#![allow(dead_code)]
/// Shared design tokens for the Blue Blood Sports UI.
///
/// All raw color literals, font sizes, spacing values, and container style
/// functions live here. UI modules import from this module rather than
/// hard-coding values inline, so visual changes can be made in one place.
use iced::widget::container::Style as CStyle;
use iced::{Background, Border, Color, Font, Shadow, Vector};

// ---------------------------------------------------------------------------
// Background colors — ordered darkest → lightest
// ---------------------------------------------------------------------------

/// Outermost app shell — the near-black backdrop behind every screen.
pub const BG_APP:     Color = Color { r: 0.05, g: 0.05, b: 0.07, a: 1.0 };
/// File-picker & rival-picker background — slightly elevated over BG_APP.
pub const BG_PICKER:  Color = Color { r: 0.07, g: 0.07, b: 0.11, a: 1.0 };
/// Alternating row A / flat panel background.
pub const BG_PANEL:   Color = Color { r: 0.09, g: 0.09, b: 0.13, a: 1.0 };
/// Header bar and tab-bar background.
pub const BG_HEADER:  Color = Color { r: 0.10, g: 0.10, b: 0.13, a: 1.0 };
/// Sidebar / list-panel background.
pub const BG_SIDEBAR: Color = Color { r: 0.10, g: 0.10, b: 0.14, a: 1.0 };
/// Alternating row B — subtly lighter than BG_PANEL.
pub const BG_ROW_B:   Color = Color { r: 0.11, g: 0.11, b: 0.15, a: 1.0 };
/// Foreground card surface (main menu, new-game wizard, league builder).
pub const BG_CARD:    Color = Color { r: 0.11, g: 0.11, b: 0.16, a: 1.0 };
/// Section-divider / sub-header strip.
pub const BG_DIVIDER: Color = Color { r: 0.13, g: 0.13, b: 0.18, a: 1.0 };
/// Highlighted / selected list row.
pub const BG_SELECTED: Color = Color { r: 0.18, g: 0.22, b: 0.35, a: 1.0 };
/// Translucent backdrop drawn behind modal overlays.
pub const BG_OVERLAY: Color = Color { r: 0.0,  g: 0.0,  b: 0.0,  a: 0.65 };

// ---------------------------------------------------------------------------
// Border colors
// ---------------------------------------------------------------------------

/// Subtle border used on the foreground card.
pub const BORDER_CARD:  Color = Color { r: 0.20, g: 0.20, b: 0.28, a: 1.0 };
/// Stronger border used on dialog panels (file picker, etc.).
pub const BORDER_PANEL: Color = Color { r: 0.25, g: 0.25, b: 0.35, a: 1.0 };

// ---------------------------------------------------------------------------
// Text / label colors
// ---------------------------------------------------------------------------

/// De-emphasised label text — used for captions, field labels, hints.
pub const TEXT_DIM: Color = Color { r: 0.55, g: 0.55, b: 0.60, a: 1.0 };

// ---------------------------------------------------------------------------
// Accent colors
// ---------------------------------------------------------------------------

/// Warning / unsaved-changes indicator.
pub const ACCENT_WARN:    Color = Color { r: 0.85, g: 0.65, b: 0.20, a: 1.0 };
/// Success / logo-set indicator.
pub const ACCENT_SUCCESS: Color = Color { r: 0.40, g: 0.85, b: 0.50, a: 1.0 };
/// Error / missing-asset indicator.
pub const ACCENT_DANGER:  Color = Color { r: 0.75, g: 0.40, b: 0.40, a: 1.0 };

// ---------------------------------------------------------------------------
// Type scale — font sizes in points
// ---------------------------------------------------------------------------

pub const TEXT_XS:      f32 = 11.0;
pub const TEXT_SM:      f32 = 12.0;
pub const TEXT_MD:      f32 = 13.0;
pub const TEXT_LG:      f32 = 15.0;
pub const TEXT_XL:      f32 = 16.0;
pub const TEXT_H3:      f32 = 18.0;
pub const TEXT_H2:      f32 = 20.0;
pub const TEXT_H1:      f32 = 24.0;
pub const TEXT_TITLE:   f32 = 36.0;
pub const TEXT_DISPLAY: f32 = 48.0;

// ---------------------------------------------------------------------------
// Spacing — padding / gap sizes in pixels
// ---------------------------------------------------------------------------

pub const SPACE_XS:  f32 = 4.0;
pub const SPACE_SM:  f32 = 8.0;
pub const SPACE_MD:  f32 = 12.0;
pub const SPACE_LG:  f32 = 16.0;
pub const SPACE_XL:  f32 = 24.0;
pub const SPACE_2XL: f32 = 32.0;

// ---------------------------------------------------------------------------
// Fonts
// ---------------------------------------------------------------------------

/// Returns the default UI font with bold weight applied.
pub fn bold() -> Font {
    Font { weight: iced::font::Weight::Bold, ..Font::DEFAULT }
}

// ---------------------------------------------------------------------------
// Container style functions
// ---------------------------------------------------------------------------

/// Returns a style closure that fills the container with `color`.
/// Usage: `.style(theme::bg(theme::BG_SIDEBAR))`
pub fn bg(c: Color) -> impl Fn(&iced::Theme) -> CStyle {
    move |_| CStyle { background: Some(Background::Color(c)), ..Default::default() }
}

/// Alternating row background (even = BG_PANEL, odd = BG_ROW_B).
/// Usage: `.style(theme::row_alt(i))`
pub fn row_alt(i: usize) -> impl Fn(&iced::Theme) -> CStyle {
    bg(if i % 2 == 0 { BG_PANEL } else { BG_ROW_B })
}

/// The foreground card surface — rounded corners, subtle border, drop shadow.
/// Used by the main menu, new-game wizard, and the League Builder outer frame.
pub fn card_style(_: &iced::Theme) -> CStyle {
    CStyle {
        background: Some(Background::Color(BG_CARD)),
        border: Border {
            radius: 10.0.into(),
            width: 1.0,
            color: BORDER_CARD,
        },
        shadow: Shadow {
            color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.45 },
            offset: Vector { x: 0.0, y: 3.0 },
            blur_radius: 18.0,
        },
        ..Default::default()
    }
}

/// App-shell background style (BG_APP, no border).
pub fn shell_style(_: &iced::Theme) -> CStyle {
    CStyle {
        background: Some(Background::Color(BG_APP)),
        ..Default::default()
    }
}

/// Dialog panel style — BG_PANEL with a visible border and rounded corners.
pub fn dialog_panel_style(_: &iced::Theme) -> CStyle {
    CStyle {
        background: Some(Background::Color(BG_PANEL)),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: BORDER_PANEL,
        },
        ..Default::default()
    }
}

/// Overlay backdrop — fully covers the screen with a translucent dim layer.
pub fn overlay_style(_: &iced::Theme) -> CStyle {
    CStyle {
        background: Some(Background::Color(BG_OVERLAY)),
        ..Default::default()
    }
}
