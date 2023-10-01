use std::marker::PhantomData;

use super::Module;
use crate::{
    terminal::{FgColor, Reset},
    Color, Powerline, Style,
};

pub struct Newline<S>(PhantomData<S>);

pub trait NewlineScheme {
    const NEWLINE_COLOR: Color;
    const NEWLINE_SPACING: usize = 2;
}

impl<S: NewlineScheme> Newline<S> {
    pub fn new() -> Newline<S> {
        Newline(PhantomData)
    }
}

impl<S: NewlineScheme> Module for Newline<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        powerline.add_short_segment(
            format!("{}{}{}\u{E0B4}\n", " ".repeat(S::NEWLINE_SPACING), Reset, FgColor::from(S::NEWLINE_COLOR)),
            Style::special(S::NEWLINE_COLOR, S::NEWLINE_COLOR, ' ', S::NEWLINE_COLOR),
        );
    }
}
