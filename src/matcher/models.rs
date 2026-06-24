use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Match {
    pub idx: usize,
    pub len: usize,
}

impl Hash for Match {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl Eq for Match {}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Match {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}
