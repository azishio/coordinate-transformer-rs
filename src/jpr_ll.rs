use std::f64::consts::PI;
use std::str::FromStr;

use num::cast::AsPrimitive;
use num::Integer;

/// Origin of plane rectangular coordinate system
///
/// 平面直角座標系の原点
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl JprOrigin {
    /// Attempts to convert from a type implementing `Into<u8>` to `JprOrigin`.
    /// Returns `Err` if the argument value is outside the range 1-19.
    ///
    /// `Into<u8>`を実装する型から`JprOrigin`への変換を試みます。
    /// 引数の値が1から19の範囲外の場合、`Err`を返します。
    ///
    /// # Examples
    ///
    /// ```
    /// use coordinate_transformer::jpr_ll::JprOrigin;
    ///
    /// let origin = JprOrigin::parse(9);
    /// assert_eq!(origin, Ok(JprOrigin::Nine));
    /// ```
    /// ```should_panic
    /// use coordinate_transformer::jpr_ll::JprOrigin;
    ///
    /// let origin = JprOrigin::parse(20);
    /// assert!(origin.is_ok());
    /// ```
    pub fn parse<T: Integer + AsPrimitive<u8>>(num: T) -> Result<Self, ()> {
        match num.as_() {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Eleven),
            12 => Ok(Self::Twelve),
            13 => Ok(Self::Thirteen),
            14 => Ok(Self::Fourteen),
            15 => Ok(Self::Fifteen),
            16 => Ok(Self::Sixteen),
            17 => Ok(Self::Seventeen),
            18 => Ok(Self::Eighteen),
            19 => Ok(Self::Nineteen),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for JprOrigin {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<u16> for JprOrigin {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<u32> for JprOrigin {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<u64> for JprOrigin {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<usize> for JprOrigin {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<i8> for JprOrigin {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<i16> for JprOrigin {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<i32> for JprOrigin {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<i64> for JprOrigin {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl TryFrom<isize> for JprOrigin {
    type Error = ();

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        JprOrigin::parse(value)
    }
}

impl FromStr for JprOrigin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>() {
            Ok(num) => JprOrigin::parse(num),
            Err(_) => Err(()),
        }
    }
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

/// Convert plane rectangular coordinates (y, x) to (longitude, latitude) expressed in arc degree method.
/// Origin is based on Japan Geodetic System 2011.
///
/// 平面直角座標(y, x)を弧度法で表現された(経度, 緯度)に変換する。
///　原点は日本測地系2011に基づく。
///
/// # Examples
///
/// Conversion from plane rectangular coordinates to longitude and latitude
///
/// 平面直角座標から緯経度への変換
///
/// ```
/// use coordinate_transformer::jpr_ll::{jpr2ll, JprOrigin};
///
/// let (long, lat) = jpr2ll((22694.980, 11573.375), JprOrigin::Nine);
/// ```
pub fn jpr2ll(yx: (f64, f64), origin: JprOrigin) -> (f64, f64) {
    let (y, x) = yx;

    /*
    Since floating-point arithmetic cannot be performed at compile-time at this time, the result of executing the following code is used as a constant.
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

    (long, lat)
}

/// Convert (longitude, latitude) expressed in arc degree method to plane rectangular coordinates (y, x).
/// Origin is based on Japan Geodetic System 2011.
///
/// 弧度法で表現された(経度, 緯度)を平面直角座標(y, x)に変換する。
/// 原点は日本測地系2011に基づく。
///
/// # Examples
///
/// Conversion from longitude and latitude to plane rectangular coordinates
///
/// 緯経度から平面直角座標への変換
///
/// ```
/// use coordinate_transformer::jpr_ll::{ll2jpr, JprOrigin};
///
/// let (y, x) = ll2jpr(
/// (
/// 140.08785504166664_f64.to_radians(),
/// 36.103774791666666_f64.to_radians(),
/// ),
/// JprOrigin::Nine,
/// );
/// ```
pub fn ll2jpr(ll: (f64, f64), origin: JprOrigin) -> (f64, f64) {
    let (long, lat) = ll;

    /*
    Since floating-point arithmetic cannot be performed at compile-time at this time, the result of executing the following code is used as a constant.
    浮動小数点演算は現時点でコンパイル時実行できないため、以下のコードを実行した結果を定数として用いる

    const F: f64 = 298.257222101;
    const N: f64 = 1. / (2. * F - 1.);

    let a0 = 1. + (N.powf(2.)) / 4. + (N.powf(4.)) / 64.;

    let a_arr = [
        -(3. / 2.) * (N - (N.powf(3.)) / 8. - (N.powf(5.)) / 64.),
        (15. / 16.) * (N.powf(2.) - (N.powf(4.)) / 4.),
        -(35. / 48.) * (N.powf(3.) - (5. / 16.) * (N.powf(5.))),
        (315. / 512.) * (N.powf(4.)),
        -(693. / 1280.) * (N.powf(5.)),
    ];
    let alpha_arr = [
        (1. / 2.) * N - (2. / 3.) * (N.powf(2.))
            + (5. / 16.) * (N.powf(3.))
            + (41. / 180.) * (N.powf(4.))
            - (127. / 288.) * (N.powf(5.)),
        (13. / 48.) * (N.powf(2.)) - (3. / 5.) * (N.powf(3.))
            + (557. / 1440.) * (N.powf(4.))
            + (281. / 630.) * (N.powf(5.)),
        (61. / 240.) * (N.powf(3.)) - (103. / 140.) * (N.powf(4.))
            + (15061. / 26880.) * (N.powf(5.)),
        (49561. / 161280.) * (N.powf(4.)) - (179. / 168.) * (N.powf(5.)),
        (34729. / 80640.) * (N.powf(5.)),
    ];

    println!("const A0: f64 = {};", a0);
    println!("const A_ARR: [f64; 5] = {:?};", a_arr);
    println!("const ALPHA_ARR: [f64; 5] = {:?};", alpha_arr);
     */

    let lat0 = LAT0[origin as usize];
    let long0 = LONG0[origin as usize];

    const A0: f64 = 1.0000007049454078;
    const A_ARR: [f64; 5] = [
        -0.0025188297041239312,
        2.6435429493240994e-6,
        -3.4526259073074147e-9,
        4.891830424387949e-12,
        -7.228726045813916e-15,
    ];
    const ALPHA_ARR: [f64; 5] = [
        0.0008377318247285465,
        7.608527848379248e-7,
        1.1976455002315586e-9,
        2.4291502606542468e-12,
        5.750164384091974e-15,
    ];

    // 定数
    const M0: f64 = 0.9999;
    const A: f64 = 6378137.;
    const F: f64 = 298.257222101;
    const N: f64 = 1. / (2. * F - 1.);

    const A_: f64 = ((M0 * A) / (1. + N)) * A0;

    let s_ = ((M0 * A) / (1. + N))
        * (A0 * lat0
        + A_ARR.iter().enumerate().fold(0., |acc, (i, &a)| {
        acc + a * (2. * (i as f64 + 1.) * lat0).sin()
    }));

    let lambda_c = (long - long0).cos();
    let lambda_s = (long - long0).sin();

    let t = (lat.sin().atanh()
        - ((2. * N.sqrt()) / (1. + N)) * (((2. * N.sqrt()) / (1. + N)) * lat.sin()).atanh())
        .sinh();
    let t_ = (1. + t.powf(2.)).sqrt();

    let xi2 = (t / lambda_c).atan();
    let eta2 = (lambda_s / t_).atanh();

    let x = A_
        * (xi2
        + ALPHA_ARR.iter().enumerate().fold(0., |acc, (i, &a)| {
        acc + a * (2. * (i as f64 + 1.) * xi2).sin() * (2. * (i as f64 + 1.) * eta2).cosh()
    }))
        - s_;

    let y = A_
        * (eta2
        + ALPHA_ARR.iter().enumerate().fold(0., |acc, (i, &a)| {
        acc + a * (2. * (i as f64 + 1.) * xi2).cos() * (2. * (i as f64 + 1.) * eta2).sinh()
    }));

    (y, x)
}

#[cfg(test)]
mod tests {
    use close_to::assert_close_to;

    use super::*;

    #[test]
    fn jpr2ll_works() {
        let (long, lat) = jpr2ll((22694.980, 11573.375), JprOrigin::Nine);

        assert_eq!(
            ((long * 1000.).floor(), (lat * 1000.).floor()),
            (
                ((140.0_f64 + 5. / 60. + 8. / 3600.) * 1000.)
                    .to_radians()
                    .floor(),
                ((36.0_f64 + 6. / 60. + 15. / 3600.) * 1000.)
                    .to_radians()
                    .floor()
            )
        );
        assert_close_to(long, 140.085555556_f64.to_radians(), 4);
        assert_close_to(lat, 36.1041666667_f64.to_radians(), 4);
    }

    #[test]
    fn ll2jpr_works() {
        let (y, x) = ll2jpr(
            (
                140.08785504166664_f64.to_radians(),
                36.103774791666666_f64.to_radians(),
            ),
            JprOrigin::Nine,
        );

        assert_close_to(y, 22916.2436, 4);
        assert_close_to(x, 11543.6883, 4);
    }
}
