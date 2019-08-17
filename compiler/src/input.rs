
use std::str::CharIndices;

pub struct Input<'a> {
    pub main : CharIndices<'a>,
    pub pushed : Vec<(usize, char)>,
}

impl<'a> Input<'a> {
    pub fn next(&mut self) -> Option<(usize, char)> {
        if self.pushed.len() > 0 {
            self.pushed.pop()
        }
        else {
            self.main.next()
        }
    }
    pub fn push(&mut self, index : usize, c : char) {
        self.pushed.push( (index, c) );
    }
}
