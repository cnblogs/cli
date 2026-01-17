//!
//! 输出统一管理
//!

use std::fmt;
use std::fmt::Display;
use std::io::IsTerminal;
use std::io::Write;

use anstream::{AutoStream, ColorChoice};
use anstyle::{AnsiColor, Color, Style};
use anyhow::Ok;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug)]
pub struct Terminal {
    pub stdout: AutoStream<std::io::Stdout>,
    pub stderr: AutoStream<std::io::Stderr>,
    pub color_choice: ColorChoice,
}

impl Terminal {
    pub fn new() -> Self {
        let use_color = std::io::stdout().is_terminal();

        let color_choice = if use_color {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        };
        let stdout = AutoStream::new(std::io::stdout(), color_choice);
        let stderr = AutoStream::new(std::io::stderr(), color_choice);
        Self {
            stdout,
            stderr,
            color_choice,
        }
    }

    pub fn write(&mut self, msg: impl Display) -> Result<()> {
        Ok(write!(self.stdout, "{}", msg)?)
    }

    pub fn json(&mut self, msg: impl fmt::Display + Serialize) -> Result<()> {
        Ok(writeln!(self.stdout, "{}", serde_json::to_string(&msg)?)?)
    }

    /// 打印信息（普通输出）
    pub fn writeln(&mut self, message: impl fmt::Display) -> Result<()> {
        Ok(writeln!(self.stdout, "{}", message)?)
    }

    /// 清除当前行
    pub fn clear_line(&mut self) -> Result<()> {
        Ok(write!(self.stdout, "\r\x1b[K")?)
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}
pub const fn blue_style() -> Style {
    Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue)))
}

pub const fn green_style() -> Style {
    Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)))
}

#[derive(Debug, PartialEq, Eq)]
pub enum Verbosity {
    Verbose,
    Normal,
    Qiuet,
}
