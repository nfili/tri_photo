use tri_photo::app::App;
extern crate byte_unit;

fn main() ->std::io::Result<()> {
    let mut app: App = App::new();
    app.run_gui();  
    Ok(())
}