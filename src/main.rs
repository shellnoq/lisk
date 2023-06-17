use piston::{WindowSettings, Event};
use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};
use piston::EventLoop;

fn main(){
    let settings = WindowSettings::new("Pelekye", (640, 480)).exit_on_esc(true);
    let mut _windows: GlutinWindow = 
        settings.build().expect("could not create window");
    
    let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut _windows) {
            
        }

    
}