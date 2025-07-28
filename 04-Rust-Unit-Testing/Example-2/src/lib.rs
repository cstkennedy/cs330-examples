use std::fmt;

/// An HTML Color is represented by a combination of red, green and blue. Each
/// component (red, green, and blue) must fall in the range [0, 255].
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HtmlColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const BLACK: HtmlColor = HtmlColor::new(0, 0, 0);
pub const WHITE: HtmlColor = HtmlColor::new(255, 255, 255);
pub const RED: HtmlColor = HtmlColor::new(255, 0, 0);
pub const GREEN: HtmlColor = HtmlColor::new(0, 255, 0);
pub const BLUE: HtmlColor = HtmlColor::new(0, 0, 255);


impl Default for HtmlColor {
    /// Construct an HTML Color with all
    /// attributes set to 0 (i.e., black, #000000)
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl HtmlColor {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl fmt::Display for HtmlColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

