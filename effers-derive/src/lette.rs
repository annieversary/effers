#[derive(Clone, Copy)]
pub struct LettersIter {
    idx: u32,
}

impl LettersIter {
    pub fn new() -> Self {
        Self {
            idx: 'A' as u32 - 1,
        }
    }
}
impl Iterator for LettersIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        for _ in 0..100 {
            self.idx += 1;
            if let Some(c) = char::from_u32(self.idx) {
                return Some(c);
            }
        }

        None
    }
}
