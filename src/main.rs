use launchpad::{Launchpad, Button, ButtonType};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut launchpad = Launchpad::new()?;

    launchpad.configure_button(Button::new(ButtonType::Top, 1), || {
        println!("Button pressed!");
    });

    launchpad.configure_button(Button::new(ButtonType::Top, 2), || {
        println!("Button pressed 2!");
    });

    launchpad.poll()?;
    Ok(())
}