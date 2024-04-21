# coordinate-transformer-rs

Converts latitude and longitude, plane rectangular coordinates, and pixel coordinates.

緯経度、平面直角座標、ピクセル座標の変換を行う。

## 使い方

```rust
// Conversion from plane rectangular coordinates to longitude and latitude.
// 平面直角座標から緯経度への変換
let (long, lat) = jpr2ll((22694.980, 11573.375), JprOrigin::Nine);

// Conversion from longitude and latitude to plane rectangular coordinates.
// 緯経度から平面直角座標への変換
let (y, x) = ll2jpr(
(
140.08785504166664_f64.to_radians(),
36.103774791666666_f64.to_radians(),
),
JprOrigin::Nine,
);

// Conversion from longitude and latitude to pixel coordinates.
// 緯経度からピクセル座標への変換
let (x, y) = ll2pixel(
(139.7649308_f64.to_radians(), 35.6812405_f64.to_radians()),
ZoomLv::Lv21,
);

// Conversion from pixel coordinates to longitude and latitude.
// ピクセル座標から緯経度への変換
let (long, lat) = pixel2ll((476868027, 211407949), ZoomLv::Lv21);


// Calculate the length per pixel (m) in pixel coordinates according to the latitude and Zoom level of the arc degree method.
// 弧度法の緯度とZoomレベルに応じたピクセル座標における1ピクセルあたりの長さ(m)を計算する。
pub fn pixel_resolution(lat: f64, zoom: ZoomLv) -> f64 {
    156543.04 * lat.cos() / 2_f64.powf(zoom as i32 as f64)
}
```

## ライセンス

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

(The English in the README and comments in the source code were translated by DeepL.)
