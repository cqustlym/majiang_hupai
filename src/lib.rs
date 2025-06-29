use rand::Rng;

//单钓带边张—听两门（7张牌）
pub fn dan_diao_dai_bian_zhang() -> [u8; 4] {
    let vec: [[u8; 4]; 16] = [
        [1, 1, 1, 2],
        [1, 2, 2, 2],
        [2, 3, 3, 3],
        [2, 2, 2, 3],
        [3, 4, 4, 4],
        [3, 3, 3, 4],
        [4, 5, 5, 5],
        [4, 4, 4, 5],
        [5, 6, 6, 6],
        [5, 5, 5, 6],
        [6, 7, 7, 7],
        [6, 6, 6, 7],
        [7, 7, 7, 8],
        [7, 8, 8, 8],
        [8, 9, 9, 9],
        [8, 8, 8, 9],
    ];
    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//单钓带坎张—听两门（7张牌）
pub fn dan_diao_dai_kan_zhang4() -> [u8; 4] {
    let vec: [[u8; 4]; 14] = [
        [1, 1, 1, 3],
        [2, 2, 2, 4],
        [3, 3, 3, 5],
        [4, 4, 4, 6],
        [5, 5, 5, 7],
        [6, 6, 6, 8],
        [7, 7, 7, 9],
        [7, 9, 9, 9],
        [6, 8, 8, 8],
        [5, 7, 7, 7],
        [4, 6, 6, 6],
        [3, 5, 5, 5],
        [2, 4, 4, 4],
        [1, 3, 3, 3],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//单钓带坎张—听两门（7张牌）
pub fn dan_diao_dai_kan_zhang7() -> [u8; 7] {
    let vec: [[u8; 7]; 8] = [
        [1, 1, 1, 2, 3, 4, 6],
        [2, 2, 2, 3, 4, 5, 7],
        [3, 3, 3, 4, 5, 6, 8],
        [4, 4, 4, 5, 6, 7, 9],
        [4, 6, 7, 8, 9, 9, 9],
        [3, 5, 6, 7, 8, 8, 8],
        [2, 4, 5, 6, 7, 7, 7],
        [1, 3, 4, 5, 6, 6, 6],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//三面听—听三门（11张牌）
pub fn san_mian_ting() -> [u8; 5] {
    let vec: [[u8; 5]; 3] = [[2, 3, 4, 5, 6], [3, 4, 5, 6, 7], [4, 5, 6, 7, 8]];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//单钓双坎听—听三门（11张牌）
pub fn dan_diao_shuang_kan_ting() -> [u8; 7] {
    let vec: [[u8; 7]; 5] = [
        [1, 1, 1, 3, 5, 5, 5],
        [2, 2, 2, 4, 6, 6, 6],
        [3, 3, 3, 5, 7, 7, 7],
        [4, 4, 4, 6, 8, 8, 8],
        [5, 5, 5, 7, 9, 9, 9],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//小三明治牌型！—胡本身的牌+前后的牌！（前后没有就省略）
pub fn xiao_san_ming_zhi() -> [u8; 7] {
    let vec: [[u8; 7]; 7] = [
        [1, 1, 1, 2, 3, 3, 3],
        [2, 2, 2, 3, 4, 4, 4],
        [3, 3, 3, 4, 5, 5, 5],
        [4, 4, 4, 5, 6, 6, 6],
        [5, 5, 5, 6, 7, 7, 7],
        [6, 6, 6, 7, 8, 8, 8],
        [7, 7, 7, 8, 9, 9, 9],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//大三明治牌型！—胡本身的牌+前后的牌！（前后没有就省略）
pub fn da_san_ming_zhi() -> [u8; 10] {
    let vec: [[u8; 10]; 4] = [
        [1, 1, 1, 2, 3, 4, 5, 6, 6, 6],
        [2, 2, 2, 3, 4, 5, 6, 7, 7, 7],
        [3, 3, 3, 4, 5, 6, 7, 8, 8, 8],
        [4, 4, 4, 5, 6, 7, 8, 9, 9, 9],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}

//超大三明治！—胡所有牌！！！！九莲宝灯
pub fn jiu_lian_bao_deng() -> [u8; 13] {
    let vec = [1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9];
    vec
}

//对碰三面听—听五门（12张牌）
pub fn dui_peng_san_mian_ting() -> [u8; 10] {
    let vec: [[u8; 10]; 6] = [
        [1, 1, 1, 2, 2, 3, 3, 4, 5, 6],
        [2, 2, 2, 3, 3, 4, 4, 5, 6, 7],
        [3, 3, 3, 4, 4, 5, 5, 6, 7, 8],
        [4, 5, 6, 7, 7, 8, 8, 9, 9, 9],
        [3, 4, 5, 6, 6, 7, 7, 8, 8, 8],
        [2, 3, 4, 5, 5, 6, 6, 7, 7, 7],
    ];

    let len = vec.len();
    let mut rng = rand::rng();
    let rand_number = rng.random_range(1..len);
    let rlt = vec[rand_number];
    rlt
}
