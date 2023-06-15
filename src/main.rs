use piston::WindowSettings;
use glutin_window::GlutinWindow;

fn main(){
    let settings = WindowSettings::new("Pelekye", (640, 480)).exit_on_esc(true);
    let _windows: GlutinWindow = settings.build().expect("could not create window");
    println!("{}", settings.get_exit_on_esc());
    
}