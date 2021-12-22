#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SequenceType {
    Right = 0,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub struct Sequence {
    pub sequence_type: SequenceType,
    pub begin: usize,
    pub len: usize,
}

impl std::cmp::PartialEq for Sequence {
    fn eq(&self, other: &Self) -> bool {
        self.begin == other.begin && self.len == other.len && self.sequence_type == other.sequence_type
    }
}

impl std::cmp::Eq for Sequence {}