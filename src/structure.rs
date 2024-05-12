use crate::{jpr2ll, JprOrigin, ll2jpr, ll2pixel, llz2xyz, pixel2ll, xyz2llz, ZoomLv};

/// structure representing latitude and longitude
///
/// 緯度経度を表す構造体
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct LL {
    long: f64,
    lat: f64,
}

impl LL {
    ///create a new latitude and longitude
    ///
    /// 緯度経度を新しく作成する
    pub fn new(long: f64, lat: f64) -> Self {
        Self { long, lat }
    }

    /// Returns a tuple of (longitude,latitude)
    ///
    /// (経度,緯度)をタプルで返す
    pub fn to_tuple(&self) -> (f64, f64) {
        (self.long, self.lat)
    }

    /// Convert to a structure representing JPR coordinates
    ///
    /// JPR座標を表す構造体に変換する
    pub fn to_jpr(&self, origin: JprOrigin) -> JPR {
        let (y, x) = ll2jpr(self.to_tuple(), origin);
        JPR::new(y, x, origin)
    }

    /// Convert to a structure representing pixel coordinates
    ///
    /// Pixel座標を表す構造体に変換する
    pub fn to_pixel(&self, zoom_lv: ZoomLv) -> Pixel {
        let (x, y) = ll2pixel(self.to_tuple(), zoom_lv);
        Pixel::new(x as u32, y as u32, zoom_lv)
    }

    /// Convert to a structure representing Cartesian (EPSG:4979) coordinates
    ///
    /// 直交座標系(EPSG:4979)座標を表す構造体に変換する
    pub fn to_xyz(&self, altitude: f64) -> XYZ {
        let (x, y, z) = llz2xyz(self.to_tuple(), altitude);
        XYZ::new(x, y, z)
    }
}

/// Convert to a structure representing plane rectangular coordinates
///
/// 平面直角座標を表す構造体に変換する
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct JPR {
    y: f64,
    x: f64,
    origin: JprOrigin,
}

impl JPR {
    /// Create a new JPR coordinate
    ///
    /// JPR座標を新しく作成する
    pub fn new(x: f64, y: f64, origin: JprOrigin) -> Self {
        Self { y, x, origin }
    }

    /// Returns a tuple of (y, x)
    ///
    /// (y, x)をタプルで返す
    pub fn to_tuple(&self) -> (f64, f64) {
        (self.y, self.x)
    }

    /// Convert to a structure representing latitude and longitude
    ///
    /// 緯度経度を表す構造体に変換する
    pub fn to_ll(&self) -> LL {
        let (long, lat) = jpr2ll(self.to_tuple(), self.origin);
        LL::new(long, lat)
    }

    /// Convert to a structure representing pixel coordinates
    ///
    /// Pixel座標を表す構造体に変換する
    pub fn to_pixel(&self, zoom_lv: ZoomLv) -> Pixel {
        let (x, y) = ll2pixel(self.to_ll().to_tuple(), zoom_lv);
        Pixel::new(x as u32, y as u32, zoom_lv)
    }

    /// Convert to a structure representing Cartesian (EPSG:4979) coordinates
    ///
    /// 直交座標系(EPSG:4979)座標を表す構造体に変換する
    pub fn to_xyz(&self, altitude: f64) -> XYZ {
        let (x, y, z) = llz2xyz(self.to_ll().to_tuple(), altitude);
        XYZ::new(x, y, z)
    }
}

/// Structure representing pixel coordinates
///
/// ピクセル座標を表す構造体
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pixel {
    x: u32,
    y: u32,
    zoom: ZoomLv,
}

impl Pixel {
    /// Create a new pixel coordinate
    ///
    /// ピクセル座標を新しく作成する
    pub fn new(x: u32, y: u32, zoom: ZoomLv) -> Self {
        Self { x, y, zoom }
    }

    /// Returns a tuple of (x, y)
    ///
    /// (x, y)をタプルで返す
    pub fn to_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    /// Convert to a structure representing latitude and longitude
    ///
    /// 緯度経度を表す構造体に変換する
    pub fn to_ll(&self) -> LL {
        let (long, lat) = pixel2ll(self.to_tuple(), self.zoom);
        LL::new(long, lat)
    }

    /// Convert to a structure representing JPR coordinates
    ///
    /// 平面直角座標を表す構造体に変換する
    pub fn to_jpr(&self, origin: JprOrigin) -> JPR {
        let (y, x) = ll2jpr(self.to_ll().to_tuple(), origin);
        JPR::new(y, x, origin)
    }

    /// Convert to a structure representing Cartesian (EPSG:4979) coordinates
    ///
    /// 直交座標系(EPSG:4979)座標を表す構造体に変換する
    pub fn to_xyz(&self, altitude: f64) -> XYZ {
        let (x, y, z) = llz2xyz(self.to_ll().to_tuple(), altitude);
        XYZ::new(x, y, z)
    }
}

/// Structure with height information added to pixel coordinates
/// Height is determined according to pixel resolution (m)
///
/// ピクセル座標に高さ情報を追加した構造体
/// 高さはピクセルの分解能(m)に合わせて決定される
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Voxel {
    x: u32,
    y: u32,
    z: u32,
    resolution: f64,
    zoom_lv: ZoomLv,
}

impl Voxel {
    /// Create a new voxel coordinate
    ///
    /// Voxel座標を新しく作成する
    pub fn new(x: u32, y: u32, z: u32, resolution: f64, zoom_lv: ZoomLv) -> Self {
        Self { x, y, z, resolution, zoom_lv }
    }

    /// Returns a tuple of (x, y, z)
    ///
    /// (x, y, z)をタプルで返す
    pub fn to_tuple(&self) -> (u32, u32, u32) {
        (self.x, self.y, self.z)
    }

    /// Returns a tuple of (x, y)
    ///
    /// (x, y)をタプルで返す
    pub fn to_2d_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    /// Convert to a structure representing latitude and longitude
    ///
    /// 緯度経度を表す構造体に変換する
    pub fn to_ll(&self) -> LL {
        let (long, lat) = pixel2ll(self.to_2d_tuple(), self.zoom_lv);
        LL::new(long, lat)
    }

    /// Convert to a structure representing latitude and longitude with altitude (m)
    ///
    /// 緯度経度を表す構造体と標高(m)に変換する
    pub fn to_ll_with_altitude(&self) -> (LL, f64) {
        let ll = self.to_ll();
        let altitude = self.z as f64 * self.resolution;
        (ll, altitude)
    }

    /// Convert to a structure representing JPR coordinates
    ///
    /// 平面直角座標を表す構造体に変換する
    pub fn to_jpr(&self, origin: JprOrigin) -> JPR {
        let (y, x) = ll2jpr(self.to_ll().to_tuple(), origin);
        JPR::new(y, x, origin)
    }

    /// Convert to a structure representing JPR coordinates with altitude (m)
    ///
    /// 平面直角座標を表す構造体と標高(m)に変換する
    pub fn to_jpr_with_altitude(&self, origin: JprOrigin) -> (JPR, f64) {
        let (ll, altitude) = self.to_ll_with_altitude();
        let jpr = ll.to_jpr(origin);
        (jpr, altitude)
    }

    /// Convert to a structure representing pixel coordinates
    ///
    /// ピクセル座標を表す構造体に変換する
    pub fn to_pixel(&self) -> Pixel {
        Pixel::new(self.x, self.y, self.zoom_lv)
    }

    /// Convert to a structure representing pixel coordinates with altitude (m)
    ///
    ///　ピクセル座標を表す構造体と標高(m)に変換する
    pub fn to_pixel_with_altitude(&self) -> (Pixel, f64) {
        let (ll, altitude) = self.to_ll_with_altitude();
        let pixel = ll.to_pixel(self.zoom_lv);
        (pixel, altitude)
    }

    /// Convert to a structure representing Cartesian (EPSG:4979) coordinates
    ///
    /// 直交座標系(EPSG:4979)座標を表す構造体に変換する
    pub fn to_xyz(&self) -> XYZ {
        let (x, y, z) = llz2xyz(self.to_ll().to_tuple(), self.z as f64 * self.resolution);
        XYZ::new(x, y, z)
    }

    /// Convert to a structure representing Cartesian (EPSG:4979) coordinates with altitude (m)
    ///
    /// 直交座標系(EPSG:4979)座標を表す構造体と標高(m)に変換する
    pub fn to_xyz_with_altitude(&self) -> (XYZ, f64) {
        let (ll, altitude) = self.to_ll_with_altitude();
        let xyz = ll.to_xyz(altitude);
        (xyz, altitude)
    }
}

/// Structure representing Cartesian (EPSG:4979) coordinates
///
/// 直交座標系(EPSG:4979)座標を表す構造体
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct XYZ {
    x: f64,
    y: f64,
    z: f64,
}

impl XYZ {
    /// Create a new Cartesian (EPSG:4979) coordinate
    ///
    /// 直交座標系(EPSG:4979)座標を新しく作成する
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Returns a tuple of (x, y, z)
    ///
    /// (x, y, z)をタプルで返す
    pub fn to_tuple(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    /// Convert to a structure representing latitude and longitude
    ///
    /// 緯度経度を表す構造体に変換する
    pub fn to_ll(&self) -> LL {
        let ((long, lat), ..) = xyz2llz(self.to_tuple());
        LL::new(long, lat)
    }

    /// Convert to a structure representing latitude and longitude with altitude (m)
    ///
    /// 緯度経度を表す構造体と標高(m)に変換する
    pub fn to_ll_with_altitude(&self) -> (LL, f64) {
        let ((long, lat), altitude) = xyz2llz(self.to_tuple());
        (LL::new(long, lat), altitude)
    }

    /// Convert to a structure representing JPR coordinates
    ///
    /// 平面直角座標を表す構造体に変換する
    pub fn to_jpr(&self, origin: JprOrigin) -> JPR {
        let (y, x) = ll2jpr(self.to_ll().to_tuple(), origin);
        JPR::new(y, x, origin)
    }

    /// Convert to a structure representing JPR coordinates with altitude (m)
    ///
    /// 平面直角座標を表す構造体と標高(m)に変換する
    pub fn to_jpr_with_altitude(&self, origin: JprOrigin) -> (JPR, f64) {
        let (ll, altitude) = self.to_ll_with_altitude();
        let jpr = ll.to_jpr(origin);
        (jpr, altitude)
    }

    /// Convert to a structure representing pixel coordinates
    /// ピクセル座標を表す構造体に変換する
    pub fn to_pixel(&self, zoom_lv: ZoomLv) -> Pixel {
        let (x, y) = ll2pixel(self.to_ll().to_tuple(), zoom_lv);
        Pixel::new(x as u32, y as u32, zoom_lv)
    }

    /// Convert to a structure representing pixel coordinates with altitude (m)
    ///
    /// ピクセル座標を表す構造体と標高(m)に変換する
    pub fn to_pixel_with_altitude(&self, zoom_lv: ZoomLv) -> (Pixel, f64) {
        let (ll, altitude) = self.to_ll_with_altitude();
        let pixel = ll.to_pixel(zoom_lv);
        (pixel, altitude)
    }
}

