#[cfg(target_os = "unix")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "linux")]
use crate::ubuntu_terminal_sceme::*;
#[cfg(target_os = "windows")]
use crate::windows_cmd_sceme::*;
