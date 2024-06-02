use std::ops::Range;

#[derive(Debug)]
pub struct Void {
    from: usize,
    to: usize
}

impl Void {
    pub fn from(range: Range<usize>) -> Void {
        Void {
            from: range.start,
            to: range.end
        }
    }
}

pub struct IndexVoid {
    gaps: Vec<Void>
}

impl IndexVoid {
    pub fn new() -> IndexVoid {
        IndexVoid {
            gaps: Vec::new()
        }
    }
    
    pub fn del_gap(&self) -> usize {
        let mut all = 0usize;
        for gap in &self.gaps {
            all += gap.to - gap.from;
        }
        all
    }

    pub fn clear(&mut self) {
        self.gaps.clear();
    }

    pub fn real_index(&self, im_idx: usize) -> usize {
        let mut cursor = im_idx;
        for gap in &self.gaps {
            if gap.from < cursor {
                cursor += gap.to - gap.from;
            }
        }
        cursor
    }

    pub fn insert(&mut self, gap: Void) {
        self.gaps.push(gap);
    }
}