use launchpad::{Launchpad, Button, ButtonType, Color};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut launchpad = Launchpad::new()?;

    launchpad.configure_button(Button::new(ButtonType::Grid, 57), |launchpad| {
        println!("Steam button pressed!");
        launchpad.color_current_button(Color::GREEN);
    });

    launchpad.configure_button(Button::new(ButtonType::Grid, 58), |launchpad| {
        println!("Plex button pressed!");
        launchpad.color_current_button(Color::RED);
    });

    launchpad.poll()?;
    Ok(())
}