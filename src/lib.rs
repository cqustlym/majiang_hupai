pub mod lib {
    use rand::Rng;
    
    #[derive(Debug)]
    pub enum TingPai {
        Ting1,
        Ting2,
    }

    impl TingPai {
        pub fn random() -> TingPai {
            let mut rng = rand::rng();
            let random_num: u8 = Rng::random_range(&mut rng, 1..3);
            match random_num {
                1 => TingPai::Ting1,
                2 => TingPai::Ting2,
                _ => panic!("Invalid random number"),
            }
        }
    }

    pub struct Ting1 {}
    impl Ting1 {
        pub fn new() -> Vec<u8> {
            let mut rlt = Vec::new();
            let mut rng = rand::rng();
            let random_num: u8 = Rng::random_range(&mut rng, 1..10);
            rlt.push(random_num);
            rlt
        }
    }
}
