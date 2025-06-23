use rand::Rng;

//单钓带边张—听两门（7张牌）
pub fn dan_diao_dai_bian_zhang() -> [u8; 4] {
    let vec: [[u8; 4]; 4] = [[1, 1, 1, 2], [1, 2, 2, 2], [8, 8, 8, 9], [8, 9, 9, 9]];
    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..=len);
    let rlt = vec[rand_number];
    rlt
}

//单钓带坎张—听两门（7张牌）
pub fn dan_diao_dai_kan_zhang4() -> [u8; 4] {
    let vec: [[u8; 4]; 7] = [
        [1, 1, 1, 3], 
        [2, 2, 2, 4], 
        [3, 3, 3, 5], 
        [4, 4, 4, 6],
        [5,5,5,7],
        [6,6,6,8],
        [7,7,7,9],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..=len);
    let rlt = vec[rand_number];
    rlt
}
