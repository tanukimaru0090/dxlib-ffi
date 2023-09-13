/*dxlib struct types*/

#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
pub struct VECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
pub struct COLOR_U8 {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[repr(C)]
// ３Ｄ描画に使用する頂点データ型
pub struct VERTEX3D {
    // 座標
    pub pos: VECTOR,

    // 法線
    pub norm: VECTOR,

    // ディフューズカラー
    pub dif: COLOR_U8,

    // スペキュラカラー
    pub spc: COLOR_U8,

    // テクスチャ座標
    pub u: f32,
    pub v: f32,

    // サブテクスチャ座標
    pub su: f32,
    pub sv: f32,
}

pub type Color = u32;
