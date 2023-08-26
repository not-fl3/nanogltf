#[rustfmt::ignore]
const B64: [i32; 124] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 63, 62, 62, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 0, 0, 0, 0, 63, 0, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 0,
];

pub fn decode(data: &str) -> Vec<u8> {
    let p = data.as_bytes();
    let len = data.len();
    let pad = len > 0 && (len % 4 != 0 || p[len - 1] == '=' as u8);
    let l = ((len + 3) / 4 - pad as usize) * 4;
    let mut res = vec![0; l / 4 * 3 + pad as usize];
    let mut j = 0;

    for i in (0..l as usize).step_by(4) {
        let n = B64[p[i] as usize] << 18
            | B64[p[i + 1] as usize] << 12
            | B64[p[i + 2] as usize] << 6
            | B64[p[i + 3] as usize];
        res[j] = (n >> 16) as u8;
        j += 1;
        res[j] = (n >> 8 & 0xFF) as u8;
        j += 1;
        res[j] = (n & 0xFF) as u8;
        j += 1;
    }
    if pad {
        let n = B64[p[l] as usize] << 18 | B64[p[l + 1] as usize] << 12;
        let res_len = res.len();
        res[res_len - 1] = (n >> 16) as u8;

        if len > l + 2 && p[l + 2] != '=' as u8 {
            let n = n | B64[p[l + 2] as usize] << 6;
            res.push((n >> 8 & 0xFF) as u8);
        }
    }
    return res;
}
