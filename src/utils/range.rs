// Ranges are exclusive i.e. [start, end[
#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        Range { start, end }
    }

    pub fn intersect(&self, other: &Range) -> Option<(i32, i32)> {
        if other.end <= self.start || self.end <= other.start {
            return None;
        }

        let start = std::cmp::max(self.start, other.start);
        let end = std::cmp::min(self.end, other.end);

        Some((start, end))
    }

    pub fn union(&self, other: &Range) -> Vec<Range> {
        if self.overlap(other) {
            return vec![Range {
                start: std::cmp::min(self.start, other.start),
                end: std::cmp::max(self.end, other.end),
            }];
        }

        vec![self.clone(), other.clone()]
    }

    pub fn overlap(&self, other: &Range) -> bool {
        if other.end <= self.start || self.end <= other.start {
            return false;
        }
        return true;
    }
}

pub fn union_vecs(a: &Vec<Range>, b: &Vec<Range>) -> Vec<Range> {
    let mut ranges = a.clone();
    ranges.extend(b.clone());

    return union_vec(&ranges);
}

pub fn union_vec(ranges: &Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }

    // Sort ranges by start
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|r| r.start);

    let mut result = vec![];
    let mut current = sorted_ranges[0];

    for range in sorted_ranges.iter().skip(1) {
        if current.end >= range.start {
            // Merge overlapping or adjacent ranges
            current.end = std::cmp::max(current.end, range.end);
        } else {
            result.push(current);
            current = *range;
        }
    }

    result.push(current);

    result
}
