pub struct Span {
    pub start: usize,
    pub stop: usize
}

impl Span {
    pub fn new(start: usize, stop: usize) -> Span {
        Span {
            start: start,
            stop: stop
        }
    }
}