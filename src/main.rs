use fltk::{
    app,
    button::Button,
    dialog::{FileDialog,FileDialogOptions,FileDialogType},
    group::{Flex, FlexType},
    input::Input,
    prelude::*,
    text::{TextDisplay, TextBuffer},
    window::*,
};

fn creategui() -> app::App {
    let app = app::App::default();
    let mut wind = Window::default().with_size(640, 400).with_label("Viewer");
    // Vertical is default. You can choose horizontal using pack.set_type(PackType::Horizontal);
    
    let mut flex = Flex::default().size_of_parent().column();
    flex.set_type(FlexType::Column);

    
    let mut flex_r1 = Flex::default().size_of_parent().row();
    let mut but_path = Button::default().with_size(10, 40).with_label("Path");
    let mut input_path = Input::default().with_size(300, 40);
    flex_r1.set_size(&mut but_path,40);
    flex_r1.end();
    flex.set_size(&mut flex_r1, 30);
    
    let mut flex_r2 = Flex::default().size_of_parent().row();
    let mut but_search = Button::default().with_size(10, 40).with_label("Search");
    let mut input_search = Input::default().with_size(300, 40);
    flex_r2.set_size(&mut but_search,60);
    flex_r2.end();
    flex.set_size(&mut flex_r2, 30);

    let mut text_r3 = TextDisplay::default().with_size(300, 400);
    let buf = TextBuffer::default();
    text_r3.set_buffer(Some(buf));
    flex.end();
    but_path.set_callback(move |_|{
        let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
        dlg.set_option(FileDialogOptions::SaveAsConfirm);
        dlg.show();
        if !dlg.filename().to_string_lossy().to_string().is_empty() {
            text_r3.buffer().unwrap().append(&dlg.filename().to_string_lossy());
        }
    });
    wind.resizable(&flex);
    wind.end();
    wind.show();
    app
}

fn main() {
    println!("Hello, world!");
    let app = creategui();
   
    app.run().unwrap();
}
