use std::mem::{swap, take};

#[derive(Clone)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

fn meld<T: Ord>(lhs: Option<Box<Node<T>>>, rhs: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    match (lhs, rhs) {
        (Some(mut lhs), Some(mut rhs)) => {
            if lhs.value < rhs.value {
                swap(&mut lhs, &mut rhs);
            }
            lhs.right = meld(lhs.right, Some(rhs));
            swap(&mut lhs.left, &mut lhs.right);
            Some(lhs)
        }
        (Some(lhs), None) => Some(lhs),
        (None, Some(rhs)) => Some(rhs),
        (None, None) => None,
    }
}

#[derive(Clone)]
pub struct SkewHeap<T> {
    data: Option<Box<Node<T>>>,
}

impl<T: Ord> SkewHeap<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    pub fn push(&mut self, item: T) {
        self.meld(SkewHeap {
            data: Some(Box::new(Node::new(item))),
        });
    }

    pub fn pop(&mut self) -> Option<T> {
        let Node { value, left, right } = *take(self).data?;
        *self = SkewHeap {
            data: meld(left, right),
        };
        Some(value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.as_ref().map(|node| &node.value)
    }

    pub fn meld(&mut self, other: Self) {
        *self = SkewHeap {
            data: meld(take(self).data, other.data),
        }
    }

    pub fn append(&mut self, other: &mut Self) {
        self.meld(take(other));
    }
}

impl<T> Default for SkewHeap<T> {
    fn default() -> Self {
        Self { data: None }
    }
}

impl<T: Ord> Extend<T> for SkewHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        iter.into_iter().for_each(|x| self.push(x));
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for SkewHeap<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T: Ord> FromIterator<T> for SkewHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = Self::new();
        res.extend(iter);
        res
    }
}

impl<T: Ord> From<Vec<T>> for SkewHeap<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_iter(v)
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for SkewHeap<T> {
    fn from(array: [T; N]) -> Self {
        Self::from_iter(array)
    }
}

// #[derive(Clone)]
// pub struct IntoIter<T> {
//     iter: vec::IntoIter<T>,
// }

// impl<T> IntoIterator for SkewHeap<T> {
//     type Item = T;
//     type IntoIter = IntoIter<T>;

//     fn into_iter(self) -> IntoIter<T> {
//         IntoIter {
//             iter: self.data.into_iter(),
//         }
//     }
// }

// impl<'a, T> IntoIterator for &'a SkewHeap<T> {
//     type Item = &'a T;
//     type IntoIter = Iter<'a, T>;

//     fn into_iter(self) -> Iter<'a, T> {
//         self.iter()
//     }
// }

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use super::*;

    #[test]
    fn test_skew_heap() {
        let mut heap = SkewHeap::new();
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.peek(), None);
        heap.push(5);
        heap.push(3);
        heap.push(7);
        assert_eq!(heap.peek(), Some(&7));
        assert_eq!(heap.pop(), Some(7));
        heap.push(4);
        assert_eq!(heap.peek(), Some(&5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.peek(), Some(&4));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.peek(), Some(&3));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);

        let mut heap = SkewHeap::from([Reverse(12), Reverse(3), Reverse(8), Reverse(5)]);
        assert_eq!(heap.peek(), Some(&Reverse(3)));
        assert_eq!(heap.pop(), Some(Reverse(3)));
        assert_eq!(heap.peek(), Some(&Reverse(5)));
        assert_eq!(heap.pop(), Some(Reverse(5)));
        assert_eq!(heap.peek(), Some(&Reverse(8)));
        assert_eq!(heap.pop(), Some(Reverse(8)));
        assert_eq!(heap.peek(), Some(&Reverse(12)));
        assert_eq!(heap.pop(), Some(Reverse(12)));
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_skew_heap_meld() {
        let mut heap1 = SkewHeap::new();
        heap1.push(2);
        heap1.push(8);

        let mut heap2 = SkewHeap::new();
        heap2.push(1);
        heap2.push(5);

        heap1.meld(heap2);

        assert_eq!(heap1.pop(), Some(8));
        assert_eq!(heap1.pop(), Some(5));
        assert_eq!(heap1.pop(), Some(2));
        assert_eq!(heap1.pop(), Some(1));
        assert_eq!(heap1.pop(), None);
    }
}
