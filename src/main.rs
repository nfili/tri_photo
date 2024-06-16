use tri_photo::app::run_gui;
extern crate byte_unit;

#[macro_use]
pub extern crate tr;
fn main() ->std::io::Result<()> {
    tr_init!("locale");
    run_gui();  
    Ok(())
}