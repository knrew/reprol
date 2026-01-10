//! テスト用

/// ランダムテストに用いる乱数生成器
#[cfg(test)]
type TestRng = rand_pcg::Pcg64Mcg;

/// ランダムテストで用いる固定seed
#[cfg(test)]
const SEED_U64: u64 = 0x2c6;

#[cfg(test)]
pub fn initialize_rng() -> TestRng {
    use rand::SeedableRng;
    TestRng::seed_from_u64(SEED_U64)
}
