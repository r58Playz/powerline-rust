#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BgColor(u8, u8, u8);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FgColor(u8, u8, u8);

pub struct Reset;

impl FgColor {
    pub fn transpose(self) -> BgColor {
        BgColor(self.0, self.1, self.2)
    }
}

impl From<Color> for FgColor {
    fn from(c: Color) -> Self {
        FgColor(c.0, c.1, c.2)
    }
}

impl BgColor {
    pub fn transpose(self) -> FgColor {
        FgColor(self.0, self.1, self.2)
    }
}

impl From<Color> for BgColor {
    fn from(c: Color) -> Self {
        BgColor(c.0, c.1, c.2)
    }
}

impl std::fmt::Display for BgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature = "bash-shell")]
        return write!(f, r#"\[\e[48;2;{};{};{}m\]"#, self.0, self.1, self.2);

        #[cfg(feature = "bare-shell")]
        return write!(f, "\x1b[48;2;{};{};{}m", self.0, self.1, self.2);

        #[cfg(feature = "zsh-shell")]
        return write!(f, "%{{\x1b[48;2;{};{};{}m%}}", self.0, self.1, self.2);
    }
}

impl std::fmt::Display for FgColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature = "bash-shell")]
        return write!(f, r#"\[\e[38;2;{};{};{}m\]"#, self.0, self.1, self.2);

        #[cfg(feature = "bare-shell")]
        return write!(f, "\x1b[38;2;{};{};{}m", self.0, self.1, self.2);

        #[cfg(feature = "zsh-shell")]
        return write!(f, "%{{\x1b[38;2;{};{};{}m%}}", self.0, self.1, self.2);
    }
}

impl std::fmt::Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        #[cfg(feature = "bash-shell")]
        return f.write_str(r#"\[\e[0m\]"#);

        #[cfg(feature = "bare-shell")]
        return f.write_str("\x1b[0m");

        #[cfg(feature = "zsh-shell")]
        return f.write_str("%{\x1b[39m%}%{\x1b[49m%}");
    }
}
