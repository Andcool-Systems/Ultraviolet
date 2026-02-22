use std::cmp;

#[derive(Clone, Debug)]
pub struct Iter<T: Clone> {
    pub vec: Vec<T>,
    pub pos: usize,
}

impl<T: Clone> Iter<T> {
    pub fn from<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            vec: iter.into_iter().collect(),
            pos: 0,
        }
    }

    pub fn next(&mut self) -> Option<T> {
        (self.pos >= self.vec.len())
            .then_some(None)
            .unwrap_or_else(|| {
                self.pos += 1;
                Some(self.vec[self.pos - 1].clone())
            })
    }

    pub fn peek(&self, indent: Option<usize>) -> Option<T> {
        (self.pos + indent.unwrap_or(0) >= self.vec.len())
            .then(|| None)
            .unwrap_or_else(|| Some(self.vec[self.pos + indent.unwrap_or(0)].clone()))
    }

    pub fn step_back(&mut self) -> Option<T> {
        if self.pos == 0 {
            return None;
        }
        self.pos = cmp::max(0, self.pos - 1);
        Some(self.vec[self.pos].clone())
    }
}
