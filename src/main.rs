use launchy::{Color, MidiError, Canvas, MsgPollingWrapper};

mod button;
use button::{Button, ButtonType as BType};

mod state;
use state::GlobalState;

// Main loop
// ----------------------------------------------

fn main() -> Result<(), MidiError> {
    let (mut canvas, poller) = launchy::mini::Canvas::guess_polling()?;
    let mut state = GlobalState::new("./res/data.conf");

    init_launchpad(&mut canvas, &mut state);

    for msg in poller.iter() {
        if msg.is_press() {
            on_press(&mut canvas, &mut state, Button::from(msg.pad()));
        } else {
            on_release(&mut canvas, &mut state, Button::from(msg.pad()));
        }
        canvas.flush()?;
    }

    return Ok(());
}



// Board setup
// ----------------------------------------------

fn init_launchpad(lp: &mut impl Canvas, state: &mut GlobalState) {
    lp.clear();

    lp.set(Button::calc_pad(BType::Top, state.get_preset()), Color::RED);

    for (id, _cmd) in state.get_commands() {
        let num_id: u8 = id.parse().unwrap();
        lp.set(Button::calc_pad(BType::Grid, num_id), Color::RED);
    }

    lp.flush().expect("Counldn't flush");
}



// Button management
// ----------------------------------------------

fn on_press(lp: &mut impl Canvas, state: &mut GlobalState, btn: Button) {

    match btn.btn_type {
        
        BType::Top => {
            state.set_preset(btn.id);
            init_launchpad(lp, state);
            println!("Switched to preset [{}]", state.get_section());
        },

        BType::Grid => {
            //lp.set(btn.get_pad(), Color::RED);
            match state.get_command(btn.id) {
                Some(cmd) => {
                    lp.set(btn.get_pad(), Color::BLACK);
                    println!("command: {}", cmd);
                }
                None => ()
            };
        }

        _ => ()
    };
}

fn on_release(lp: &mut impl Canvas, state: &mut GlobalState, btn: Button) {
    match state.get_command(btn.id) {
        Some(_) => lp.set(btn.get_pad(), Color::RED),
        None => Some(())
    };
}