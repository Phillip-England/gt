
#[derive(Debug, Clone)]
pub struct Lexer<T> {
    items: Vec<T>,
    pub pos: usize,
    len: usize,
    pub marked_pos: usize,
}

impl<T: Clone> Lexer<T> {

    pub fn new(items: Vec<T>) -> Lexer<T> {
        let vec_len: usize = items.len();
        let l: Lexer<T> = Lexer {
            items: items,
            pos: 0,
            len: vec_len-1,
            marked_pos: 0,
         };
         return l;
    }


    pub fn next(&mut self) {
        if self.pos < self.len {
            self.pos = self.pos + 1;
        }
    }

    pub fn next_by(&mut self, by: usize) {
        let ends_at: usize = self.pos + by;
        if ends_at < self.len {
            self.pos = ends_at;
            return;
        }
        self.pos = self.len;
    }

    pub fn prev(&mut self) {
        if self.pos > 0 {
            self.pos = self.pos - 1
        }
    }

    pub fn item(&self) -> T {
        return self.items[self.pos].clone();
    }

    pub fn peek(&mut self, by: i32) -> T {
        let current_pos = self.pos;
        let mut count = 0;

        loop {
            if by < 0 {
                self.prev();
            } else {
                self.next();
            }
            count = count + 1;
            if count >= by.abs() {
                break;
            }
        }
        let target_pos = self.pos.clone();
        self.pos = current_pos;
        return self.items[target_pos].clone();

    }

    
    pub fn at_end(&self) -> bool {
        if self.pos >= self.len {
            return true;
        }
        return false
    }


    pub fn go_to(&mut self, i: usize) {
        self.pos = i;
    }

    pub fn pos(&self) -> usize {
        return self.pos.clone();
    }


    // returns true is we found the search target
    pub fn  next_until(&mut self, _stop_at: T) -> bool {
        self.next();
        loop {
            if matches!(self.item(), _stop_at) {
                return true;
            }
            if self.at_end() {
                return true;
            }   
            self.next();
        }
    }

    pub fn mark(&mut self) {
        self.marked_pos = self.pos;
    }

    pub fn go_to_mark(&mut self) {
        self.pos = self.marked_pos;
    }
    
    pub fn marked_pos(&self) -> usize {
        return self.marked_pos.clone();
    }

    pub fn collect(&mut self, mut start_index: usize, end_index: usize) -> Vec<T> {
        if start_index > self.len {
            start_index = self.len;
        }
        let col = self.items[start_index..end_index].iter().map(|c| {
            (*c).clone()
        });
        let mut vec = vec![];
        for c in col {
            vec.push(c);
        }
        return vec;

    }

}