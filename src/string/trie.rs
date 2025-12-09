//! Trie(木)
//!
//! `OFFSET` を基準値，`SIGMA` をアルファベットサイズとした文字列集合を管理するトライ木．
//! 文字列の挿入・検索・辞書順`n`番目の列挙などを提供する．
//!
//! # 使用例
//! ```
//! use reprol::string::trie::Trie;
//!
//! let mut trie = Trie::<b'a', 26>::new();
//! trie.insert(b"apple");
//! trie.insert(b"app");
//! trie.insert(b"banana");
//! assert!(trie.contains(b"apple"));
//! assert!(!trie.contains(b"ap"));
//! assert_eq!(trie.count_prefix(b"ap"), 2);
//! assert_eq!(trie.nth(0).as_deref(), Some(b"app".as_ref()));
//! ```

/// Trie木の各頂点を表すノード．
#[derive(Clone)]
pub struct Node<const OFFSET: u8, const SIGMA: usize> {
    childs: [Option<usize>; SIGMA],
    parent: Option<(usize, u8)>,
    count_passed: usize,
    count_terminated: usize,
}

impl<const OFFSET: u8, const SIGMA: usize> Node<OFFSET, SIGMA> {
    /// 子ノードが存在しない空ノードを生成する．
    fn new() -> Self {
        Self {
            childs: [None; SIGMA],
            parent: None,
            count_passed: 0,
            count_terminated: 0,
        }
    }

    /// 親ノードのインデックスを返す．
    pub fn parent(&self) -> Option<usize> {
        self.parent.map(|(id, _)| id)
    }

    /// 親からこのノードへ辺を張るときの文字を返す．
    pub fn parent_char(&self) -> Option<u8> {
        self.parent.map(|(_, c)| c)
    }

    /// 根ノードかどうかを判定する．
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// このノードを通過した文字列の本数を返す．
    pub fn count_passed(&self) -> usize {
        self.count_passed
    }

    /// このノードで終わる文字列の個数．
    /// 同じ文字列も重複でカウントする．
    pub fn count_terminated(&self) -> usize {
        self.count_terminated
    }

    /// 文字cの方向に進んだ次のノード．
    pub fn next(&self, c: u8) -> Option<usize> {
        let c = shift_right::<OFFSET, SIGMA>(c);
        self.childs[c]
    }
}

/// `OFFSET`基準，アルファベットサイズ`SIGMA`のトライ木．
#[derive(Clone)]
pub struct Trie<const OFFSET: u8, const SIGMA: usize = 26> {
    nodes: Vec<Node<OFFSET, SIGMA>>,
}

impl<const OFFSET: u8, const SIGMA: usize> Trie<OFFSET, SIGMA> {
    /// 根ノード1つのみを持つ空のトライを生成する．
    pub fn new() -> Self {
        Trie {
            nodes: vec![Node::new()],
        }
    }

    /// ノード数を返す．
    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// 挿入された文字列の個数．
    /// 同じ文字列も重複でカウントする．
    pub fn len(&self) -> usize {
        self.nodes[0].count_passed
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 根ノードのインデックスを返す．
    pub fn root(&self) -> usize {
        0
    }

    /// インデックス`v`のノードを返す．
    pub fn node(&self, v: usize) -> &Node<OFFSET, SIGMA> {
        &self.nodes[v]
    }

    /// 文字列`s`を挿入し，終端ノードのインデックスを返す．
    pub fn insert(&mut self, s: &[u8]) -> usize {
        let mut v = self.root();

        self.nodes[v].count_passed += 1;

        for &c_raw in s {
            let c = shift_right::<OFFSET, SIGMA>(c_raw);

            v = match self.nodes[v].childs[c] {
                Some(nv) => nv,
                None => {
                    let new_id = self.nodes.len();
                    self.nodes.push(Node::new());
                    self.nodes[new_id].parent = Some((v, c_raw));
                    self.nodes[v].childs[c] = Some(new_id);
                    new_id
                }
            };

            self.nodes[v].count_passed += 1;
        }

        self.nodes[v].count_terminated += 1;

        v
    }

    /// 文字列`s`に対応するノードを返す．
    pub fn find(&self, s: &[u8]) -> Option<usize> {
        let mut v = self.root();

        for &c in s {
            let c = shift_right::<OFFSET, SIGMA>(c);
            v = self.nodes[v].childs[c]?;
        }

        Some(v)
    }

    /// 文字列`s`が挿入済みかを返す．
    pub fn contains(&self, s: &[u8]) -> bool {
        self.find(s)
            .is_some_and(|v| self.nodes[v].count_terminated > 0)
    }

    /// `prefix`を持つ文字列の本数を返す．
    pub fn count_prefix(&self, prefix: &[u8]) -> usize {
        self.find(prefix)
            .map(|v| self.nodes[v].count_passed)
            .unwrap_or(0)
    }

    /// 挿入済み文字列を辞書順に並べたときの`n`番目を返す．
    pub fn nth(&self, n: usize) -> Option<Vec<u8>> {
        self.nth_with_prefix(&[], n)
    }

    /// `prefix`を共通接頭辞として持つ文字列のうち，辞書順`n`番目を返す．
    pub fn nth_with_prefix(&self, prefix: &[u8], mut n: usize) -> Option<Vec<u8>> {
        let start = self.find(prefix)?;
        let count_start = self.nodes[start].count_passed;

        if n >= count_start {
            return None;
        }

        let mut v = start;

        let mut res = prefix.to_vec();

        loop {
            if n < self.nodes[v].count_terminated {
                break Some(res);
            }

            n -= self.nodes[v].count_terminated;

            for c in 0..SIGMA {
                if let Some(nv) = self.nodes[v].childs[c] {
                    let count = self.nodes[nv].count_passed;

                    if n < count {
                        res.push(shift_left::<OFFSET, SIGMA>(c));
                        v = nv;
                        break;
                    }

                    n -= count;
                }
            }
        }
    }

    /// ノード`v`から根までのインデックス列を返す．
    pub fn path(&self, mut v: usize) -> Vec<usize> {
        let mut path = vec![v];

        while let Some((pv, _)) = self.nodes[v].parent {
            path.push(pv);
            v = pv;
        }

        assert_eq!(path.last(), Some(&self.root()));

        path
    }
}

impl<const OFFSET: u8, const SIGMA: usize> Default for Trie<OFFSET, SIGMA> {
    fn default() -> Self {
        Self::new()
    }
}

#[inline(always)]
fn shift_right<const OFFSET: u8, const SIGMA: usize>(c: u8) -> usize {
    debug_assert!(c >= OFFSET);
    let c = (c - OFFSET) as usize;
    debug_assert!(c < SIGMA);
    c
}

#[inline(always)]
fn shift_left<const OFFSET: u8, const SIGMA: usize>(c: usize) -> u8 {
    debug_assert!(c < SIGMA);
    c as u8 + OFFSET
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_WORDS: [&[u8]; 5] = [b"app", b"apple", b"banana", b"band", b"band"];

    fn build_sample_trie() -> Trie<b'a', 26> {
        let mut trie = Trie::new();
        for word in SAMPLE_WORDS {
            trie.insert(word);
        }
        trie
    }

    #[test]
    fn test_insert_contains_and_counts() {
        let mut trie = Trie::<b'a', 26>::default();
        assert_eq!(trie.nodes_len(), 1);
        assert!(trie.is_empty());

        for word in SAMPLE_WORDS {
            trie.insert(word);
        }

        assert_eq!(trie.len(), SAMPLE_WORDS.len());
        assert!(!trie.is_empty());
        assert!(trie.contains(b"app"));
        assert!(trie.contains(b"apple"));
        assert!(trie.contains(b"band"));
        assert!(!trie.contains(b"apricot"));
        assert_eq!(trie.count_prefix(b""), SAMPLE_WORDS.len());
        assert_eq!(trie.count_prefix(b"ban"), 3);
        assert_eq!(trie.count_prefix(b"band"), 2);
        assert_eq!(trie.count_prefix(b"bandit"), 0);

        let banana = trie.find(b"banana").unwrap();
        assert_eq!(trie.node(banana).count_passed(), 1);
        assert_eq!(trie.node(banana).count_terminated(), 1);

        let band = trie.find(b"band").unwrap();
        assert_eq!(trie.node(band).count_terminated(), 2);
        assert!(trie.node(band).parent().is_some());

        let mut dup = Trie::<b'a', 26>::new();
        let first = dup.insert(b"band");
        let second = dup.insert(b"band");
        assert_eq!(first, second);
        assert_eq!(dup.node(first).count_terminated(), 2);
    }

    #[test]
    fn test_nth_and_nth_with_prefix() {
        let trie = build_sample_trie();
        let expected: [&[u8]; 5] = [b"app", b"apple", b"banana", b"band", b"band"];

        for (i, word) in expected.iter().enumerate() {
            assert_eq!(trie.nth(i).as_deref(), Some(*word));
        }
        assert_eq!(trie.nth(expected.len()), None);

        assert_eq!(
            trie.nth_with_prefix(b"ban", 0).as_deref(),
            Some(&b"banana"[..])
        );
        assert_eq!(
            trie.nth_with_prefix(b"ban", 1).as_deref(),
            Some(&b"band"[..])
        );
        assert_eq!(
            trie.nth_with_prefix(b"ban", 2).as_deref(),
            Some(&b"band"[..])
        );
        assert_eq!(trie.nth_with_prefix(b"ban", 3), None);
        assert_eq!(trie.nth_with_prefix(b"zzz", 0), None);

        assert_eq!(
            trie.nth_with_prefix(b"app", 0).as_deref(),
            Some(&b"app"[..])
        );
        assert_eq!(
            trie.nth_with_prefix(b"app", 1).as_deref(),
            Some(&b"apple"[..])
        );
        assert_eq!(trie.nth_with_prefix(b"app", 2), None);
    }

    #[test]
    fn test_path_and_parent_characters() {
        let mut trie = build_sample_trie();
        let leaf = trie.insert(b"cat");
        let path = trie.path(leaf);

        assert_eq!(path.first(), Some(&leaf));
        assert_eq!(path.last(), Some(&trie.root()));
        assert_eq!(path.len(), b"cat".len() + 1);

        let mut chars = vec![];
        let mut v = leaf;
        while let Some(ch) = trie.node(v).parent_char() {
            chars.push(ch);
            v = trie.node(v).parent().unwrap();
        }
        assert_eq!(chars, b"cat".iter().rev().copied().collect::<Vec<_>>());
        assert!(trie.node(trie.root()).is_root());
        assert!(!trie.node(leaf).is_root());
    }
}
