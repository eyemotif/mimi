#[derive(Debug)]
pub struct History {
    edits: Vec<(Edit, usize)>,
    undo_offset: usize,
    save_index: usize,
}

#[derive(Debug, Clone)]
pub enum Edit {
    Add(String),
    Delete(String),
}

fn index_to_byte(index: usize, in_text: &str) -> usize {
    for (char_index, (byte_index, _)) in in_text.char_indices().enumerate() {
        if char_index == index {
            return byte_index;
        }
    }

    in_text.len()
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

        if last_edit_index.saturating_sub(at_index) >= 2
            || at_index > *last_edit_index + last_edit_text.chars().count()
        {
            self.push_edit(Edit::Add(add.to_string()), at_index);
            return;
        }

        // ab+c|d

        let position_in_last_edit = at_index - *last_edit_index;
        last_edit_text.insert(index_to_byte(position_in_last_edit, last_edit_text), add);
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
        last_edit_text.insert(index_to_byte(position_in_last_edit, last_edit_text), del);
        if position_in_last_edit == 0 {
            *last_edit_index -= 1;
        }
    }

    pub fn undo(&mut self, file: &mut String, cursor: &mut usize) -> Option<Edit> {
        if self.undo_offset >= self.edits.len() {
            return None;
        }

        self.undo_offset += 1;
        let (edit, index) = &self.edits[self.edits.len() - self.undo_offset];

        match edit {
            Edit::Add(text) => {
                for _ in text.chars() {
                    file.remove(index_to_byte(*index, file));
                }
                *cursor = *index;
            }
            Edit::Delete(text) => {
                file.insert_str(index_to_byte(*index, file), text);
                *cursor = *index + text.chars().count();
            }
        }

        Some(edit.clone())
    }
    pub fn redo(&mut self, file: &mut String, cursor: &mut usize) -> Option<Edit> {
        if self.undo_offset == 0 {
            return None;
        }

        self.undo_offset -= 1;
        let (edit, index) = &self.edits[self.edits.len() - self.undo_offset];

        match edit {
            Edit::Add(text) => {
                file.insert_str(index_to_byte(*index, file), text);
                *cursor = *index + text.chars().count();
            }
            Edit::Delete(text) => {
                for _ in text.chars() {
                    file.remove(index_to_byte(*index, file));
                }
                *cursor = *index;
            }
        }

        Some(edit.clone())
    }

    pub fn save(&mut self) {
        self.save_index = self.edits.len() - self.undo_offset;
    }
    pub fn edited_since_last_save(&self) -> bool {
        self.save_index != self.edits.len() - self.undo_offset
    }
}

impl std::fmt::Display for Edit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edit::Add(text) => f.write_fmt(format_args!("Add {text:?}")),
            Edit::Delete(text) => f.write_fmt(format_args!("Delete {text:?}")),
        }
    }
}
