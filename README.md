# coordinate-transformer-rs

緯経度、平面直角座標、ピクセル座標の変換を行う。

## 使い方

```rust
// 平面直角座標から緯経度への変換
let (long, lat) = jpr2ll((22694.980, 11573.375), JprOrigin::Nine);

// 緯経度から平面直角座標への変換
let (y, x) = ll2jpr(
(
140.08785504166664_f64.to_radians(),
36.103774791666666_f64.to_radians(),
),
JprOrigin::Nine,
);

// 緯経度からピクセル座標への変換
let (x, y) = ll2pixel(
(139.7649308_f64.to_radians(), 35.6812405_f64.to_radians()),
ZoomLv::Lv21,
);

// ピクセル座標から緯経度への変換
let (long, lat) = pixel2ll((476868027, 211407949), ZoomLv::Lv21);
```

