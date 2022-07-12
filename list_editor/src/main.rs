#[macro_use]
mod rbp;
use rbp::{vt_close, vt_open};

mod list_editor;
use list_editor::List;

fn main() {
    vt_open();
    let mut list = List {
        item_list: String::new(),
        copied_item: String::new(),
        next_index: 1,
    };

    List::main_loop(&mut list);
    vt_close("\n     Press any key to quit...");
}
