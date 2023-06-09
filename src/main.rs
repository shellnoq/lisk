use piston::WindowSettings;

fn main(){
    let settings = WindowSettings::new("Pelekye", (640, 480)).exit_on_esc(true);
    println!("{}", settings.get_exit_on_esc());
}