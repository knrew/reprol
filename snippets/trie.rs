/// 例: `let mut trie = Trie::new(26, b'a');`
pub struct Trie<T> {
    sigma: usize,
    offset: T,
    child: Vec<Vec<Option<usize>>>,
    parent: Vec<Option<usize>>,
}

impl<T> Trie<T>
where
    T: Char,
{
    pub fn new(sigma: usize, offset: T) -> Self {
        Self {
            sigma,
            offset,
            child: vec![vec![None; sigma]],
            parent: vec![None],
        }
    }

    pub fn insert(&mut self, str: &[T]) -> usize {
        let mut cur = 0;
        for &c in str {
            let c = c.sub_as_usize(self.offset);
            if self.child[cur][c].is_none() {
                self.parent.push(Some(cur));
                self.child[cur][c] = Some(self.child.len());
                self.child.push(vec![None; self.sigma]);
            }
            cur = self.child[cur][c].unwrap();
        }
        cur
    }
}

#[allow(private_in_public)]
trait Char: Copy {
    fn sub_as_usize(self, rhs: Self) -> usize;
}

impl Char for u8 {
    fn sub_as_usize(self, rhs: u8) -> usize {
        (self - rhs) as usize
    }
}

impl Char for char {
    fn sub_as_usize(self, rhs: Self) -> usize {
        (self as u8 - rhs as u8) as usize
    }
}
