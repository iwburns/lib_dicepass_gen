pub mod eff_long;
pub mod eff_short;
pub mod eff_short_2;

#[cfg(test)]
mod benches {
    extern crate test;
    use self::test::Bencher;
    use super::*;

    #[bench]
    fn eff_long_get(b: &mut Bencher) {
        b.iter(|| {
            let key = test::black_box("11111");
            eff_long::WORD_LIST.get(key)
        });
    }

    #[bench]
    fn eff_short_get(b: &mut Bencher) {
        b.iter(|| {
            let key = test::black_box("1111");
            eff_short::WORD_LIST.get(key)
        });
    }

    #[bench]
    fn eff_short_2_get(b: &mut Bencher) {
        b.iter(|| {
            let key = test::black_box("1111");
            eff_short_2::WORD_LIST.get(key)
        });
    }
}