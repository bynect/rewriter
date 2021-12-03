#[derive(Eq, PartialEq, Debug)]
pub struct Match<'a> {
    pub start: usize,
    pub end: usize,
    pub slice: &'a str,
}

pub struct Split<'a> {
    slice: &'a [u8],
    start: usize,
    end: usize,
    done: bool,
    max: Option<usize>,
    pred: fn(u8) -> bool,
}

impl<'a> Iterator for Split<'a> {
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

        let mut end = self.start;
        if self.start == self.end {
            self.done = true;
        }

        if let Some(max) = self.max {
            if max <= 1 {
                end = self.end;
                self.done = true;
            } else {
                self.max = Some(max - 1);
            }
        }

        if start == end {
            return None;
        }

        Some(Match {
            start,
            end,
            slice: std::str::from_utf8(&self.slice[start..end]).unwrap(),
        })
    }
}

pub fn split_n<'a>(s: &'a str, n: usize, p: fn(u8) -> bool) -> Split<'a> {
    Split {
        slice: s.as_bytes(),
        start: 0,
        end: s.len(),
        done: n == 0,
        max: Some(n),
        pred: p,
    }
}

pub fn split<'a>(s: &'a str, p: fn(u8) -> bool) -> Split<'a> {
    Split {
        slice: s.as_bytes(),
        start: 0,
        end: s.len(),
        done: false,
        max: None,
        pred: p,
    }
}

pub fn split_n_whitespace<'a>(s: &'a str, n: usize) -> Split<'a> {
    split_n(s, n, |b| b == b' ' || b == b'\t' || b == b'\n')
}
