use std::marker::PhantomData;

use super::Module;
use crate::{utils::is_remote_shell, Color, Powerline, Style};

pub struct Ssh<S>(PhantomData<S>);

pub trait SshScheme {
    const SSH_BG: Color;
    const SSH_FG: Color;
}

impl<S: SshScheme> Ssh<S> {
    pub fn new() -> Ssh<S> {
        Ssh(PhantomData)
    }
}

impl<S: SshScheme> Module for Ssh<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        if is_remote_shell() {
            powerline.add_segment("\u{F0318}", Style::simple(S::SSH_FG, S::SSH_BG));
        }
    }
}
