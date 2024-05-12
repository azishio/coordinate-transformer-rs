/// Transforms (longitude, latitude) and altitude expressed in the arc degree method into (x, y, z) in the Cartesian coordinate system (EPSG:4979).
///
/// 弧度法で表された(経度,緯度)と標高を直交座標系(EPSG:4979)の(x, y, z)に変換する
///
/// # Examples
///
/// Transformation from latitude/longitude and altitude to the Cartesian coordinate system.
///
/// 緯経度と標高から直交座標系への変換
///
/// ```
/// use coordinate_transformer::xyz_ll::llz2xyz;
///
/// let (long, lat) = (140_f64.to_radians(), 36_f64.to_radians());
/// let altitude = 100_f64;
///
/// let (x, y, z) = llz2xyz((long, lat), altitude);
/// ```
pub fn llz2xyz(ll: (f64, f64), altitude: f64) -> (f64, f64, f64) {
    let (long, lat) = ll;

    const A: f64 = 6378137.; // wgs84 長半径
    const F: f64 = 1. / 298.257223563; // wgs84 扁平率
    const E2: f64 = F * (2. - F); // 第一離心率の二乗

    let n = A / (1. - E2 * lat.sin().powf(2.)).sqrt(); // 卯酉線曲率半径

    let x = (n + altitude) * lat.cos() * long.cos();
    let y = (n + altitude) * lat.cos() * long.sin();
    let z = (n * (1. - E2) + altitude) * lat.sin();

    (x, y, z)
}

/// Convert (x, y, z) in the Cartesian coordinate system (EPSG:4979) to ((longitude, latitude), altitude) expressed using the arc degree method.
///
/// 直交座標系(EPSG:4979)の(x, y, z)を弧度法で表された((経度, 緯度), 標高)に変換する
///
/// # Examples
///
/// Transformation from Cartesian coordinate system to latitude, longitude and altitude
///
/// 直交座標系から緯度経度と標高への変換
///
/// ```
/// use coordinate_transformer::xyz_ll::xyz2llz;
///
/// let xyz = (-3957446.631, 3320692.008, 3728250.454);
///
/// let ((long, lat), altitude) = xyz2llz(xyz);
/// ```
pub fn xyz2llz(xyz: (f64, f64, f64)) -> ((f64, f64), f64) {
    let (x, y, z) = xyz;

    const A: f64 = 6378137.; // wgs84 長半径
    const F: f64 = 1. / 298.257223563; // wgs84 扁平率
    const E2: f64 = F * (2. - F); // 第一離心率の二乗

    let p = (x.powf(2.) + y.powf(2.)).sqrt();


    let mut lat = (z / (p * (1. - E2))).atan();

    loop {
        let n = A / (1. - E2 * lat.sin().powf(2.)).sqrt();

        let next_lat = (z / (p - E2 * n * lat.cos())).atan();

        if (lat - next_lat).abs() < 1e-12 {
            lat = next_lat;
            break;
        }
        lat = next_lat;
    }

    let long = y.atan2(x);
    let h = p / lat.cos() - A / (1. - E2 * lat.sin().powf(2.)).sqrt();

    ((long, lat), h)
}

#[cfg(test)]
mod tests {
    use close_to::assert_close_to;

    use super::*;

    #[test]
    fn llz2xyz_works() {
        let (long, lat) = (140_f64.to_radians(), 36_f64.to_radians());
        let altitude = 100_f64;

        let (x, y, z) = llz2xyz((long, lat), altitude);

        assert_close_to(x, -3957446.631, 3);
        assert_close_to(y, 3320692.008, 3);
        assert_close_to(z, 3728250.454, 3);
    }

    #[test]
    fn xyz2llz_works() {
        let xyz = (-3957446.631, 3320692.008, 3728250.454);

        let ((long, lat), altitude) = xyz2llz(xyz);

        assert_close_to(long.to_degrees(), 140., 3);
        assert_close_to(lat.to_degrees(), 36., 3);
        assert_close_to(altitude, 100., 3);
    }
}
