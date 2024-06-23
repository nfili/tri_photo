use gtk::{
	Box,
	Align,
	Button,
	Image,
	Label,
	Orientation,
	prelude::{BoxExt,ButtonExt,WidgetExt}
};

pub fn new<F>(title: &str,callback: F) -> Box
	where
	F: Fn() + 'static {
		let hbar =base(title);
    	let hbar_quit = quit();
		let btn_quit = Button::new();
		btn_quit.set_child(Some(&hbar_quit));
		btn_quit.set_css_classes(&["btn_hb"]);
        hbar.append(&btn_quit);
        btn_quit.connect_clicked(move |_| {
         	callback
        ()});
        hbar
}
pub fn new_without_widget(title: &str) -> Box {
	base(title)
}
pub fn new_with_widget_at_end<F>(title: &str, widgets: Vec<&Button>,callback: F) -> Box
	where
	F: Fn() + 'static {
		let hbar = base(title);
    	let hbar_quit = quit();
		let btn_quit = Button::new();
		btn_quit.set_child(Some(&hbar_quit));
        for wid in widgets{
        	hbar.append(wid);
        }
        hbar.append(&btn_quit);
        btn_quit.set_css_classes(&["btn_hb"]);
        btn_quit.connect_clicked(move |_| {
         	callback
        ()});
        hbar
}
fn base(title: &str)->Box{
	let hbar = Box::new(Orientation::Horizontal,0);
	hbar.set_hexpand(true);
    hbar.set_css_classes(&["header_bar"]);

    let hbar_img = img();
    let hbar_title = lab_title(title);
    hbar.append(&hbar_img);
    hbar.append(&hbar_title);
    hbar
}
fn img() -> Image {
		gtk::Image::builder()
		// .icon_name("tp")
		.file("/usr/share/icons/hicolor/32x32/apps/tp.png")
    	.margin_start(0)
    	.width_request(32)
    	.build()
}
fn lab_title(title: &str) ->Label {
	 gtk::Label::builder()
    	.label(title)
    	.hexpand(true)
    	.halign(Align::Center)
    	.build()
}
fn quit()-> Image {
		let img = gtk::Image::builder()
    	.resource("/org/gtk_rs/tri_photo/hb_quit.png")
    	.width_request(8)
    	.build();
    	img.set_css_classes(&["btn_lab_hb","btn_lab_quit_hb"]);
    	img
}