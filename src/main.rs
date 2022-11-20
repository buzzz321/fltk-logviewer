use fltk::{
    app::{self},
    button::Button,
    dialog::{self, FileDialog},
    enums::Event,
    group::{Flex, FlexType},
    input::Input,
    prelude::*,
    text::{TextBuffer, TextDisplay},
    window::*,
};
use fltk_table::{SmartTable, TableOpts};

#[derive(Debug)]
struct Matches {
    line_start: usize,
    _row: usize,
}

impl Matches {
    fn new(source_matches_col: usize, source_matches_row: usize) -> Self {
        Self {
            line_start: source_matches_col,
            _row: source_matches_row,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Message {
    Open,
    Search,
    Resize,
    Quit,
}
#[allow(dead_code)]
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
    search_result: SmartTable,
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
        wind.resize_callback(move |_, _, _, _, _| {
            send.send(Message::Resize);
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
        but_search.emit(send, Message::Search);

        let input_search = Input::default().with_size(300, 40);
        flex_r2.set_size(&mut but_search, 60);
        flex_r2.end();
        flex.set_size(&mut flex_r2, 30);

        let mut text = TextDisplay::default().with_size(300, 400);
        let buf = TextBuffer::default();
        text.set_buffer(Some(buf.clone()));

        let mut search_result = SmartTable::default()
            .with_size(300, 400)
            .with_opts(TableOpts {
                rows: 1,
                cols: 2,
                editable: true,
                ..Default::default()
            });
        search_result.set_col_header(false);
        let col_width = search_result.col_width(0) + search_result.col_width(1) - 30;
        search_result.set_col_width(1, wind.width() - col_width);

        flex.end();

        wind.resizable(&flex);
        wind.end();
        wind.show();
        /*     wind.resize_callback(move |_win, x, _y, _w, h| {
            search_result.set_col_width(1, x - col_width);
        });
        */
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
            search_result,
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
                                    Ok(_) => {
                                        self.input_path.set_value(filename.to_str().unwrap());
                                    }
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
                    Search => {
                        println!("{:?}", self.input_search.value());
                        let matches = self.finder(&self.input_search.value());
                        for m in matches {
                            let tmp: [&str; 2] = [
                                "",
                                &self
                                    .buf
                                    .line_text((m.line_start as i32).try_into().unwrap())[..],
                            ];

                            self.search_result
                                .append_row(&m.line_start.to_string(), &tmp);
                            println!("{:?}", m);
                        }
                    }
                    Resize => {
                        let col_width = self.search_result.col_width(0)
                            + self.search_result.row_header_width()
                            + 2;
                        self.search_result
                            .set_col_width(1, self.wind.width() - col_width);
                    }
                }
            }
        }
    }
    // find a key and return the line number it was found on.
    /*   fn finder_line(&mut self, key: &str) -> Vec<Matches> {
            let res = self
                .buf
                .text()
                .lines()
                .enumerate()
                .filter(|(_, line)| line.contains(key))
                .map(|(index, line)| Matches::new(line.find(key).unwrap(), index))
                .collect::<Vec<Matches>>();
            println!("{:?}", res);
            res
    }
        */
    //return all positions of a key
    fn finder(&mut self, key: &str) -> Vec<Matches> {
        let buf = &self.buf.text();

        let res = buf
            .match_indices(key)
            .map(|(m, _)| Matches::new(m, 0))
            .collect::<Vec<Matches>>();
        println!("{:?}", res);
        res
    }

    #[allow(dead_code)]
    fn find_line(&mut self, line_no: i32) -> i32 {
        let mut line: i32 = 0;
        let mut pos: i32 = 0;
        while line < line_no {
            if let Some(next_pos) = self.buf.search_forward(pos, "\n", false) {
                pos += next_pos;
            } else {
                break;
            }

            line += 1;
        }
        pos
    }
}

fn main() {
    let mut app = TheApp::new(); //creategui();

    app.launch();
}
