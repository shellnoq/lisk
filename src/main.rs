use glutin_window::GlutinWindow;
use piston::{WindowSettings, EventLoop};
use piston::event_loop::{EventSettings, Events};

fn main(){
    let settings = WindowSettings::new("Pelekye", (640, 480)).exit_on_esc(true);
    let mut _windows: GlutinWindow = 
        settings.build().expect("could not create window");
    
    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(e) = events.next(&mut _windows) {
        
    }

    
}