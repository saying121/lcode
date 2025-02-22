use crate::{
    app::inner::App,
    mytui::{myevent::EventsHandler, term::Term},
};

impl App<'_> {
    /// send info for render tui
    pub fn render(&self) {
        self.events.render();
    }
    pub fn exit(&mut self) -> bool {
        self.events.exit();
        false
    }
    /// leave alter screen, and stop event stream
    pub fn pause(&mut self) {
        Term::stop().ok();
        self.events.stop_events().ok();
    }
    /// enter alter screen, and start event-stream
    pub fn r#continue(&mut self) {
        Term::start().ok();
        self.events = EventsHandler::new();
        self.events.redraw_tui();
    }
}
