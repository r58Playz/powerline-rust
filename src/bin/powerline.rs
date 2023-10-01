extern crate powerline;

#[cfg(feature = "time")]
use powerline::modules::Time;
use powerline::modules::*;
use powerline::theme::SimpleTheme;

fn main() {
    let mut prompt = powerline::Powerline::new();

    #[cfg(feature = "time")]
    prompt.add_module(Time::<SimpleTheme>::with_time_format("%H:%M:%S"))?;

    prompt.add_module(Ssh::<SimpleTheme>::new());
    prompt.add_module(User::<SimpleTheme>::new());
    prompt.add_module(Host::<SimpleTheme>::new());
    prompt.add_module(VirtualEnv::<SimpleTheme>::new());
    prompt.add_module(Cwd::<SimpleTheme>::new(45, 4, false));
    prompt.add_module(Git::<SimpleTheme>::new());
    prompt.add_module(ReadOnly::<SimpleTheme>::new());
    prompt.add_module(Newline::<SimpleTheme>::new());
    prompt.add_module(ExitCode::<SimpleTheme>::new());
    prompt.add_module(Cmd::<SimpleTheme>::new());

    println!("{} ", prompt);
}
