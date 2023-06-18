use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
use piston::{WindowSettings, EventLoop};
use piston::event_loop::{EventSettings, Events};

fn main(){
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Pelekye", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);

    // let mut _windows: GlutinWindow = 
    //     settings.build().expect("could not create window");
    
    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(e) = events.next(&mut _windows) {}

    
}