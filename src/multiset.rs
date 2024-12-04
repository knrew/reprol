use std::{
    collections::BTreeMap,
    ops::{Add, Sub},
};

/// 多重集合
/// NOTE: Copyできる型Tのみ
pub struct MultiSet<T> {
    map: BTreeMap<T, usize>,
    len: usize,
    sum: T,
}

#[allow(dead_code)]
impl<T> MultiSet<T>
where
    T: Copy + Ord + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(zero: T) -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
            sum: zero,
        }
    }

    /// 挿入されている要素の総和
    /// 複数挿入されている要素がある場合、その分総和に含まれる
    pub fn sum(&self) -> T {
        self.sum
    }

    /// 要素数
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 挿入されているvの個数
    /// 挿入されていなければ0を返す
    pub fn count(&self, value: &T) -> usize {
        *self.map.get(value).unwrap_or(&0)
    }

    /// valueを挿入する
    /// 既に挿入されている場合はカウントを1増やす
    pub fn insert(&mut self, value: T) {
        *self.map.entry(value).or_insert(0) += 1;
        self.len += 1;
        self.sum = self.sum + value;
    }

    /// valueをremoveする
    /// 挿入されていない場合何もしない
    /// 複数挿入されている場合1個のみをremoveする(カウントを1減らす)
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(count) = self.map.get_mut(value) {
            *count -= 1;
            // if *count == 0 {
            //     self.map.remove(value);
            // }
            self.len -= 1;
            self.sum = self.sum - *value;
            true
        } else {
            false
        }
    }

    /// valueが1個以上含まれているかどうか
    pub fn contains(&self, value: &T) -> bool {
        match self.map.get(value) {
            Some(&count) if count > 0 => true,
            _ => false,
        }
    }

    pub fn min(&self) -> Option<&T> {
        if let Some(res) = self.map.iter().min() {
            Some(res.0)
        } else {
            None
        }
    }

    pub fn max(&self) -> Option<&T> {
        if let Some(res) = self.map.iter().max() {
            Some(res.0)
        } else {
            None
        }
    }

    /// 最小値をpopして返す
    /// 複数挿入されている場合1個のみをpopする(カウントを1減らす)
    pub fn pop_min(&mut self) -> Option<T> {
        if let Some(&res) = self.min() {
            self.remove(&res);
            Some(res)
        } else {
            None
        }
    }

    /// 最大値をpopして返す
    /// 複数挿入されている場合1個のみをpopする(カウントを1減らす)
    pub fn pop_max(&mut self) -> Option<T> {
        if let Some(&res) = self.max() {
            self.remove(&res);
            Some(res)
        } else {
            None
        }
    }
}
