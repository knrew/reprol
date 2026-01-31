//! PathTracker
//!
//! 単一始点最短路探索アルゴリズムにおいて， 経路情報を管理するためのモジュール．
//!
//! `PathTracker`トレイトとその2個の実装を提供する:
//! - `WithPath`: 経路を保存し，始点から終点までの経路を構築できる．
//! - `NoPath`: 経路情報を保存しない場合のダミー．
//!
//! bfsやdijkstraなどのモジュールで使用する．

/// 経路情報を管理するためのトレイト．
pub trait PathTracker<V> {
    fn new(n: usize) -> Self;

    /// `index`の直前の頂点を返す．
    fn get_previous(&self, index: usize) -> Option<&V>;

    /// `index`の直前の頂点を`v`に更新する．
    fn set_previous(&mut self, index: usize, v: &V);

    /// 始点から`end`までの経路を構築する．
    /// 到達判定は行わない．
    fn construct_path(&self, to_index: &impl Fn(&V) -> usize, end: &V) -> Vec<V>;
}

/// 経路を保存する場合に用いる構造体．
/// 各頂点の直前の頂点を保存する．
pub struct WithPath<V> {
    previous: Vec<Option<V>>,
}

impl<V: Clone> PathTracker<V> for WithPath<V> {
    fn new(n: usize) -> Self {
        Self {
            previous: vec![None; n],
        }
    }

    fn get_previous(&self, index: usize) -> Option<&V> {
        self.previous[index].as_ref()
    }

    fn set_previous(&mut self, index: usize, value: &V) {
        self.previous[index] = Some(value.clone());
    }

    fn construct_path(&self, to_index: &impl Fn(&V) -> usize, end: &V) -> Vec<V> {
        let mut v = end;
        let mut path = vec![v];

        while let Some(pv) = self.previous[to_index(v)].as_ref() {
            path.push(pv);
            v = pv;
        }

        path.into_iter().rev().cloned().collect()
    }
}

/// 経路を保存しない場合に用いる構造体(ダミー)．
pub struct NoPath;

impl<V> PathTracker<V> for NoPath {
    #[inline(always)]
    fn new(_: usize) -> Self {
        Self
    }

    #[inline(always)]
    fn get_previous(&self, _: usize) -> Option<&V> {
        None
    }

    #[inline(always)]
    fn set_previous(&mut self, _: usize, _: &V) {}

    #[inline(always)]
    fn construct_path(&self, _: &impl Fn(&V) -> usize, _: &V) -> Vec<V> {
        panic!("NoPath::construct_path should not be called.");
    }
}
