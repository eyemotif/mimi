#[derive(Debug)]
pub struct History {
    edits: Vec<(Edit, usize)>,
    undo_offset: usize,
    save_index: usize,
}

#[derive(Debug)]
enum Edit {
    Add(String),
    Delete(String),
}

impl History {
    fn push_edit(&mut self, edit: Edit, at_index: usize) {
        for _ in 0..self.undo_offset {
            self.edits.pop();
        }
        self.undo_offset = 0;
        self.edits.push((edit, at_index));
    }

    pub fn new() -> Self {
        History {
            edits: Vec::new(),
            undo_offset: 0,
            save_index: 0,
        }
    }
    pub fn edit_add(&mut self, add: char, at_index: usize) {
        let Some((last_edit, last_edit_index)) = self.edits.last_mut() else {
            self.push_edit(Edit::Add(add.to_string()), at_index);
            return;
        };

        let Edit::Add(last_edit_text) = last_edit else {
            self.push_edit(Edit::Add(add.to_string()), at_index);
            return;
        };

        if at_index < *last_edit_index - 1
            || at_index > *last_edit_index + last_edit_text.chars().count()
        {
            self.push_edit(Edit::Add(add.to_string()), at_index);
            return;
        }

        let position_in_last_edit = *last_edit_index + last_edit_text.chars().count() - at_index;
        last_edit_text.insert(position_in_last_edit, add);
        if position_in_last_edit == 0 {
            *last_edit_index -= 1;
        }
    }
    pub fn edit_del(&mut self, del: char, at_index: usize) {
        let Some((last_edit, last_edit_index)) = self.edits.last_mut() else {
            self.push_edit(Edit::Delete(del.to_string()), at_index);
            return;
        };

        let Edit::Delete(last_edit_text) = last_edit else {
            self.push_edit(Edit::Delete(del.to_string()), at_index);
            return;
        };

        if at_index < *last_edit_index - 1
            || at_index > *last_edit_index + last_edit_text.chars().count()
        {
            self.push_edit(Edit::Delete(del.to_string()), at_index);
            return;
        }

        let position_in_last_edit = *last_edit_index + last_edit_text.chars().count() - at_index;
        last_edit_text.insert(position_in_last_edit, del);
        if position_in_last_edit == 0 {
            *last_edit_index -= 1;
        }
    }

    pub fn undo(&mut self, file: &mut String, cursor: &mut usize) {
        if self.undo_offset >= self.edits.len() - 1 {
            return;
        }

        self.undo_offset += 1;
        let (edit, index) = &self.edits[self.edits.len() - self.undo_offset];

        match edit {
            Edit::Add(text) => {
                for _ in text.chars() {
                    file.remove(*index);
                }
                *cursor = *index;
            }
            Edit::Delete(text) => {
                file.insert_str(*index, text);
                *cursor = *index + text.chars().count();
            }
        }
    }
    pub fn redo(&mut self, file: &mut String, cursor: &mut usize) {
        if self.undo_offset == 0 {
            return;
        }

        self.undo_offset -= 1;
        let (edit, index) = &self.edits[self.edits.len() - self.undo_offset];

        match edit {
            Edit::Add(text) => {
                file.insert_str(*index, text);
                *cursor = *index + text.chars().count();
            }
            Edit::Delete(text) => {
                for _ in text.chars() {
                    file.remove(*index);
                }
                *cursor = *index;
            }
        }
    }

    pub fn save(&mut self) {
        self.save_index = self.edits.len().saturating_sub(1);
    }
    pub fn edited_since_last_save(&self) -> bool {
        self.save_index != self.edits.len() - 1 - self.undo_offset
    }
}
