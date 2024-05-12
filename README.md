# coordinate-transformer-rs

It provides functions to transform geocentric Cartesian coordinates, latitude and longitude, plane rectangular
coordinates, and pixel coordinates.
It also defines a structure to represent each coordinate value.

地心直交座標、緯度経度、平面直角座標、ピクセル座標の変換をう関数を提供する。
また、それぞれの座標値を表すための構造体も定義している。

## 使い方

```rust

fn example() {
    use coordinate_transformer_rs::*;

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

    // Coordinate transformations using structures
    // 構造体を使用した座標変換
    let ll = LL::new(140_f64.to_radians(), 36_f64.to_radians());
    let jpr = ll.to_jpr(JprOrigin::Nine);
    let pixel = ll.to_pixel(ZoomLv::Lv21);
}
```

## ライセンス

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

(The English in the README and comments in the source code were translated by DeepL.)
