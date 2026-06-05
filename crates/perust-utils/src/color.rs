//! Console color formatting utilities using ANSI escape codes.
//!
//! This module provides:
//! - [`ColorCode`]: An enum of standard ANSI color codes.
//! - [`format_color`]: Formats a string with foreground color, background color, and style.
//! - [`StyledStringBuilder`]: A builder for constructing strings with mixed colors and styles.
//!
//! # Examples
//!
//! ```
//! use perust_utils::color::{ColorCode, format_color, StyledStringBuilder};
//!
//! // Simple colored text
//! let red_text = format_color("Error!", ColorCode::Red, None, false, false);
//! println!("{}", red_text);
//!
//! // Building a styled string with multiple segments
//! let msg = StyledStringBuilder::new()
//!     .fg(ColorCode::Green).text("OK: ")
//!     .fg(ColorCode::White).text("Server started successfully")
//!     .build();
//! println!("{}", msg);
//! ```

// ---------------------------------------------------------------------------
// ANSI escape code constants
// ---------------------------------------------------------------------------

const ESC: &str = "\x1B[";
const RESET: &str = "\x1B[0m";

// ---------------------------------------------------------------------------
// ColorCode
// ---------------------------------------------------------------------------

/// Standard ANSI terminal color codes.
///
/// Each variant maps to a well-known foreground/background color code.
/// Both normal (0–7) and bright/bold (8–15) variants are provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColorCode {
    /// Black (code 30/40)
    Black,
    /// Red (code 31/41)
    Red,
    /// Green (code 32/42)
    Green,
    /// Yellow (code 33/43)
    Yellow,
    /// Blue (code 34/44)
    Blue,
    /// Magenta (code 35/45)
    Magenta,
    /// Cyan (code 36/46)
    Cyan,
    /// White (code 37/47)
    White,
    /// Bright black / gray (code 90/100)
    BrightBlack,
    /// Bright red (code 91/101)
    BrightRed,
    /// Bright green (code 92/102)
    BrightGreen,
    /// Bright yellow (code 93/103)
    BrightYellow,
    /// Bright blue (code 94/104)
    BrightBlue,
    /// Bright magenta (code 95/105)
    BrightMagenta,
    /// Bright cyan (code 96/106)
    BrightCyan,
    /// Bright white (code 97/107)
    BrightWhite,
}

impl ColorCode {
    /// Returns the ANSI foreground code for this color.
    pub fn fg_code(self) -> u8 {
        match self {
            ColorCode::Black => 30,
            ColorCode::Red => 31,
            ColorCode::Green => 32,
            ColorCode::Yellow => 33,
            ColorCode::Blue => 34,
            ColorCode::Magenta => 35,
            ColorCode::Cyan => 36,
            ColorCode::White => 37,
            ColorCode::BrightBlack => 90,
            ColorCode::BrightRed => 91,
            ColorCode::BrightGreen => 92,
            ColorCode::BrightYellow => 93,
            ColorCode::BrightBlue => 94,
            ColorCode::BrightMagenta => 95,
            ColorCode::BrightCyan => 96,
            ColorCode::BrightWhite => 97,
        }
    }

    /// Returns the ANSI background code for this color.
    pub fn bg_code(self) -> u8 {
        match self {
            ColorCode::Black => 40,
            ColorCode::Red => 41,
            ColorCode::Green => 42,
            ColorCode::Yellow => 43,
            ColorCode::Blue => 44,
            ColorCode::Magenta => 45,
            ColorCode::Cyan => 46,
            ColorCode::White => 47,
            ColorCode::BrightBlack => 100,
            ColorCode::BrightRed => 101,
            ColorCode::BrightGreen => 102,
            ColorCode::BrightYellow => 103,
            ColorCode::BrightBlue => 104,
            ColorCode::BrightMagenta => 105,
            ColorCode::BrightCyan => 106,
            ColorCode::BrightWhite => 107,
        }
    }
}

// ---------------------------------------------------------------------------
// TextStyle
// ---------------------------------------------------------------------------

/// Text styling attributes that can be applied alongside colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextStyle {
    /// Bold / increased intensity.
    Bold,
    /// Underlined text.
    Underline,
    /// Bold + underline.
    BoldUnderline,
    /// No extra style.
    None,
}

impl TextStyle {
    /// Returns the ANSI style codes for this text style.
    pub fn codes(self) -> &'static [u8] {
        match self {
            TextStyle::Bold => &[1],
            TextStyle::Underline => &[4],
            TextStyle::BoldUnderline => &[1, 4],
            TextStyle::None => &[],
        }
    }
}

// ---------------------------------------------------------------------------
// format_color
// ---------------------------------------------------------------------------

/// Formats a string with ANSI foreground color, optional background color,
/// and optional bold/underline styles.
///
/// The returned string includes the ANSI escape sequences and a reset sequence
/// at the end. If the output does not support ANSI codes, the original text
/// is still readable (but unstyled).
///
/// # Arguments
///
/// * `text` - The text to colorize.
/// * `fg` - Foreground color.
/// * `bg` - Optional background color.
/// * `bold` - Whether to apply bold style.
/// * `underline` - Whether to apply underline style.
///
/// # Examples
///
/// ```
/// use perust_utils::color::{format_color, ColorCode};
///
/// let msg = format_color("Warning!", ColorCode::Yellow, None, true, false);
/// ```
pub fn format_color(
    text: &str,
    fg: ColorCode,
    bg: Option<ColorCode>,
    bold: bool,
    underline: bool,
) -> String {
    let mut seq = String::from(ESC);

    // Build the parameter list
    let mut params: Vec<String> = Vec::new();

    if bold {
        params.push("1".to_string());
    }
    if underline {
        params.push("4".to_string());
    }

    params.push(fg.fg_code().to_string());

    if let Some(bg_color) = bg {
        params.push(bg_color.bg_code().to_string());
    }

    seq.push_str(&params.join(";"));
    seq.push('m');

    format!("{seq}{text}{RESET}")
}

// ---------------------------------------------------------------------------
// Convenience functions
// ---------------------------------------------------------------------------

/// Formats text with just a foreground color (no bold/underline).
#[inline]
pub fn fg(text: &str, color: ColorCode) -> String {
    format_color(text, color, None, false, false)
}

/// Formats text with a foreground color and bold style.
#[inline]
pub fn bold(text: &str, color: ColorCode) -> String {
    format_color(text, color, None, true, false)
}

/// Formats text with a foreground color, background color, and bold style.
#[inline]
pub fn colored(text: &str, fg_color: ColorCode, bg_color: ColorCode) -> String {
    format_color(text, fg_color, Some(bg_color), false, false)
}

/// Resets all ANSI styling.
#[inline]
pub fn reset() -> &'static str {
    RESET
}

// ---------------------------------------------------------------------------
// StyledStringBuilder
// ---------------------------------------------------------------------------

/// A builder for constructing strings with mixed ANSI colors and styles.
///
/// This allows you to build up a single string where different segments
/// have different colors or styles, without manually managing escape codes.
///
/// # Examples
///
/// ```
/// use perust_utils::color::{ColorCode, StyledStringBuilder};
///
/// let msg = StyledStringBuilder::new()
///     .fg(ColorCode::Cyan).text("[INFO] ")
///     .fg(ColorCode::White).text("Player joined the game")
///     .build();
/// ```
pub struct StyledStringBuilder {
    /// The accumulated output string.
    output: String,
    /// Current foreground color (if set).
    current_fg: Option<ColorCode>,
    /// Current background color (if set).
    current_bg: Option<ColorCode>,
    /// Whether bold is active.
    current_bold: bool,
    /// Whether underline is active.
    current_underline: bool,
}

impl StyledStringBuilder {
    /// Creates a new, empty `StyledStringBuilder`.
    pub fn new() -> Self {
        Self {
            output: String::new(),
            current_fg: None,
            current_bg: None,
            current_bold: false,
            current_underline: false,
        }
    }

    /// Sets the foreground color for subsequent text.
    pub fn fg(mut self, color: ColorCode) -> Self {
        self.current_fg = Some(color);
        self
    }

    /// Sets the background color for subsequent text.
    pub fn bg(mut self, color: ColorCode) -> Self {
        self.current_bg = Some(color);
        self
    }

    /// Enables bold for subsequent text.
    pub fn bold(mut self) -> Self {
        self.current_bold = true;
        self
    }

    /// Enables underline for subsequent text.
    pub fn underline(mut self) -> Self {
        self.current_underline = true;
        self
    }

    /// Appends text with the current style settings.
    ///
    /// If no foreground color has been set, the text is appended without
    /// ANSI codes.
    pub fn text(mut self, text: &str) -> Self {
        if let Some(fg_color) = self.current_fg {
            self.output
                .push_str(&format_color(text, fg_color, self.current_bg, self.current_bold, self.current_underline));
        } else {
            self.output.push_str(text);
        }
        self
    }

    /// Appends raw text without any styling, ignoring current style settings.
    pub fn raw(mut self, text: &str) -> Self {
        self.output.push_str(text);
        self
    }

    /// Appends a newline character.
    pub fn newline(self) -> Self {
        self.raw("\n")
    }

    /// Resets the current style settings (foreground, background, bold, underline).
    pub fn reset_style(mut self) -> Self {
        self.current_fg = None;
        self.current_bg = None;
        self.current_bold = false;
        self.current_underline = false;
        self
    }

    /// Consumes the builder and returns the final styled string.
    pub fn build(self) -> String {
        self.output
    }
}

impl Default for StyledStringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for StyledStringBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_code_fg() {
        assert_eq!(ColorCode::Red.fg_code(), 31);
        assert_eq!(ColorCode::BrightCyan.fg_code(), 96);
    }

    #[test]
    fn test_color_code_bg() {
        assert_eq!(ColorCode::Red.bg_code(), 41);
        assert_eq!(ColorCode::BrightCyan.bg_code(), 106);
    }

    #[test]
    fn test_format_color_simple() {
        let result = format_color("Hello", ColorCode::Green, None, false, false);
        assert!(result.starts_with("\x1B[32m"));
        assert!(result.ends_with("\x1B[0m"));
        assert!(result.contains("Hello"));
    }

    #[test]
    fn test_format_color_bold() {
        let result = format_color("Hello", ColorCode::Red, None, true, false);
        assert!(result.starts_with("\x1B[1;31m"));
    }

    #[test]
    fn test_format_color_with_bg() {
        let result = format_color("Hello", ColorCode::White, Some(ColorCode::Red), false, false);
        assert!(result.starts_with("\x1B[37;41m"));
    }

    #[test]
    fn test_fg_convenience() {
        let result = fg("test", ColorCode::Cyan);
        assert!(result.contains("test"));
        assert!(result.starts_with("\x1B[36m"));
    }

    #[test]
    fn test_styled_string_builder() {
        let msg = StyledStringBuilder::new()
            .fg(ColorCode::Green)
            .text("OK")
            .reset_style()
            .raw(": ")
            .fg(ColorCode::White)
            .text("done")
            .build();

        assert!(msg.contains("OK"));
        assert!(msg.contains("done"));
        assert!(msg.contains(": "));
    }
}
