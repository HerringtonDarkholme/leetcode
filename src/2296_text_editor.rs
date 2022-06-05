struct TextEditor {
    content: Vec<char>,
    pos: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    fn new() -> Self {
        Self {
            content: vec![],
            pos: 0,
        }
    }

    fn add_text(&mut self, text: String) {
        drop(self.content.splice(self.pos..self.pos, text.chars()));
        self.pos += text.len();
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k as usize;
        let start = if self.pos >= k { self.pos - k } else { 0 };
        let ret = self.pos - start;
        self.content.drain(start..self.pos);
        self.pos = start;
        ret as i32
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let k = k as usize;
        if self.pos <= k {
            self.pos = 0;
            return "".to_owned();
        }
        self.pos -= k;
        let start = if self.pos > 10 { self.pos - 10 } else { 0 };
        return self
            .content
            .get(start..self.pos)
            .unwrap()
            .into_iter()
            .collect();
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let k = k as usize;
        self.pos = (self.pos + k).min(self.content.len());
        let start = if self.pos > 10 { self.pos - 10 } else { 0 };
        return self
            .content
            .get(start..self.pos)
            .unwrap()
            .into_iter()
            .collect();
    }
}
