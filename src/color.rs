use std::fmt;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new Color object.
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
        }
    }

    /// Create a new Color object from a hex code.
    ///
    /// The hex code must be in the format #RRGGBB.
    /// The # is required.
    /// The hex code is not case sensitive.
    /// If the hex code is invalid, the function will return None.
    /// If the hex code is valid, the function will return Some(Color).
    pub fn convert_hex_to_rgb(hex: &str) -> Option<Color> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return None;
        }

        let red = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let green = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let blue = u8::from_str_radix(&hex[5..7], 16).ok()?;

        Some(Color::new(red, green, blue))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color: r: {}, g: {}, b: {}", self.r, self.g, self.b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn new_color() {
        let red = 255;
        let green = 200;
        let blue = 102;

        let color = super::Color::new(red, green, blue);

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 200);
        assert_eq!(color.b, 102);
    }

    #[test]
    fn convert_hex_to_rgb() {
        let hex_color = "#9F2B68";

        let color = super::Color::convert_hex_to_rgb(hex_color).unwrap();

        assert_eq!(color.r, 159);
        assert_eq!(color.g, 43);
        assert_eq!(color.b, 104);
    }
}
