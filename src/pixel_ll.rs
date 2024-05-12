use std::f64::consts::PI;

/// Enumerated type representing the Zoom level.
///
/// Zoomレベルを表す列挙型。
///
/// The maximum Zoom level shall be 24 for [this](https://github.com/mapbox/geojson-vt/issues/87) reason.
///
/// Zoomレベルの最大は[これ](https://github.com/mapbox/geojson-vt/issues/87)を理由に24とする。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Function to convert longitude and latitude to pixel coordinates.
/// Converts (longitude, latitude) given by the arc degree method to pixel coordinates (x, y) according to Zoom level.
///
/// 緯度と経度をピクセル座標に変換する関数。
/// 弧度法で与えられた(経度, 緯度)をZoomレベルに応じたピクセル座標(x, y)に変換する。
///
/// # Examples
///
/// Convert longitude and latitude to pixel coordinates.
///
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

/// Function to convert pixel coordinates to longitude and latitude.
/// Converts pixel coordinates (x, y) according to Zoom level to (longitude, latitude) expressed in arc degree method.
///
/// ピクセル座標を緯度と経度に変換する関数。
/// Zoomレベルに応じたピクセル座標(x, y)を弧度法で表された(経度, 緯度)に変換する。
///
/// # Examples
///
/// Convert pixel coordinates to longitude and latitude.
///
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

/// Function to return the length per pixel (m) in pixel coordinates according to the latitude and Zoom level of the arc degree method.
/// 
/// 弧度法の緯度とZoomレベルに応じたピクセル座標における1ピクセルあたりの長さ(m)を返す関数。
///
/// # Examples
///
/// Calculate the length per pixel (m) in pixel coordinates according to the latitude and Zoom level of the arc degree method.
///
/// 弧度法の緯度とZoomレベルに応じたピクセル座標における1ピクセルあたりの長さ(m)を計算する。
/// ```
/// use coordinate_transformer::pixel_ll::{pixel_resolution, ZoomLv};
///
/// let resolution = pixel_resolution(0_f64.to_radians(), ZoomLv::Lv17);
///```
pub fn pixel_resolution(lat: f64, zoom: ZoomLv) -> f64 {
    156543.04 * lat.cos() / 2_f64.powf(zoom as i32 as f64)
}

/// Function to convert pixel coordinates to tile coordinates.
///
/// ピクセル座標をタイル座標に変換する関数。
///
/// # Examples
///
/// Calculate tile coordinates from pixel coordinates.
///
/// ピクセル座標からタイル座標を計算する。
///
/// ```
/// use coordinate_transformer::pixel_ll::pixel2tile;
///
/// let (tile_x, tile_y) = pixel2tile((476868027, 211407949));
/// ```
pub fn pixel2tile(pixel: (u32, u32)) -> (u32, u32) {
    let (x, y) = pixel;
    (x / 256, y / 256)
}

#[cfg(test)]
mod tests {
    use assert_be_close::assert_be_close;

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

        assert_be_close(139.7649308_f64.to_radians(), long, 5);
        assert_be_close(35.6812405_f64.to_radians(), lat, 5);
    }

    #[test]
    fn pixel_resolution_works() {
        let equator_length_m = 40075_f64 * 1000_f64;
        let zoom_lv = ZoomLv::Lv17;
        let resolution = pixel_resolution(0_f64.to_radians(), zoom_lv);

        assert_be_close(
            resolution,
            equator_length_m / (2_f64.powf(zoom_lv as i32 as f64) * 256.),
            5,
        );
    }
}
