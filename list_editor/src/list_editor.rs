use crate::rbp::*;

/* Data structure */
pub struct List {
    pub item_list: String,
    pub copied_item: String,
    pub next_index: usize,
}

/* Methods */
impl List {
    pub fn main_loop(&mut self) {
        let mut options = vec_of_strings![
            "MENU", "Create", "Read", "Update", "Delete", "Insert", "Copy", "Cut", "Paste", "Load",
            "Save", "Exit"
        ];

        loop {
            let choice = vt_menu(&mut options);
            match choice {
                1 => self.create_loop(),
                2 => self.read_list(),
                3 => self.update_item(),
                4 => self.delete_loop(),
                5 => self.insert_item(),
                6 => self.copy_item(),
                7 => self.cut_item(),
                8 => self.paste_item(),
                9 => self.load_file(),
                10 => self.save_file(),
                11 => break,
                _ => (),
            }
        }
    }

    fn create_loop(&mut self) {
        vt_cls();

        loop {
            if self.next_index > 1 {
                self.print_list();
            }

            vt_put_slice("\n     Next item? ");
            let new_item = vt_input(80);
            if new_item == "" {
                break;
            }

            let f = format!("{}\n", new_item.trim());
            self.item_list.push_str(&f);
            self.next_index += 1;
        }
    }

    fn print_list(&mut self) {
        vt_cls();
        let mut result = String::from("\n     LIST\n\n");
        let mut count: usize = 1;

        while count < self.next_index {
            let f = format!("     {}. {}\n", count, nth_line(&self.item_list, count - 1));
            result.push_str(&f);
            count += 1;
        }

        vt_put_slice(&result);
    }

    fn read_list(&mut self) {
        if self.next_index > 1 {
            self.print_list();
            vt_put_slice("\n     Press any key to continue...");
            let _ = vt_key_i32();
        }
    }

    fn update_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to update? ");

            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_int(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            let old_item = nth_line(&self.item_list, num - 1);
            let updated_item =
                vt_edit_prompt("     UPDATE\n\n   > ", &mut old_item.to_string(), 80);

            self.item_list = replace_line_at(&self.item_list, &updated_item.trim(), num - 1);
            self.read_list();
            break;
        }
    }

    fn delete_loop(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to delete? ");

            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_int(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            self.item_list = remove_nth_line(&self.item_list, num - 1);
            self.next_index -= 1;
        }
    }

    fn insert_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to insert? ");
            let new_item = vt_input(80);
            if new_item.trim() == "" {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Position to insert? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_int(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            self.item_list = insert_line_at(&self.item_list, &new_item, num - 1);
            self.next_index += 1;

            self.read_list();
            break;
        }
    }

    fn copy_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to copy? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_int(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            self.copied_item = nth_line(&self.item_list, num - 1).to_string();
            break;
        }
    }

    fn cut_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Item to cut? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_int(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            self.copied_item = nth_line(&self.item_list, num - 1).to_string();
            self.item_list = remove_nth_line(&self.item_list, num - 1);
            self.next_index -= 1;

            self.read_list();
            break;
        }
    }

    fn paste_item(&mut self) {
        loop {
            if self.next_index == 1 {
                break;
            }

            self.print_list();
            vt_put_slice("\n     Position to paste? ");
            let num_string = vt_input(2);
            if num_string == "" || !is_pos_int(&num_string) {
                break;
            }

            let num = to_int(&num_string) as usize;
            if num >= self.next_index {
                break;
            }

            self.item_list = insert_line_at(&self.item_list, &self.copied_item, num - 1);
            self.next_index += 1;

            self.read_list();
            break;
        }
    }

    fn load_file(&mut self) {
        if self.next_index > 1 {
            self.print_list();
        }

        vt_put_slice("\n     File to load? ");
        let name = vt_input(80);

        self.item_list = read_from_file(&name);
        self.next_index = line_count(&self.item_list) + 1;
        self.read_list();
    }

    fn save_file(&mut self) {
        if self.next_index > 1 {
            self.print_list();
        }

        vt_put_slice("\n     File to save? ");
        let name = vt_input(80);

        write_to_file(&name, &self.item_list);
        self.read_list();
    }
}
