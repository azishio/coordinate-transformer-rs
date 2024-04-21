use std::f64::consts::PI;

/// Zoomレベルを表す列挙型。
///
/// Zoomレベルの最大は以下を理由に24とする。
/// https://github.com/mapbox/geojson-vt/issues/87
#[derive(Clone, Copy)]
pub enum ZoomLv {
    Lv0,
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5,
    Lv6,
    Lv7,
    Lv8,
    Lv9,
    Lv10,
    Lv11,
    Lv12,
    Lv13,
    Lv14,
    Lv15,
    Lv16,
    Lv17,
    Lv18,
    Lv19,
    Lv20,
    Lv21,
    Lv22,
    Lv23,
    Lv24,
}

/// 緯度と経度をピクセル座標に変換する関数。
/// 弧度法で与えられた(経度, 緯度)をZoomレベルに応じたピクセル座標(x, y)に変換する。
///
/// # Examples
/// 緯経度をピクセル座標に変換する。
///
/// ```
/// use coordinate_transformer::pixel_ll::{ll2pixel, ZoomLv};
///
/// let (x, y) = ll2pixel(
/// (139.7649308_f64.to_radians(), 35.6812405_f64.to_radians()),
/// ZoomLv::Lv21,
/// );
/// ```
pub fn ll2pixel(ll: (f64, f64), zoom: ZoomLv) -> (u32, u32) {
    let (long, lat) = ll;
    const L: f64 = 85.05112878;

    let x = (2_f64.powf(zoom as i32 as f64 + 7.)) * (long / PI + 1.);
    let y = (2_f64.powf(zoom as i32 as f64 + 7.) / PI)
        * (-(lat.sin().atanh()) + (L * PI / 180.).sin().atanh());

    (x as u32, y as u32)
}

/// ピクセル座標を緯度と経度に変換する関数。
/// Zoomレベルに応じたピクセル座標(x, y)を弧度法で表された(経度, 緯度)に変換する。
///
/// # Examples
/// ピクセル座標を緯経度に変換する。
///
/// ```
/// use coordinate_transformer::pixel_ll::{pixel2ll, ZoomLv};
///
/// let (long, lat) = pixel2ll((476868027, 211407949), ZoomLv::Lv21);
/// ```
pub fn pixel2ll(pixel: (u32, u32), zoom: ZoomLv) -> (f64, f64) {
    let (x, y) = pixel;
    const L: f64 = 85.05112878;

    let long = PI * (x as f64 / 2_f64.powf(zoom as i32 as f64 + 7.) - 1.);
    let lat = ((-PI * y as f64 / (2_f64.powf(zoom as i32 as f64 + 7.))
        + (PI * L / 180.).sin().atanh())
    .tanh())
    .asin();

    (long, lat)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ll2pixel_works() {
        let (x, y) = ll2pixel(
            (139.7649308_f64.to_radians(), (35.6812405_f64).to_radians()),
            ZoomLv::Lv21,
        );

        assert_eq!((x, y), (476868027, 211407949));
    }

    #[test]
    fn pixel2ll_works() {
        let (long, lat) = pixel2ll((476868027, 211407949), ZoomLv::Lv21);

        println!("{}, {}", long.to_degrees(), lat.to_degrees());

        assert_eq!(
            (
                (139.7649308_f64.to_radians() * 1000.).floor(),
                (35.6812405_f64.to_radians() * 1000.).floor()
            ),
            ((long * 1000.).floor(), (lat * 1000.).floor())
        );
    }
}
