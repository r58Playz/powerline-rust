use std::env;
use std::marker::PhantomData;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct ExitCode<S: ExitCodeScheme> {
    scheme: PhantomData<S>,
}

pub trait ExitCodeScheme {
    const EXIT_CODE_BG: Color;
    const EXIT_CODE_FG: Color;
}

impl<S: ExitCodeScheme> ExitCode<S> {
    pub fn new() -> ExitCode<S> {
        ExitCode { scheme: PhantomData }
    }
}

impl<S: ExitCodeScheme> Module for ExitCode<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if env::args().len() > 2 {
            if !Some(env::args().skip(1).map(|x| x == "0").all(|x| x)).unwrap_or(false) {
                powerline.add_segment(
                    env::args().skip(1).map(|x| x).collect::<Vec<String>>().join("\u{F19B0}"),
                    Style::simple(S::EXIT_CODE_FG, S::EXIT_CODE_BG),
                );
            }
        } else {
            if let Some(exit_code) = env::args().nth(1).as_deref() {
                if exit_code != "0" {
                    powerline.add_segment(exit_code, Style::simple(S::EXIT_CODE_FG, S::EXIT_CODE_BG))
                }
            }
        }
    }
}
