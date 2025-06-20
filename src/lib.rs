pub mod lib {
    use rand::Rng;

    #[derive(Debug)]
    pub enum TingPai {
        Ting1(Ting1),
        Ting2,
    }

    impl TingPai {
        pub fn random() -> Self {
            let mut rng = rand::rng();
            let random_num: u8 = Rng::random_range(&mut rng, 1..3);
            match random_num {
                1 => Ting1(Ting1),
                2 => TingPai::Ting2,
                _ => panic!("Invalid random number"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Ting1 {
        TingDanDiao,
        TingKaZhang,
        TingBianZhang,
    }
    impl Ting1 {
        pub fn random() -> Self {
            let mut rng = rand::rng();
            let random_num: u8 = Rng::random_range(&mut rng, 1..4);
            match random_num {
                1 => Self::TingDanDiao,
                2 => Self::TingKaZhang,
                3 => Self::TingBianZhang,
                _ => panic!("Invalid random number"),
            }
        }
    }
}
