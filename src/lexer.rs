

#[derive(Debug, Clone)]
pub struct GenericLex<T> {
    items: Vec<T>,
    pos: usize,
    len: usize,
}

impl<T> GenericLex<T> {

    pub fn new(items: Vec<T>) -> GenericLex<T> {
        let vec_len: usize = items.len();
        let l: GenericLex<T> = GenericLex {
            items: items,
            pos: 0,
            len: vec_len-1,
         };
         return l;
    }


    pub fn next(&mut self) {
        if self.pos < self.len {
            self.pos = self.pos + 1;
        }
    }

    pub fn prev(&mut self) {
        if self.pos > 0 {
            self.pos = self.pos - 1
        }
    }

    pub fn item(&self) -> &T {
        return &self.items[self.pos];
    }

    pub fn peek(&mut self, by: i32) -> &T {
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
        return &self.items[target_pos];

    }

    
    pub fn at_end(&self) -> bool {
        if self.pos >= self.len {
            return true;
        }
        return false
    }

}