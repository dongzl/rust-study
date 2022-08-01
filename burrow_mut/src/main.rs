

struct File {
    rows: Vec<Row>,
    filename: Option<String>
}

impl File {
    fn row(&self, index: u16) -> &Row{
        &self.rows[index as usize]
    }

    fn row_mut(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }
}

struct Row {
    string: String,
}

struct EditorState {
    file: File,
}

impl Row {
    fn insert(&mut self, at: u16, c: char) {
        let at = at as usize;
        if at >= self.string.len() {
            self.string.push(c);
        } else {
            self.string.insert(at, c)
        }
    }
}

fn update_row(mut state: &mut EditorState)  {
    let row = &state.file.row(0);
    row.insert(0, 'a');

}

fn main() {

}