const LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[derive(Clone, Copy)]
pub struct LettersIter {
    idx: usize,
}

impl LettersIter {
    pub fn new() -> Self {
        Self { idx: 0 }
    }
}
impl Iterator for LettersIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let l = LETTERS.chars().nth(self.idx % LETTERS.len()).unwrap();
        let c = self.idx / LETTERS.len();

        self.idx += 1;

        Some(l.to_string().repeat(c + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let mut i = LettersIter::new();

        assert_eq!(i.next(), Some("A".to_string()));
        assert_eq!(i.next(), Some("B".to_string()));
        assert_eq!(i.nth(23), Some("Z".to_string()));
        assert_eq!(i.next(), Some("AA".to_string()));
        assert_eq!(i.next(), Some("BB".to_string()));
    }
}
