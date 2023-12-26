use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use miette::IntoDiagnostic;
use ratatui::{prelude::CrosstermBackend, Terminal};

fn main() -> miette::Result<()> {
    // setup terminal
    enable_raw_mode().into_diagnostic()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).into_diagnostic()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).into_diagnostic()?;

    // create app and run it
    // let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode().into_diagnostic()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .into_diagnostic()?;
    terminal
        .show_cursor()
        .into_diagnostic()?;

    // if let Err(err) = res {
    //     println!("{err:?}");
    // }

    Ok(())
}
// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     let mut selected_button: usize = 0;
//     // let button_states = &mut [State::Selected, State::Normal, State::Normal];
//     loop {
//         terminal.draw(|frame| ui(frame, button_states))?;
//         if !event::poll(Duration::from_millis(100))? {
//             continue;
//         }
//         match event::read()? {
//             Event::Key(key) => {
//                 if key.kind != event::KeyEventKind::Press {
//                     continue;
//                 }
//                 if handle_key_event(key, button_states, &mut selected_button).is_break() {
//                     break;
//                 }
//             }
//             Event::Mouse(mouse) => handle_mouse_event(mouse, button_states, &mut selected_button),
//             _ => (),
//         }
//     }
//     Ok(())
// }
