use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum JprOrigin {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
}

const DEG2RAD: f64 = PI / 180.;
// 秒単位
const LAT0: [f64; 20] = [
    0.,
    33. * DEG2RAD,
    33. * DEG2RAD,
    36. * DEG2RAD,
    33. * DEG2RAD,
    36. * DEG2RAD,
    36. * DEG2RAD,
    36. * DEG2RAD,
    36. * DEG2RAD,
    36. * DEG2RAD,
    40. * DEG2RAD,
    44. * DEG2RAD,
    44. * DEG2RAD,
    44. * DEG2RAD,
    26. * DEG2RAD,
    26. * DEG2RAD,
    26. * DEG2RAD,
    26. * DEG2RAD,
    20. * DEG2RAD,
    26. * DEG2RAD,
];
// 分単位
const LONG0: [f64; 20] = [
    0.,
    7770. / 60. * DEG2RAD,
    7860. / 60. * DEG2RAD,
    7930. / 60. * DEG2RAD,
    8010. / 60. * DEG2RAD,
    8060. / 60. * DEG2RAD,
    8160. / 60. * DEG2RAD,
    8230. / 60. * DEG2RAD,
    8310. / 60. * DEG2RAD,
    8390. / 60. * DEG2RAD,
    8450. / 60. * DEG2RAD,
    8415. / 60. * DEG2RAD,
    8535. / 60. * DEG2RAD,
    8655. / 60. * DEG2RAD,
    8520. / 60. * DEG2RAD,
    7650. / 60. * DEG2RAD,
    7440. / 60. * DEG2RAD,
    7860. / 60. * DEG2RAD,
    8160. / 60. * DEG2RAD,
    9240. / 60. * DEG2RAD,
];

pub fn jpr2ll(yx: (f64, f64), origin: JprOrigin) -> (f64, f64) {
    let (y, x) = yx;

    /*
    浮動小数点演算は現時点でコンパイル時実行できないため、以下のコードを実行した結果を定数として用いる

    const F: f64 = 298.257222101;
    const N: f64 = 1. / (2. * F - 1.);

    let a0 =
        1. + (N.powf(2.)) / 4. + (N.powf(4.)) / 64.;

    let a_array: [f64; 5] = [
        -(3. / 2.) * (N - (N.powf(3.)) / 8. - (N.powf(5.)) / 64.),
        (15. / 16.) * (N.powf(2.) - (N.powf(4.)) / 4.),
        -(35. / 48.) * (N.powf(3.) - (5. / 16.) * (N.powf(5.))),
        (315. / 512.) * (N.powf(4.)),
        -(693. / 1280.) * (N.powf(5.)),
        ];

    let beta_array: [f64; 5] = [
        (1. / 2.) * N - (2. / 3.) * (N.powf(2.)) + (37. / 96.) * (N.powf(3.))
            - (1. / 360.) * (N.powf(4.))
            - (81. / 512.) * (N.powf(5.)),
        (1. / 48.) * (N.powf(2.)) + (1. / 15.) * (N.powf(3.)) - (437. / 1440.) * (N.powf(4.))
            + (46. / 105.) * (N.powf(5.)),
        (17. / 480.) * (N.powf(3.)) - (37. / 840.) * (N.powf(4.)) - (209. / 4480.) * (N.powf(5.)),
        (4397. / 161280.) * (N.powf(4.)) - (11. / 504.) * (N.powf(5.)),
        (4583. / 161280.) * (N.powf(5.)),
    ];

    let delta_array: [f64; 6] = [
        2. * N - (2. / 3.) * (N.powf(2.)) - 2. * (N.powf(3.))
            + (116. / 45.) * (N.powf(4.))
            + (26. / 45.) * (N.powf(5.))
            - (2854. / 675.) * (N.powf(6.)),
        (7. / 3.) * (N.powf(2.)) - (8. / 5.) * (N.powf(3.)) - (227. / 45.) * (N.powf(4.))
            + (2704. / 315.) * (N.powf(5.))
            + (2323. / 945.) * (N.powf(6.)),
        (56. / 15.) * (N.powf(3.)) - (136. / 35.) * (N.powf(4.)) - (1262. / 105.) * (N.powf(5.))
            + (73814. / 2835.) * (N.powf(6.)),
        (4279. / 630.) * (N.powf(4.))
            - (332. / 35.) * (N.powf(5.))
            - (399572. / 14175.) * (N.powf(6.)),
        (4174. / 315.) * (N.powf(5.)) - (144838. / 6237.) * (N.powf(6.)),
        (601676. / 22275.) * (N.powf(6.)),
    ];


    println!("const A0:f64 = {:?};", a0);
    println!("const A_ARR:[f64;5] = {:?};", a_array);
    println!("const BETA_ARR:[f64;5] = {:?};", beta_array);
    println!("const DELTA_ARR:[f64;6] = {:?};", delta_array);
     */

    const A0: f64 = 1.0000007049454078;
    const A_ARR: [f64; 5] = [
        -0.0025188297041239312,
        2.6435429493240994e-6,
        -3.4526259073074147e-9,
        4.891830424387949e-12,
        -7.228726045813916e-15,
    ];
    const BETA_ARR: [f64; 5] = [
        0.0008377321681620316,
        5.905870211016955e-8,
        1.6734826761541112e-10,
        2.1648237311010893e-13,
        3.79409187887551e-16,
    ];
    const DELTA_ARR: [f64; 6] = [
        0.003356551485604312,
        6.571873263127177e-6,
        1.7646404372866207e-8,
        5.3877538900094696e-11,
        1.7640075159133883e-13,
        6.056074055207582e-16,
    ];

    // 定数
    const M0: f64 = 0.9999;
    const A: f64 = 6378137.;
    const F: f64 = 298.257222101;
    const N: f64 = 1. / (2. * F - 1.);

    const A_: f64 = M0 * A * A0 / (1. + N);

    let lat0 = LAT0[origin as usize];
    let long0 = LONG0[origin as usize];

    let s_ = ((M0 * A) / (1. + N))
        * (A0 * lat0
            + A_ARR.iter().enumerate().fold(0., |acc, (i, &a)| {
                acc + a * (2. * (i as f64 + 1.) * lat0).sin()
            }));

    let xi = (x + s_) / A_;
    let eta = y / A_;

    let xi2 = xi
        - BETA_ARR.iter().enumerate().fold(0., |acc, (i, &b)| {
            acc + b * (2. * (i as f64 + 1.) * xi).sin() * (2. * (i as f64 + 1.) * eta).cosh()
        });

    let eta2 = eta
        - BETA_ARR.iter().enumerate().fold(0., |acc, (i, &b)| {
            acc + b * (2. * (i as f64 + 1.) * xi).cos() * (2. * (i as f64 + 1.) * eta).sinh()
        });

    let chi = (xi2.sin() / eta2.cosh()).asin();

    let lat = chi
        + DELTA_ARR.iter().enumerate().fold(0., |acc, (i, &d)| {
            acc + d * (2. * (i as f64 + 1.) * chi).sin()
        });

    let long = long0 + (eta2.sinh() / xi2.cos()).atan();

    (long / PI * 180., lat / PI * 180.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::ToPrimitive;

    #[test]
    fn it_works() {
        let (long, lat) = jpr2ll((22694.980, 11573.375), JprOrigin::Nine);

        assert_eq!(
            ((long * 1000.).floor(), (lat * 1000.).floor()),
            (
                ((140. + 5. / 60. + 8. / 3600.) * 1000.)
                    .to_f64()
                    .unwrap()
                    .floor(),
                ((36. + 6. / 60. + 15. / 3600.) * 1000.)
                    .to_f64()
                    .unwrap()
                    .floor()
            )
        );
    }
}
