use std::collections::HashMap;
use std::sync::Arc;

use launchy::{Canvas, DeviceCanvas, DeviceCanvasPoller, MidiError, MsgPollingWrapper};

mod button;
pub use button::{Button, ButtonType};

pub use launchy::Color;

pub struct Launchpad {
    canvas: DeviceCanvas<launchy::mini::Spec>,
    poller: Arc<DeviceCanvasPoller>,
    button_configurations: HashMap<Button, Arc<dyn Fn(&mut Launchpad)>>,
    current_button: Option<Button>
}

impl Launchpad {
    pub fn new() -> Result<Self, MidiError> {
        let (canvas, poller) = launchy::mini::Canvas::guess_polling()?;

        let mut self_ = Self { canvas, poller: Arc::new(poller), button_configurations: HashMap::new(), current_button: None };
        self_.clear()?;

        Ok(self_)
    }

    pub fn clear(&mut self) -> Result<(), MidiError> {
        println!("Clearing launchpad...");
        self.canvas.clear();
        self.canvas.flush()?;
        Ok(())
    }

    pub fn color_current_button(&mut self, color: Color) -> () {
        if let Some(button) = self.current_button {
            self.canvas.set(button.get_pad(), color);
        }
    }

    pub fn configure_button<F: Fn(&mut Launchpad) + 'static>(&mut self, button: Button, callback: F) {
        self.button_configurations.insert(button, Arc::new(callback));
    }

    pub fn poll(&mut self) -> Result<(), MidiError> {
        let poller = Arc::clone(&self.poller);
        for msg in poller.as_ref().iter() {
            let button = Button::from(msg.pad());
            if msg.is_press() {
                println!("Button {:?} pressed!", button);
                self.current_button = Some(button);
                if let Some(callback) = self.button_configurations.get(&button).map(|cb| Arc::clone(cb)) {
                    callback(self);
                }
            } else {
                println!("Button {:?} released!", button);
                self.color_current_button(Color::BLACK);
                self.current_button = None;
            }
            self.canvas.flush()?;
        }
        Ok(())
    }
    
}