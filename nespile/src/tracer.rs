use std::{cmp::Ordering, collections::BTreeSet, ops::Add};



#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct RangeSetItem<T>
    where T: Ord + Add<T, Output = T> + Copy
{
    left: T,
    length: T,
}
impl<T> RangeSetItem<T>
    where T: Ord + Add<T, Output = T> + Copy
{
    pub fn new(left: T, length: T) -> Self {
        RangeSetItem { left, length }
    }

    pub fn contains(&self, other: &T) -> bool {
        return *other >= self.left && *other < self.left + self.length;
    }

    /// Assumes the range set items _don't_ intersect.
    pub fn adjacent(&self, other: &Self) -> bool {
        if self.left < other.left {
            // Check left-to-right adjacency
            self.left + self.length == other.left
        } else { 
            other.left + other.length == self.left
        }
    }
    /// Assumes the range set items are adjacent.
    pub fn merge(self, other: Self) -> Self {
        if self.left < other.left {
            RangeSetItem{ left: self.left, length: self.length + other.length }
        } else {
            RangeSetItem{ left: other.left, length: other.length + self.length }
        }
    }
}



pub struct RangeSet<T>
    where T: Ord + Add<T, Output = T> + Copy
{
    inner: Vec<RangeSetItem<T>>,
}
impl<T: Ord + Add<T, Output = T> + Copy> RangeSet<T> {
    pub fn new() -> Self {
        RangeSet { inner: Vec::new() }
    }

    fn search_ranges(&self, other: &T) -> Result<usize, usize> {
        self.inner.binary_search_by(|item| {
            if other < &item.left {
                Ordering::Less
            } else {
                let right = item.left + item.length;
                if other >= &right { Ordering::Greater }
                else { Ordering::Equal }
            }
        })
    }

    pub fn contains(&self, other: &T) -> bool {
        self.search_ranges(other).is_ok()
    }
}
// Could make this generic across some multiplicative identity trait, but didn't want to add the `num` crate.
impl RangeSet<u16> {
    pub fn insert(&mut self, other: u16) -> bool {
        if self.inner.is_empty() {
            self.inner.push(RangeSetItem::new(other, 1));
            return true;
        }
        if let Err(idx) = self.search_ranges(&other) {
            let new_range = RangeSetItem::new(other, 1);

            let overlaps_right = self.inner.get(idx)
                .map_or(false, |right_range| new_range.adjacent(right_range));
            let overlaps_left = self.inner.get(idx - 1)
                .map_or(false, |left_range| new_range.adjacent(left_range));

            // TODO: consider mem::replace for range replacing to avoid remove and insert cost of Vec.
            let (insert_idx, insert_range) = match (overlaps_left, overlaps_right) {
                (true, true) => {
                    let right_range = self.inner.remove(idx);
                    let left_range = self.inner.remove(idx - 1);
                    (idx - 1, left_range.merge(new_range).merge(right_range))
                },
                (true, false) => {
                    let left_range = self.inner.remove(idx - 1);
                    (idx - 1, left_range.merge(new_range))
                },
                (false, true) => {
                    let right_range = self.inner.remove(idx);
                    (idx, right_range.merge(new_range))
                },
                (false, false) => (idx, new_range)
            };

            self.inner.insert(insert_idx, insert_range);
            true
        } else {
            false
        }
    }
}


pub struct TraceContext {
    read_range: RangeSet<u16>,
    write_range: RangeSet<u16>,
}