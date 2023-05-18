use crate::color::Color;

pub struct Style {
    pub code: String,
    pub text: String,
}

impl Style {
    /// Create a new Style object.
    pub fn new() -> Style {
        Style {
            code: String::new(),
            text: String::new(),
        }
    }

    /// Set the text bold.
    pub fn bold(mut self) -> Style {
        self.code.push_str("\x1b[1m");
        self
    }

    /// Set the text italic.
    pub fn italic(mut self) -> Style {
        self.code.push_str("\x1b[3m");
        self
    }

    /// Set the text underline.
    pub fn underline(mut self) -> Style {
        self.code.push_str("\x1b[4m");
        self
    }

    /// Set the text foreground color.
    pub fn foreground(mut self, color: Color) -> Style {
        let code = format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b);
        self.code.push_str(code.as_str());
        self
    }

    /// Set the text background color.
    pub fn background(mut self, color: Color) -> Style {
        let code = format!("\x1b[48;2;{};{};{}m", color.r, color.g, color.b);
        self.code.push_str(code.as_str());
        self
    }

    /// Set the text
    pub fn set_string(mut self, text: &str) -> Style {
        self.text = text.to_string();
        self
    }

    /// Render the text setted with the style.
    /// Use when you are done with your style.
    ///
    /// # Example
    ///
    /// ```
    /// use rusty_style::color::Color;
    ///
    /// let my_style = rusty_style::style::Style::new()
    ///    .bold()
    ///   .italic()
    ///   .underline()
    ///   .foreground(Color::new(255, 0, 0))
    ///   .background(Color::new(0, 0, 255))
    ///   .set_string("Hello");
    ///
    /// let rendered_string = my_style.render("World");
    /// assert_eq!(rendered_string, "\x1b[1m\x1b[3m\x1b[4m\x1b[38;2;255;0;0m\x1b[48;2;0;0;255mHelloWorld\x1b[0m");
    /// ```
    pub fn render(mut self, text: &str) -> String {
        self.text.push_str(text);

        let rendered_string = format!("{}{}\x1b[0m", self.code, self.text);
        rendered_string
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;

    use super::Style;

    #[test]
    fn new_style() {
        let style = Style::new();

        assert_eq!(style.code, "");
        assert_eq!(style.text, "");
    }

    #[test]
    fn bold() {
        let style = Style::new().bold();

        assert_eq!(style.code, "\x1b[1m");
    }

    #[test]
    fn italic() {
        let style = Style::new().italic();

        assert_eq!(style.code, "\x1b[3m");
    }

    #[test]
    fn underline() {
        let style = Style::new().underline();

        assert_eq!(style.code, "\x1b[4m");
    }

    #[test]
    fn foreground() {
        let style = Style::new().foreground(Color::new(255, 0, 0));

        assert_eq!(style.code, "\x1b[38;2;255;0;0m");
    }

    #[test]
    fn background() {
        let style = Style::new().background(Color::new(255, 0, 0));

        assert_eq!(style.code, "\x1b[48;2;255;0;0m");
    }

    #[test]
    fn set_string() {
        let style = Style::new().set_string("Hello");

        assert_eq!(style.text, "Hello");
    }
}
