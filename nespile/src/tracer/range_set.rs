use std::{cmp::Ordering, fmt, ops::{Add, Sub}};



pub trait Unit {
    fn unit() -> Self;
}
impl Unit for u16 {
    fn unit() -> Self { 1u16 }
}



#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
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
impl<T> RangeSetItem<T>
    where T: Ord + Add<T, Output = T> + Sub<T, Output=T> + Copy
{
    pub fn trim_start(self, count: T) -> Option<Self> {
        if count >= self.length {
            None
        } else {
            Some(RangeSetItem::new(self.left + count, self.length - count))
        }
    }
    pub fn trim_end(self, count: T) -> Option<Self> {
        if count >= self.length {
            None
        } else {
            Some(RangeSetItem::new(self.left, self.length - count))
        }
    }
}
impl<T> RangeSetItem<T>
    where T: Ord + Add<T, Output = T> + Sub<T, Output=T> + Unit + Copy
{
    pub fn split_at(self, value: T) -> (Self, Option<Self>) {
        let right = self.left + self.length;

        if value < self.left {
            (self, None)
        } else if value == self.left {
            (self.trim_start(T::unit()).unwrap(), None)
        } else if value >= right {
            (self, None)
        } else if value == right + T::unit() {
            (self.trim_end(T::unit()).unwrap(), None)
        } else {
            // split value inside of this range set item.
            let left_length = value - self.left;
            let right = self.left + self.length;
            let right_length = right - value - T::unit();

            (
                RangeSetItem::new(self.left, left_length),
                Some(RangeSetItem::new(value + T::unit(), right_length))
            )
        }
    }
}
impl<T> fmt::Debug for RangeSetItem<T>
    where T: Ord + Add<T, Output = T> + Copy + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RangeSetItem(0x{:04x?}..0x{:04x?})", self.left, self.left + self.length)
    }
}




#[derive(Clone)]
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
impl<T> RangeSet<T>
    where T: Ord + Add<T, Output = T> + Copy + Unit
{
    pub fn insert_unit(&mut self, other: T) -> bool {
        if self.inner.is_empty() {
            self.inner.push(RangeSetItem::new(other, T::unit()));
            return true;
        }
        if let Err(idx) = self.search_ranges(&other) {
            let new_range = RangeSetItem::new(other, T::unit());

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
impl<T> RangeSet<T>
    where T: Ord + Add<T, Output=T> + Sub<T, Output=T> + Copy + Unit
{
    pub fn remove(&mut self, value: T) -> bool {
        if self.inner.is_empty() {
            return false;
        }

        if let Ok(idx) = self.search_ranges(&value) {
            let range_set_item = self.inner.remove(idx);
            let (left, maybe_right) = range_set_item.split_at(value);

            self.inner.insert(idx, left);
            if let Some(right) = maybe_right {
                self.inner.insert(idx + 1, right);
            }

            true
        } else {
            false
        }
    }
}



#[cfg(test)]
mod test {
    use super::{RangeSetItem, RangeSet};

    #[test]
    fn test_range_set_item_contains() {
        let item = RangeSetItem::new(2u16, 3u16);

        assert_eq!(item.contains(&0), false);
        assert_eq!(item.contains(&1), false);
        assert_eq!(item.contains(&2), true);
        assert_eq!(item.contains(&3), true);
        assert_eq!(item.contains(&4), true);
        assert_eq!(item.contains(&5), false);
        assert_eq!(item.contains(&6), false);
    }

    #[test]
    fn test_range_set_item_adjacent() {
        let item1 = RangeSetItem::new(2u16, 3u16);

        let item2 = RangeSetItem::new(7u16, 2u16);
        assert_eq!(item1.adjacent(&item2), false);

        let item2 = RangeSetItem::new(5u16, 2u16);
        assert_eq!(item1.adjacent(&item2), true);

        let item2 = RangeSetItem::new(0u16, 1u16);
        assert_eq!(item1.adjacent(&item2), false);

        let item2 = RangeSetItem::new(0u16, 2u16);
        assert_eq!(item1.adjacent(&item2), true);
    }

    #[test]
    fn test_range_set_item_merge() {
        let item1 = RangeSetItem::new(2u16, 3u16);
        let item2 = RangeSetItem::new(5u16, 2u16);
        let expected = RangeSetItem::new(2u16, 5u16);
        let merged = item1.merge(item2);
        assert_eq!(merged, expected);

        let item1 = RangeSetItem::new(2u16, 4u16);
        let item2 = RangeSetItem::new(0u16, 2u16);
        let expected = RangeSetItem::new(0u16, 6u16);
        let merged = item1.merge(item2);
        assert_eq!(merged, expected);
    }

    #[test]
    fn test_range_set_item_trim_start() {
        let item1 = RangeSetItem::new(2u16, 5u16);
        assert!(item1.contains(&2));
        assert!(item1.contains(&3));

        let item2 = item1.trim_start(1).unwrap();
        assert!(!item2.contains(&2));
        assert!(item2.contains(&3));

        let item3 = item2.trim_start(2).unwrap();
        assert!(!item3.contains(&3));
        assert!(!item3.contains(&4));
        assert!(item3.contains(&5));
    }
    #[test]
    fn test_range_set_item_trim_end() {
        let item1 = RangeSetItem::new(2u16, 5u16);
        assert!(item1.contains(&5));
        assert!(item1.contains(&6));

        let item2 = item1.trim_end(1).unwrap();
        assert!(item2.contains(&5));
        assert!(!item2.contains(&6));

        let item3 = item2.trim_end(2).unwrap();
        assert!(item3.contains(&3));
        assert!(!item3.contains(&4));
        assert!(!item3.contains(&5));
    }
    #[test]
    fn test_range_set_item_split() {
        let item1 = RangeSetItem::new(2u16, 5u16);
        let (split_left, maybe_split_right) = item1.split_at(4);

        assert_eq!(split_left, RangeSetItem::new(2, 2));
        assert_eq!(maybe_split_right.unwrap(), RangeSetItem::new(5, 2));
    }



    #[test]
    fn test_range_set_contains() {
        let mut set = RangeSet::new();
        assert_eq!(set.contains(&3u16), false);

        set.insert_unit(3u16);
        assert_eq!(set.contains(&3u16), true);
    }

    #[test]
    fn test_range_set_merge() {
        let mut set = RangeSet::new();
        set.insert_unit(4u16);
        set.insert_unit(2u16);
        set.insert_unit(3u16);

        assert_eq!(set.inner, vec![RangeSetItem::new(2u16, 3u16)])
    }
}