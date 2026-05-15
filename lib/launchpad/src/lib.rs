use std::collections::HashMap;

use launchy::{Canvas, Color, DeviceCanvas, DeviceCanvasPoller, MidiError, MsgPollingWrapper};

mod button;
pub use button::{Button, ButtonType};

pub struct Launchpad {
    canvas: DeviceCanvas<launchy::mini::Spec>,
    poller: DeviceCanvasPoller,
    button_configurations: HashMap<Button, Box<dyn Fn()>>,
}

impl Launchpad {
    pub fn new() -> Result<Self, MidiError> {
        let (canvas, poller) = launchy::mini::Canvas::guess_polling()?;

        let mut self_ = Self { canvas, poller, button_configurations: HashMap::new() };
        self_.clear()?;

        Ok(self_)
    }

    pub fn clear(&mut self) -> Result<(), MidiError> {
        println!("Clearing launchpad...");
        self.canvas.clear();
        self.canvas.flush()?;
        Ok(())
    }

    pub fn configure_button<F: Fn() + 'static>(&mut self, button: Button, callback: F) {
        self.button_configurations.insert(button, Box::new(callback));
    }

    pub fn poll(&mut self) -> Result<(), MidiError> {
        for msg in self.poller.iter() {
            let button = Button::from(msg.pad());
            if msg.is_press() {
                println!("Button {:?} pressed!", button);
                self.canvas.set(button.get_pad(), Color::GREEN);
                if let Some(callback) = self.button_configurations.get(&button) {
                    callback();
                }
            } else {
                println!("Button {:?} released!", button);
                self.canvas.set(button.get_pad(), Color::BLACK);
            }
            self.canvas.flush()?;
        }
        Ok(())
    }
    
}