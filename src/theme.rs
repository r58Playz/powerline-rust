use crate::modules::*;
use crate::Color;

#[derive(Copy, Clone)]
pub struct SimpleTheme;

impl CmdScheme for SimpleTheme {
    const CMD_FAILED_BG: Color = Color(243, 139, 168);
    const CMD_FAILED_FG: Color = Color(49, 50, 68);
    const CMD_PASSED_BG: Color = Color(203, 166, 247);
    const CMD_PASSED_FG: Color = Color(49, 50, 68);
}

impl CwdScheme for SimpleTheme {
    const CWD_FG: Color = Color(49, 50, 68);
    const HOME_BG: Color = Color(88, 91, 112);
    const HOME_FG: Color = Color(205, 214, 244);
    const PATH_BG: Color = Color(69, 71, 90);
    const PATH_FG: Color = Color(205, 214, 244);
    const SEPARATOR_FG: Color = Color(147, 153, 178);
}

impl ExitCodeScheme for SimpleTheme {
    const EXIT_CODE_BG: Color = Color(49, 50, 68);
    const EXIT_CODE_FG: Color = Color(243, 139, 168);
}

impl UserScheme for SimpleTheme {
    const USERNAME_BG: Color = Color(180, 190, 254);
    const USERNAME_FG: Color = Color(49, 50, 68);
    const USERNAME_ROOT_BG: Color = Color(180, 190, 254);
}

impl HostScheme for SimpleTheme {
    const HOSTNAME_BG: Color = Color(137, 180, 250);
    const HOSTNAME_FG: Color = Color(49, 50, 68);
}

impl ReadOnlyScheme for SimpleTheme {
    const READONLY_BG: Color = Color(249, 226, 175);
    const READONLY_FG: Color = Color(49, 50, 68);
}

#[cfg(feature = "time")]
impl TimeScheme for SimpleTheme {
    const TIME_BG: Color = Color(69, 71, 90);
    const TIME_FG: Color = Color(205, 214, 244);
}

impl GitScheme for SimpleTheme {
    const GIT_AHEAD_BG: Color = Color(88, 91, 112);
    const GIT_AHEAD_FG: Color = Color(205, 214, 244);
    const GIT_BEHIND_BG: Color = Color(88, 91, 112);
    const GIT_BEHIND_FG: Color = Color(205, 214, 244);
    const GIT_CONFLICTED_BG: Color = Color(243, 139, 168);
    const GIT_CONFLICTED_FG: Color = Color(49, 50, 68);
    const GIT_NOTSTAGED_BG: Color = Color(250, 179, 135);
    const GIT_NOTSTAGED_FG: Color = Color(49, 50, 68);
    const GIT_REPO_CLEAN_BG: Color = Color(166, 227, 161);
    const GIT_REPO_CLEAN_FG: Color = Color(49, 50, 68);
    const GIT_REPO_DIRTY_BG: Color = Color(235, 160, 172);
    const GIT_REPO_DIRTY_FG: Color = Color(49, 50, 68);
    const GIT_STAGED_BG: Color = Color(166, 227, 161);
    const GIT_STAGED_FG: Color = Color(49, 50, 68);
    const GIT_UNTRACKED_BG: Color = Color(243, 139, 168);
    const GIT_UNTRACKED_FG: Color = Color(49, 50, 68);
}

impl VirtualEnvScheme for SimpleTheme {
    const PYVENV_BG: Color = Color(116, 227, 161);
    const PYVENV_FG: Color = Color(49, 50, 68);
}

impl SshScheme for SimpleTheme {
    const SSH_BG: Color = Color(245, 194, 231);
    const SSH_FG: Color = Color(49, 50, 68);
}

impl NewlineScheme for SimpleTheme {
    const NEWLINE_COLOR: Color = Color(203, 166, 247);
}
