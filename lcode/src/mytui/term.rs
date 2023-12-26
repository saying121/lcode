use std::{
    io,
    io::Stdout,
    ops::{Deref, DerefMut}
};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand
};
use miette::{IntoDiagnostic, Result};
use ratatui::{prelude::*, Terminal};

pub struct Term {
    pub(crate) inner: Terminal<CrosstermBackend<Stdout>>
}

impl Drop for Term {
    fn drop(&mut self) {
        self.stop().ok();
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
            inner: Terminal::new(backend).into_diagnostic()?
        })
    }
    /// setup terminal
    pub fn setup_terminal(&self) -> Result<()> {
        enable_raw_mode().into_diagnostic()?;
        io::stdout()
            .execute(EnterAlternateScreen)
            .into_diagnostic();
        Ok(())
    }
    /// restore terminal
    pub fn stop(&mut self) -> Result<()> {
        disable_raw_mode().into_diagnostic()?;
        io::stdout()
            .execute(LeaveAlternateScreen)
            .into_diagnostic()?;
        self.show_cursor()
            .into_diagnostic()
    }

    pub fn redraw(&mut self) -> Result<()> {
        // self.inner.autoresize().into_diagnostic()
        let pat = self.size().into_diagnostic()?;
        self.resize(pat).into_diagnostic()
    }
}
