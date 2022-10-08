use fltk::{
    app,
    button::Button,
    dialog::{self, FileDialog, FileDialogOptions, FileDialogType},
    enums::Event,
    group::{Flex, FlexType},
    input::Input,
    prelude::*,
    text::{TextBuffer, TextDisplay},
    window::*,
};

#[derive(Copy, Clone)]
pub enum Message {
    Open,
    Quit,
}

pub struct TheApp {
    app: app::App,
    wind: Window,
    recv: app::Receiver<Message>,
    buf: TextBuffer,
    text: TextDisplay,
    but_path: Button,
    but_search: Button,
    input_path: Input,
    input_search: Input,
}

impl TheApp {
    pub fn new() -> Self {
        let app = app::App::default();
        let mut wind = Window::default().with_size(640, 400).with_label("Viewer");
        // Vertical is default. You can choose horizontal using pack.set_type(PackType::Horizontal);
        let (send, recv) = app::channel::<Message>();

        wind.set_callback(move |_| {
            if app::event() == Event::Close {
                send.send(Message::Quit);
            }
        });

        let mut flex = Flex::default().size_of_parent().column();
        flex.set_type(FlexType::Column);

        let mut flex_r1 = Flex::default().size_of_parent().row();
        let mut but_path = Button::default().with_size(10, 40).with_label("Path");
        let input_path = Input::default().with_size(300, 40);
        flex_r1.set_size(&mut but_path, 40);
        but_path.emit(send, Message::Open);

        flex_r1.end();
        flex.set_size(&mut flex_r1, 30);

        let mut flex_r2 = Flex::default().size_of_parent().row();
        let mut but_search = Button::default().with_size(10, 40).with_label("Search");
        let input_search = Input::default().with_size(300, 40);
        flex_r2.set_size(&mut but_search, 60);
        flex_r2.end();
        flex.set_size(&mut flex_r2, 30);

        let mut text = TextDisplay::default().with_size(300, 400);
        let buf = TextBuffer::default();
        text.set_buffer(Some(buf.clone()));
        flex.end();

        wind.resizable(&flex);
        wind.end();
        wind.show();
        Self {
            app,
            wind,
            recv,
            buf,
            text,
            but_path,
            but_search,
            input_path,
            input_search,
        }
    }
    pub fn launch(&mut self) {
        while self.app.wait() {
            use Message::*;
            if let Some(msg) = self.recv.recv() {
                match msg {
                    Open => {
                        let mut dlg = FileDialog::new(dialog::FileDialogType::BrowseFile);
                        dlg.set_option(dialog::FileDialogOptions::NoOptions);
                        dlg.set_filter("*.{txt,rs,toml,*.log}");
                        dlg.show();
                        let filename = dlg.filename();
                        if !filename.to_string_lossy().to_string().is_empty() {
                            if filename.exists() {
                                match self.buf.load_file(&filename) {
                                    Ok(_) => self.input_path.set_value(filename.to_str().unwrap()),
                                    Err(e) => dialog::alert(
                                        200,
                                        100,
                                        &format!("Couldnt load the file: {}", e),
                                    ),
                                }
                            } else {
                                dialog::alert(200, 100, "File does not exist!")
                            }
                        }
                    }
                    Quit => {
                        self.app.quit();
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut app = TheApp::new(); //creategui();

    app.launch();
}
