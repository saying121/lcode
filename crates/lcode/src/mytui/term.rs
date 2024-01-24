use std::{
    io,
    io::Stdout,
    ops::{Deref, DerefMut},
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use miette::{IntoDiagnostic, Result};
use ratatui::{prelude::*, Terminal};

#[derive(Debug)]
pub struct Term {
    pub(crate) inner: Terminal<CrosstermBackend<Stdout>>,
}

impl Drop for Term {
    fn drop(&mut self) {
        Self::stop().ok();
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Term {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        Ok(Self {
            inner: Terminal::new(backend).into_diagnostic()?,
        })
    }
    pub fn start() -> Result<()> {
        enable_raw_mode().into_diagnostic()?;
        io::stdout()
            .execute(EnterAlternateScreen)
            .into_diagnostic()?;
        Ok(())
    }
    /// restore terminal
    pub fn stop() -> Result<()> {
        disable_raw_mode().into_diagnostic()?;
        io::stdout()
            .execute(LeaveAlternateScreen)
            .into_diagnostic()?;
        Ok(())
    }

    pub fn resize(&mut self, width: u16, height: u16) -> Result<()> {
        self.inner
            .resize(Rect::new(0, 0, width, height))
            .into_diagnostic()
    }
}
