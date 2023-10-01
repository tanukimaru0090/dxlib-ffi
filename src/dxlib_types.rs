/*dxlib struct types*/
use std::os::raw::*;
#[repr(C)]
pub struct RECT {
    pub left: c_int,
    pub top: c_int,
    pub right: c_int,
    pub bottom: c_int,
}
#[repr(C)]
pub struct VECTOR {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Clone for VECTOR {
    fn clone(&self) -> Self {
        VECTOR {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
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

#[repr(C)]
pub struct DATEDATA {
    pub Year: c_int, // 年
    pub Mon: c_int,  // 月
    pub Day: c_int,  // 日
    pub Hour: c_int, // 時間
    pub Min: c_int,  // 分
    pub Sec: c_int,  // 秒
}
// DirectX
#[repr(C)]
pub struct XAUDIO2FX_REVERB_PARAMETERS {
    pub WetDryMix: f32,
    pub ReflectionsDelay: u32,
    pub ReverbDelay: u8,
    pub RearDelay: u8,
    pub PositionLeft: u8,
    pub PositionRight: u8,
    pub PositionMatrixLeft: u8,
    pub PositionMatrixRight: u8,
    pub EarlyDiffusion: u8,
    pub LateDiffusion: u8,
    pub LowEQGain: u8,
    pub LowEQCutoff: u8,
    pub HighEQGain: u8,
    pub HighEQCutoff: u8,
    pub RoomFilterFreq: f32,
    pub RoomFilterMain: f32,
    pub RoomFilterHF: f32,
    pub ReflectionsGain: f32,
    pub ReverbGain: f32,
    pub DecayTime: f32,
    pub Density: f32,
    pub RoomSize: f32,
}

#[repr(C)]
pub struct IPDATA {
    d1: c_uchar,
    d2: c_uchar,
    d3: c_uchar,
    d4: c_uchar,
}
#[repr(C)]
pub struct MATRIX {
    pub m: [[f32; 4]; 4],
}
//pub type TCHAR = u16;
pub type Color = u32;
