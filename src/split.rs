pub struct Match<'a> {
    start: usize,
    end: usize,
    slice: &'a str,
}

pub struct Split<'a, P: Fn(u8) -> bool> {
    pub slice: &'a [u8],
    pub start: usize,
    pub end: usize,
    pub done: bool,
    pub max: Option<usize>,
    pub pred: P,
}

impl<'a, P: Fn(u8) -> bool> Iterator for Split<'a, P> {
    type Item = Match<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let p = self.pred;
        while self.start < self.end && p(self.slice[self.start]) {
            self.start += 1;
        }

        let start = self.start;
        while self.start < self.end && !p(self.slice[self.start]) {
            self.start += 1;
        }

        let end = self.start;
        if self.start == self.end {
            self.done = true;
        }

        if let Some(max) = self.max {
            if max == 0 {
                self.done = true;
            } else {
                self.max = Some(max - 1);
            }
        }

        Some(Match {
            start,
            end,
            slice: std::str::from_utf8_unchecked(&self.slice[start..end]),
        })
    }
}

pub fn split_n<'a, P: Fn(u8) -> bool>(s: &'a str, n: usize, p: P) -> Split<'a, P> {
    Split {
        slice: s.as_bytes(),
        start: 0,
        end: s.len(),
        done: n != 0,
        max: Some(n),
        pred: p,
    }
}

pub fn split<'a, P: Fn(u8) -> bool>(s: &'a str, p: P) -> Split<'a, P> {
    Split {
        slice: s.as_bytes(),
        start: 0,
        end: s.len(),
        done: false,
        max: None,
        pred: p,
    }
}
