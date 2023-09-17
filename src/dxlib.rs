
#![allow(non_snake_case)]
extern crate encoding_rs;
//extern crate alloc;
use self::encoding_rs::SHIFT_JIS;
pub use crate::dxlib_common::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::*;
use std::vec::Vec;
//エンコーディング変換などのヘルパーやオペレーターのオーバーロード
pub trait Encode {
    fn to_shiftjis(&self) -> CString;
    fn to_cstring(&self) -> CString;
}
impl Encode for &str {
    // &strをshiftjisのエンコーディングとして変換し、CStringを返す
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(self);
        let mut string = CString::new(res).unwrap();
        return string;
    }
    // &strをデフォルトのUTF-8のエンコーディングとして変換し、CStringを返す
    fn to_cstring(&self) -> CString {
        return CString::new(self.as_bytes()).unwrap();
    }
}
impl Encode for String {
    // Stringをshiftjisのエンコーディングとして変換し、CStringを返す
    fn to_shiftjis(&self) -> CString {
        let (res, _enc, errors) = SHIFT_JIS.encode(&self);
        let mut string = CString::new(res).unwrap();
        return string;
    }
    // StringをデフォルトのUTF-8のエンコーディングとして変換し、CStringを返す
    fn to_cstring(&self) -> CString {
        return CString::new(self.clone().into_bytes()).unwrap();
    }
}

/// dxlib function extern declaration (based on Ver3.22a)
/// see: https://dxlib.xsrv.jp/dxfunc.html
#[link(name = "DxLib_x64")]
#[no_mangle]
extern "stdcall" {
    // 使用必須関数

    /// ライブラリの初期化
    pub fn dx_DxLib_Init() -> i32;
    /// ライブラリ使用の終了関数
    pub fn dx_DxLib_End() -> i32;
    /// ウインドウズのメッセージを処理する
    pub fn dx_ProcessMessage() -> i32;

    pub fn dx_SetWindowSizeExtendRate(ExRateX:f64,ExRateY:f64)->i32;
    //算術演算関数
    pub fn dx_VGet(x: f32, y: f32, z: f32) -> VECTOR;

    // ３Ｄ関係関数
    pub fn dx_MV1AttachAnim(
        MHandle: i32,
        AnimIndex: i32,
        AnimSrcMHandle: i32,
        NameCheck: i32,
    ) -> i32;

    pub fn dx_MV1GetAnimNum(MHandle: i32) -> i32;
    pub fn dx_MV1DetachAnim(MHandle: i32, AttachIndex: i32) -> i32;
    pub fn dx_MV1SetAttachAnimTime(MHandle: i32, AttachIndex: i32, Time: f32) -> i32;
    pub fn dx_MV1GetAttachAnimTotalTime(MHandle: i32, AttachIndex: i32) -> f32;
    pub fn dx_MV1GetAttachAnimTime(MHandle: i32, AttachIndex: i32) -> f32;
    pub fn dx_MV1SetAttachAnimBlendRate(MHandle: i32, AttachIndex: i32, Rate: f32) -> i32;
    pub fn dx_SetCameraPositionAndTarget_UpVecY(Position: VECTOR, Target: VECTOR) -> i32;
    pub fn dx_SetCameraNearFar(Near: f32, Far: f32) -> i32;
    // Zバッファを使うかどうかのフラグ
    pub fn dx_SetUseZBuffer3D(Flag: i32) -> i32;
    // Zバッファへの書き込みするかどうかのフラグ
    pub fn dx_SetWriteZBuffer3D(Flag: i32) -> i32;

    // モデルの読み込み
    //pub fn dx_MV1LoadModel(FileName: *const c_char) -> i32;
    // モデルの描画
    pub fn dx_MV1DrawModel(MHandle: i32) -> i32;
    pub fn dx_MV1DeleteModel(MHandle: i32) -> i32;
    //　モデルの位置の指定
    pub fn dx_MV1SetPosition(MHandle: i32, Position: VECTOR) -> i32;
    pub fn dx_MV1SetScale(MHandle: i32, Scale: VECTOR) -> i32;

    pub fn dx_DrawLine3D(Pos1: VECTOR, Pos2: VECTOR, Color: Color) -> i32;
    pub fn dx_DrawTriangle3D(
        Pos1: VECTOR,
        Pos2: VECTOR,
        Pos3: VECTOR,
        Color: Color,
        FillFlag: i32,
    ) -> i32;

    pub fn dx_DrawSphere3D(
        CenterPos: VECTOR,
        r: f32,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;

    pub fn dx_DrawCapsule3D(
        Pos1: VECTOR,
        Pos2: VECTOR,
        r: f32,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;

    pub fn dx_DrawCone3D(
        TopPos: VECTOR,
        BottomPos: VECTOR,
        r: f32,
        DivNum: i32,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: i32,
    ) -> i32;

    pub fn dx_DrawBillboard3D(
        Pos: VECTOR,
        cx: f32,
        cy: f32,
        Size: f32,
        Angle: f32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    pub fn dx_DrawPolygon3D(
        Vertex: *mut VERTEX3D,
        PolygonNum: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    /*psb fn dx_DrawPolygonIndexed3D(Vertex:*mut VERTEX3D,VertexNum:i32, *Indices, int PolygonNum, int GrHandle, int TransFlag ) ;
     */

    // Live2D関係関数
    // unimplemented!();

    // 図形描画関数

    /// 線を描画
    pub fn dx_DrawLine(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) -> i32;
    /// 線を描画(アンチエイリアス効果付き)
    pub fn dx_DrawLineAA(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) -> i32;
    /// 四角を描画
    pub fn dx_DrawBox(x1: i32, y1: i32, x2: i32, y2: i32, color: Color, fill_flag: i32) -> i32;
    /// 四角を描画(アンチエイリアス効果付き)
    pub fn dx_DrawBoxAA(x1: f32, y1: f32, x2: f32, y2: f32, color: Color, fill_flag: i32) -> i32;
    /// 円の描画
    pub fn dx_DrawCircle(x: i32, y: i32, r: i32, color: Color, fill_flag: i32) -> i32;
    /// 円の描画(アンチエイリアス効果付き)
    pub fn dx_DrawCircleAA(
        x: f32,
        y: f32,
        r: f32,
        pos_num: i32,
        color: Color,
        fill_flag: i32,
    ) -> i32;
    /// 楕円の描画
    //pub fn dx_DrawOval() -> i32;
    /// 楕円の描画(アンチエイリアス効果付き)
    //pub fn dx_DrawOvalAA() -> i32;
    /// 三角形の描画
    //pub fn dx_DrawTriangle() -> i32;
    /// 三角形の描画(アンチエイリアス効果付き)
    //pub fn dx_DrawTriangleAA() -> i32;
    /// 点を描画する
    //pub fn dx_DrawPixel() -> i32;
    /// 指定点の色を取得
    //pub fn dx_GetPixel() -> i32;

    // グラフィックデータ制御関数

    /// 画像ファイルを読みこんで画面に表示する
    //pub fn dx_LoadGraphScreen() -> i32;
    /// 画像ファイルのメモリへの読みこみ、及び動画ファイルのロード
    //pub fn dx_LoadGraph() -> i32;
    /// 画像ファイルのメモリへの分割読みこみ
    //pub fn dx_LoadDivGraph(FileName:*const c_char,AllNum:i32,XNum:i32,YNum:i32,XSize:i32,YSize:i32,HandleBuf:*mut i32) -> i32;
    /// 空のグラフィックを作成する
    pub fn dx_MakeGraph(SizeX: i32, SizeY: i32) -> i32;
    /// 描画対象にできるグラフィックを作成する
    pub fn dx_MakeScreen(SizeX: i32, SizeY: i32, UseAlphaChannel: i32) -> i32;
    /// 描画対象にできるグラフィックのマルチサンプリング設定を行う
    //pub fn dx_SetCreateDrawValidGraphMultiSample() -> i32;
    /// 作成するグラフィックのビット深度を設定する
    //pub fn dx_SetCreateGraphColorBitDepth() -> i32;
    /// 描画可能な浮動小数点型のグラフィックを作成するかどうかの設定を行う
    //pub fn dx_SetDrawValidFloatTypeGraphCreateFlag() -> i32;
    /// 作成する描画可能なグラフィックのチャンネル数の設定を行う
    //pub fn dx_SetCreateDrawValidGraphChannelNum() -> i32;
    /// 読み込み時に画像を乗算済みα画像に変換するかを設定する
    //pub fn dx_SetUsePremulAlphaConvertLoad() -> i32;
    /// メモリに読みこんだグラフィックの描画
    pub fn dx_DrawGraph(x: i32, y: i32, GrHandle: i32, TransFlag: i32) -> i32;
    // メモリに読み込んだグラフィックの描画　(float版)
    pub fn dx_DrawGraphF(x: f32, y: f32, GrHandle: i32, TransFlag: i32) -> i32;
    /// メモリに読みこんだグラフィックのＬＲ反転描画
    pub fn dx_DrawTurnGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    /// メモリに読みこんだグラフィックの拡大縮小描画
    pub fn dx_DrawExtendGraph(
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        GrHandle: i32,
        TransFlag: i32,
    ) -> i32;
    /// メモリに読みこんだグラフィックの回転描画
    pub fn dx_DrawRotaGraph(
        x: i32,
        y: i32,
        ex_rate: f64,
        angle: f64,
        gr_handle: i32,
        trans_flag: i32,
        turn_flag: i32,
    ) -> i32;
    /// メモリに読みこんだグラフィックの回転描画(回転中心指定あり)
    //pub fn dx_DrawRotaGraph2() -> i32;
    /// メモリに読みこんだグラフィックの回転描画(回転中心指定あり、縦横拡大率別指定)
    //pub fn dx_DrawRotaGraph3() -> i32;
    /// メモリに読みこんだグラフィックの自由変形描画
    //pub fn dx_DrawModiGraph() -> i32;
    /// グラフィックの指定矩形部分のみを描画
    //pub fn dx_DrawRectGraph() -> i32;
    /// 指定のグラフィックの指定部分だけを抜き出して新たなグラフィックを作成する
    //pub fn dx_DerivationGraph() -> i32;
    /// 描画先に設定されているグラフィック領域から指定領域のグラフィックを読みこむ
    //pub fn dx_GetDrawScreenGraph() -> i32;
    /// グラフィックのサイズを得る
    //pub fn dx_GetGraphSize() -> i32;
    /// 読みこんだグラフィックデータをすべて削除する
    //pub fn dx_InitGraph() -> i32;
    /// 指定のグラフィックをメモリ上から削除する
    //pub fn dx_DeleteGraph() -> i32;
    /// 描画モードをセットする
    //pub fn dx_SetDrawMode() -> i32;
    /// 描画の際のブレンドモードをセットする
    pub fn dx_SetDrawBlendMode(blend_mode: i32, pal: i32) -> i32;
    /// 描画輝度をセット
    //pub fn dx_SetDrawBright() -> i32;
    /// グラフィックに設定する透過色をセットする
    //pub fn dx_SetTransColor() -> i32;
    /// 画像ファイルからブレンド画像を読み込む
    //pub fn dx_LoadBlendGraph() -> i32;
    /// ブレンド画像と通常画像を合成して描画する
    //pub fn dx_DrawBlendGraph() -> i32;
    /// 画像にフィルター処理を施す
    //pub fn dx_GraphFilter() -> i32;
    /// 画像にフィルター処理を施す( 出力先画像指定版 )
    //pub fn dx_GraphFilterBlt() -> i32;
    /// 画像にフィルター処理を施す( 出力先画像、使用矩形指定版 )
    //pub fn dx_GraphFilterRectBlt() -> i32;
    /// 二つの画像を特殊効果付きでブレンドする
    //pub fn dx_GraphBlend() -> i32;
    /// 二つの画像を特殊効果付きでブレンドする( 出力先画像指定版 )
    //pub fn dx_GraphBlendBlt() -> i32;
    /// 二つの画像を特殊効果付きでブレンドする( 出力先画像、使用矩形指定版 )
    //pub fn dx_GraphBlendRectBlt() -> i32;

    // 文字描画関係関数

    /// 文字列を描画する
    //  pub fn dx_DrawString(x: i32, y: i32, string: *const c_char, color: Color) -> i32;
    /// 書式付き文字列を描画する
    //pub fn dx_DrawFormatString(x:i32,y:i32,color:Color,format:*const c_char, ...) -> i32;
    /// DrawString で描画される文字列の幅(ドット単位)を得る
    //pub fn dx_GetDrawStringWidth() -> i32;
    /// DrawFormatString 関数書式付き文字列の描画幅(ドット単位)を得る
    //pub fn dx_GetDrawFormatStringWidth() -> i32;
    /// 描画する文字列のサイズをセットする
    pub fn dx_SetFontSize(size: i32) -> i32;
    /// 描画する文字列の文字の太さをセットする
    //pub fn dx_SetFontThickness() -> i32;
    /// 描画するフォントを変更する
    //pub fn dx_ChangeFont(name: *const c_char) -> i32;
    /// 文字列描画に使用するフォントのタイプを変更する
    //pub fn dx_ChangeFontType() -> i32;
    /// 新しいフォントデータを作成
    //pub fn dx_CreateFontToHandle(FontName: *const c_char,Size: i32,Thick: i32,FontType: i32) -> i32;
    /// ＤＸフォントデータファイルを読み込む
    // pub fn dx_LoadFontDataToHandle(FileName: *const c_char, EdgeSize: i32) -> i32;
    /// フォントデータを削除する
    pub fn dx_DeleteFontToHandle(FontHandle: i32) -> i32;
    /// 作成するフォントデータを『乗算済みα』用にするかどうかを設定する
    //pub fn dx_SetFontCacheUsePremulAlphaFlag() -> i32;
    /// 指定のフォントデータで文字列を描画する
    // pub fn dx_DrawStringToHandle(x: i32,y: i32,String: *const c_char,Color: Color,FontHandle: i32,) -> i32;
    /// 指定のフォントデータで書式付き文字列を描画する
    //pub fn dx_DrawFormatStringToHandle() -> i32;
    /// 指定のフォントデータで描画する文字列の幅を得る
    //pub fn dx_GetDrawStringWidthToHandle() -> i32;
    /// 指定のフォントデータで書式付き文字列の描画幅を得る
    //pub fn dx_GetDrawFormatStringWidthToHandle() -> i32;
    /// 指定のフォントデータの情報を得る
    //pub fn dx_GetFontStateToHandle() -> i32;
    /// フォントデータを全て初期化する
    //pub fn dx_InitFontToHandle() -> i32;

    // 簡易画面出力関数

    //pub fn dx_() -> i32;
    // 簡易文字列描画

    //pub unsafe fn  dx_printfDx(arg: T, arg2: U, mut args: ...) -> i32;
    /// 簡易画面出力履歴をクリアする
    //pub fn dx_clsDx() -> i32;

    // その他画面操作系関数

    /// 画面モードの変更
    pub fn dx_SetGraphMode(SizeX: i32, SizeY: i32, ColorBitNum: i32, RefreshRate: i32) -> i32;
    /// フルスクリーンモード時の解像度モードを設定する
    //pub fn dx_SetFullScreenResolutionMode() -> i32;
    /// フルスクリーンモード時の画面拡大モードを設定する
    //pub fn dx_SetFullScreenScalingMode() -> i32;
    /// 現在の画面の大きさとカラービット数を得る
    //pub fn dx_GetScreenState() -> i32;
    /// 描画可能領域のセット
    //pub fn dx_SetDrawArea() -> i32;
    /// 画面に描かれたものを消去する
    //pub fn dx_ClearDrawScreen() -> i32;
    /// 画面の背景色を設定する
    //pub fn dx_SetBackgroundColor() -> i32;
    /// 色コードを取得する
    pub fn dx_GetColor(Red: i32, Green: i32, Blue: i32) -> Color;
    pub fn dx_GetColorU8(Red: i32, Green: i32, Blue: i32, Alpha: i32) -> COLOR_U8;
    /// 描画先グラフィック領域の指定
    pub fn dx_SetDrawScreen(DrawScreen: i32) -> i32;
    /// フリップ関数、画面の裏ページ(普段は表示されていない)の内容を表ページ(普段表示されている)に反映する
    pub fn dx_ScreenFlip() -> i32;
    /// 画面のフルスクリーンアンチエイリアスモードの設定をする
    //pub fn dx_SetFullSceneAntiAliasingMode() -> i32;

    // 動画関係関数

    /// 動画ファイルの再生
    //pub fn dx_PlayMovie(FileName:*const c_char,ExRate:i32,PlayType:i32) -> i32;
    /// ムービーグラフィックの動画の再生を開始する
    //pub fn dx_PlayMovieToGraph() -> i32;
    /// ムービーグラフィックの動画再生を一時停止する
    //pub fn dx_PauseMovieToGraph() -> i32;
    /// ムービーグラフィックの動画の再生位置を変更する
    //pub fn dx_SeekMovieToGraph() -> i32;
    /// ムービーグラフィックの動画の再生位置を得る
    //pub fn dx_TellMovieToGraph() -> i32;
    /// ムービーグラフィックの動画の再生状態を得る
    //pub fn dx_GetMovieStateToGraph() -> i32;

    // マスク関係関数

    /// マスク画面を作成する
    //pub fn dx_CreateMaskScreen() -> i32;
    /// マスク画面を削除する
    //pub fn dx_DeleteMaskScreen() -> i32;
    /// マスクデータを画像ファイル(BMP.JPEG.PNG)から構築する
    //pub fn dx_LoadMask() -> i32;
    /// マスクデータを画像ファイル(BMP.JPEG.PNG)から分割構築する
    //pub fn dx_LoadDivMask() -> i32;
    /// マスクデータをマスク画面に描画する
    //pub fn dx_DrawMask() -> i32;
    /// 指定のマスク画面領域を指定のマスクデータをタイル上に並べて埋める
    //pub fn dx_DrawFillMask() -> i32;
    /// マスクデータを削除
    //pub fn dx_DeleteMask() -> i32;
    /// マスクデータを初期化する
    //pub fn dx_InitMask() -> i32;
    /// マスク画面を指定の色で塗りつぶす
    //pub fn dx_FillMaskScreen() -> i32;
    /// マスク画面の有効の有無を変更
    //pub fn dx_SetUseMaskScreenFlag() -> i32;
    /// 空のマスクデータの作成
    //pub fn dx_MakeMask() -> i32;
    /// マスクデータの大きさを得る
    //pub fn dx_GetMaskSize() -> i32;
    /// マスクのデータをマスクデータ領域に転送する
    //pub fn dx_SetDataToMask() -> i32;
    /// マスクのデータをマスク画面に直接描画する
    //pub fn dx_DrawMaskToDirectData() -> i32;
    /// マスクのデータをタイル上に並べた形で直接マスク画面全体に描画する
    //pub fn dx_DrawFillMaskToDirectData() -> i32;

    // 入力関係の関数

    /// ジョイパッドが接続されている数を取得する
    pub fn dx_GetJoypadNum() -> i32;
    /// ジョイパッドの入力状態を得る
    pub fn dx_GetJoypadInputState(InputType: i32) -> i32;
    /// ジョイパッドのアナログ的なレバー入力情報を得る
    pub fn dx_GetJoypadAnalogInput(XBuf: *mut i32, YBuf: *mut i32, InputType: i32) -> i32;
    /// ジョイパッドのDirectInputから取得できる情報を得る
    //pub fn dx_GetJoypadDirectInputState() -> i32;
    /// ジョイパッドのXInputから取得できる情報を得る
    //pub fn dx_GetJoypadXInputState() -> i32;
    /// ジョイパッドの方向入力の無効範囲を設定する
    //pub fn dx_SetJoypadDeadZone() -> i32;
    /// ジョイパッドの振動を開始する
    pub fn dx_StartJoypadVibration(InputType: i32, Power: i32, Time: i32, EffectIndex: i32) -> i32;
    /// ジョイパッドの振動を停止する
    //pub fn dx_StopJoypadVibration() -> i32;

    /// マウスカーソルの表示設定フラグのセット
    //pub fn dx_SetMouseDispFlag() -> i32;
    /// マウスカーソルの位置を取得する
    pub fn dx_GetMousePoint(XBuf: *mut c_int, YBuf: *mut c_int) -> i32;
    /// マウスカーソルの位置をセットする
    //pub fn dx_SetMousePoint() -> i32;
    /// マウスのボタンの状態を得る
    //pub fn dx_GetMouseInput() -> i32;
    /// マウスのボタンが押されたり離されたりした履歴を取得する
    //pub fn dx_GetMouseInputLog2() -> i32;
    /// マウスホイールの回転量を得る
    //pub fn dx_GetMouseWheelRotVol() -> i32;

    /// タッチされている箇所の数を取得する
    //pub fn dx_GetTouchInputNum() -> i32;
    /// タッチされている箇所の情報を取得する
    //pub fn dx_GetTouchInput() -> i32;

    /// すべてのキーの押下状態を取得
    pub fn dx_CheckHitKeyAll(check_type: i32) -> i32;
    /// 特定キーの入力状態を得る
    pub fn dx_CheckHitKey(key_code: i32) -> i32;
    /// キーボードのすべてのキーの押下状態を取得する
    pub fn dx_GetHitKeyStateAll(key_state_buf: *mut c_char) -> i32;

    /// 文字入力バッファに溜まった文字データから１文字取得する
    //pub fn dx_GetInputChar() -> i32;
    /// 文字入力バッファに溜まった文字データから１文字取得する、バッファになにも文字コードがない場合はキーが押されるまで待つ
    //pub fn dx_GetInputCharWait() -> i32;
    /// 文字入力バッファをクリアする
    //pub fn dx_ClearInputCharBuf() -> i32;

    /// キーボードによる文字列の入力
    pub fn dx_KeyInputString(
        x: i32,
        y: i32,
        CharMaxLength: i32,
        StrBuffer: *mut c_char,
        CancelValidFlag: i32,
    ) -> i32;
    /// キーボードによる半角文字列のみの入力
    pub fn dx_KeyInputSingleCharString(
        x: i32,
        y: i32,
        CharMaxLength: i32,
        StrBuffer: *mut c_char,
        CancelValidFlag: i32,
    ) -> i32;
    /// キーボードによる数値の入力
    pub fn dx_KeyInputNumber(x: i32, y: i32, MaxNum: i32, MinNum: i32, CancelValidFlag: i32)
        -> i32;
    /// KeysnputString系 関数使用時の文字の各色を変更する
    pub fn dx_SetKeyInputStringColor(
        NmlStr: u32,
        NmlCur: u32,
        IMEStrBack: u32,
        IMECur: u32,
        IMELine: u32,
        IMESelectStr: u32,
        IMEModeStr: u32,
        NmlStrE: u32,
        IMESelectStrE: u32,
        IMEModeStrE: u32,
        IMESelectWinE: u32,
        IMESelectWinF: u32,
        SelectStrBackColor: u32,
        SelectStrColor: u32,
        SelectStrEdgeColor: u32,
        IMEStr: u32,
        IMEStrE: u32,
    ) -> i32;
    /// 新しいキー入力データの作成
    pub fn dx_MakeKeyInput(
        MaxStrLength: i32,
        CancelValidFlag: i32,
        SingleCharOnlyFlag: i32,
        NumCharOnlyFlag: i32,
    ) -> i32;
    /// キー入力データの削除
    pub fn dx_DeleteKeyInput(InputHandle: i32) -> i32;
    /// すべてのキー入力データの削除
    pub fn dx_InitKeyInput() -> i32;
    /// 指定のキー入力をアクティブにする
    pub fn dx_SetActiveKeyInput(InputHandle: i32) -> i32;
    /// 入力が終了しているか取得する
    pub fn dx_CheckKeyInput(InputHandle: i32) -> i32;
    /// キー入力中データの描画
    pub fn dx_DrawKeyInputString(x: i32, y: i32, InputHandle: i32) -> i32;
    /// 入力モード文字列を描画する
    pub fn dx_DrawKeyInputModeString(x: i32, y: i32) -> i32;
    /// キー入力データに指定の文字列をセットする
    //pub fn dx_SetKeyInputString() -> i32;
    /// キー入力データに指定の数値を文字に置き換えてセットする
    pub fn dx_SetKeyInputNumber(Number: i32, InputHandle: i32) -> i32;
    /// 入力データの文字列を取得する
    //pub fn dx_GetKeyInputString() -> i32;
    /// 入力データの文字列を数値として取得する
    pub fn dx_GetKeyInputNumber(InputHandle: i32) -> i32;

    // 音利用関数

    /// 音ファイルを再生する
    //pub fn dx_PlaySoundFile(FileName:&str,PlayType:i32) -> i32;
    /// 音ファイルが再生中か調べる
    //pub fn dx_CheckSoundFile() -> i32;
    /// 音ファイルの再生を止める
    //pub fn dx_StopSoundFile() -> i32;
    /// 音ファイルをメモリに読みこむ
    //pub fn dx_LoadSoundMem() -> i32;
    /// メモリに読みこんだ音データを再生する
    //pub fn dx_PlaySoundMem() -> i32;
    /// メモリに読みこんだ音データが再生中か調べる
    pub fn dx_CheckSoundMem(SoundHandle: i32) -> i32;
    /// メモリに読み込んだ音データの再生を止める
    pub fn dx_StopSoundMem(SoundHandle: i32) -> i32;
    /// メモリに読みこんだ音データを削除する
    pub fn dx_DeleteSoundMem(SoundHandle: i32) -> i32;
    /// メモリに読みこんだ音データをすべて消去する
    pub fn dx_InitSoundMem() -> i32;
    /// メモリに読みこんだ音データの再生にパンを設定する
    //pub fn dx_ChangePanSoundMem() -> i32;
    /// メモリに読みこんだ音データの再生にボリュームを設定する
    //pub fn dx_ChangeVolumeSoundMem() -> i32;
    /// メモリに読みこんだ音データの次の再生にのみ使用するパンを設定する
    //pub fn dx_ChangeNextPlayPanSoundMem() -> i32;
    /// メモリに読みこんだ音データの次の再生にのみ使用するボリュームを設定する
    //pub fn dx_ChangeNextPlayVolumeSoundMem() -> i32;
    /// メモリに読み込んだ音データの再生周波数を設定する
    //pub fn dx_SetFrequencySoundMem() -> i32;
    /// メモリに読み込んだ音データのループ位置を設定する
    //pub fn dx_SetLoopPosSoundMem() -> i32;
    /// メモリに読み込んだ音データのループ位置を設定する(サンプル位置指定)
    //pub fn dx_SetLoopSamplePosSoundMem() -> i32;
    /// メモリに読み込んだ音データの再生位置をサンプル単位で変更する
    //pub fn dx_SetCurrentPositionSoundMem() -> i32;
    /// 既にメモリに読み込んである音データを使用するサウンドハンドルを新たに作成する( 非ストリームサウンドのみ )
    //pub fn dx_DuplicateSoundMem() -> i32;
    /// 作成するメモリに読み込んだ音データのピッチ( 音の長さを変えずに音程を変更する )レートを設定する
    //pub fn dx_SetCreateSoundPitchRate() -> i32;
    /// 作成するメモリに読み込んだ音データのタイムストレッチ( 音程を変えずに音の長さを変更する )レートを設定する
    //pub fn dx_SetCreateSoundTimeStretchRate() -> i32;
    /// メモリに読み込んだ音データの３Ｄサウンド用の再生位置を設定する
    //pub fn dx_Set3DPositionSoundMem() -> i32;
    /// メモリに読み込んだ音データの３Ｄサウンド用の音が聞こえる距離を設定する
    //pub fn dx_Set3DRadiusSoundMem() -> i32;
    /// メモリに読み込んだ音データの３Ｄサウンド用の移動速度を設定する
    //pub fn dx_Set3DVelocitySoundMem() -> i32;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の再生位置を設定する
    //pub fn dx_SetNextPlay3DPositionSoundMem() -> i32;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の音が聞こえる距離を設定する
    //pub fn dx_SetNextPlay3DRadiusSoundMem() -> i32;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の移動速度を設定する
    //pub fn dx_SetNextPlay3DVelocitySoundMem() -> i32;
    /// メモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータを設定する
    //pub fn dx_Set3DReverbParamSoundMem() -> i32;
    /// メモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータをプリセットを使用して設定する
    //pub fn dx_Set3DPresetReverbParamSoundMem() -> i32;
    /// ３Ｄサウンド用のプリセットのリバーブエフェクトパラメータを取得する
    //pub fn dx_Get3DPresetReverbParamSoundMem() -> i32;
    /// 全てのメモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータを設定する
    //pub fn dx_Set3DReverbParamSoundMemAll() -> i32;
    /// 全てのメモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータをプリセットを使用して設定する
    //pub fn dx_Set3DPresetReverbParamSoundMemAll() -> i32;
    /// 次に作成するメモリに読み込む音データを３Ｄサウンド用にするかどうかを設定する
    //pub fn dx_SetCreate3DSoundFlag() -> i32;
    /// サウンドの再生にXAudio2を使用するかどうかを設定する
    //pub fn dx_SetEnableXAudioFlag() -> i32;
    /// ３Ｄ空間の１メートルに相当する距離を設定する
    //pub fn dx_Set3DSoundOneMetre() -> i32;
    /// ３Ｄサウンドのリスナーの位置とリスナーの前方位置を設定する
    //pub fn dx_Set3DSoundListenerPosAndFrontPos_UpVecY() -> i32;
    /// ３Ｄサウンドのリスナーの位置とリスナーの前方位置とリスナーの上方向を設定する
    //pub fn dx_Set3DSoundListenerPosAndFrontPosAndUpVec() -> i32;
    /// ３Ｄサウンドのリスナーの移動速度を設定する
    //pub fn dx_Set3DSoundListenerVelocity() -> i32;

    // 音楽再生関数

    /// ＭＩＤＩ又はＭＰ３ファイルを演奏(再生)する
    //pub fn dx_PlayMusic() -> i32;
    /// ＭＩＤＩ又はＭＰ３ファイルが演奏(再生)中かの情報を取得する
    //pub fn dx_CheckMusic() -> i32;
    /// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)停止
    //pub fn dx_StopMusic() -> i32;
    /// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)の音量を設定する
    //pub fn dx_SetVolumeMusic() -> i32;

    // ウエイト関係の関数

    /// 指定の時間だけ処理をとめる
    pub fn dx_WaitTimer(WaitTime: i32) -> i32;
    /// ディスプレイの垂直同期信号を指定回数待つ
    //pub fn dx_WaitVSync() -> i32;
    /// キーの入力待ち
    pub fn dx_WaitKey() -> i32;

    // 時間関係の関数

    pub fn dx_SetWaitVSyncFlag(Flag: i32) -> i32;
    /// ミリ秒単位の精度を持つカウンタの現在値を得る
    pub fn dx_GetNowCount() -> i32;
    /// GetNowCountの高精度バージョン
    pub fn dx_GetNowHiPerformanceCount() -> i64;
    /// 現在時刻を取得する
    //pub fn dx_GetDateTime() -> i32;

    // 乱数取得関数

    /// 乱数を取得する
    pub fn dx_GetRand(rand_max: i32) -> i32;
    /// 乱数の初期値を設定する
    pub fn dx_SRand(seed: i32) -> i32;

    // ウインドウモード関係

    /// ウインドウモード・フルスクリーンモードの変更を行う
    pub fn dx_ChangeWindowMode(flag: i32) -> i32;
    /// ウインドウのタイトルを変更する
    //pub fn dx_SetMainWindowText(WindowText: *const c_char) -> i32;

    /// ウインドウのアイコンを変更する
    pub fn dx_SetWindowIconID(ID: i32) -> i32;
    /// ウインドウモードの時にウインドウのサイズを自由に変更出来るようにするかどうかを設定する
    pub fn dx_SetWindowSizeChangeEnableFlag(Flag: i32) -> i32;
    /// ウインドウモードの時のウインドウの大きさと描画画面の大きさの比率を設定する
    //pub fn dx_SetWindowSizeExtendRate() -> i32;

    // 通信関係

    /// 他のマシンに接続する
    //pub fn dx_ConnectNetWork() -> i32;
    /// 接続を終了する
    //pub fn dx_CloseNetWork() -> i32;
    /// 接続を受け付けられる状態にする
    //pub fn dx_PreparationListenNetWork() -> i32;
    /// 接続を受け付けている状態を解除する
    //pub fn dx_StopListenNetWork() -> i32;
    /// データを送信する
    //pub fn dx_NetWorkSend() -> i32;
    /// 受信データ一時記憶バッファに溜まっているデータの量を得る
    //pub fn dx_GetNetWorkDataLength() -> i32;
    /// 未送信のデータの量を得る
    //pub fn dx_GetNetWorkSendDataLength() -> i32;
    /// 受信データ一時記憶バッファに溜まっているデータを取得する
    //pub fn dx_NetWorkRecv() -> i32;
    /// 受信したデータを読み込む、読み込んだデータはバッファから削除されない
    //pub fn dx_NetWorkRecvToPeek() -> i32;
    /// 新たに確立した接続を示すネットワークハンドルを得る
    //pub fn dx_GetNewAcceptNetWork() -> i32;
    /// 新たに破棄された接続を示すネットワークハンドルを得る
    //pub fn dx_GetLostNetWork() -> i32;
    /// 接続状態を取得する
    //pub fn dx_GetNetWorkAcceptState() -> i32;
    /// 接続先のＩＰアドレスを得る
    //pub fn dx_GetNetWorkIP() -> i32;
    /// ＵＤＰを使用して通信するためのソケットを作成する
    //pub fn dx_MakeUDPSocket() -> i32;
    /// ＵＤＰを使用して通信するためのソケットを削除する
    //pub fn dx_DeleteUDPSocket() -> i32;
    /// ＵＤＰを使用して他のマシンにデータを送信する
    //pub fn dx_NetWorkSendUDP() -> i32;
    /// ＵＤＰを使用して他のマシンからのデータを受信する
    //pub fn dx_NetWorkRecvUDP() -> i32;
    /// ＵＤＰを使用した他のマシンから受信データがあるかどうかを取得する
    //pub fn dx_CheckNetWorkRecvUDP() -> i32;

    // ファイル読み込み関係

    /// ファイルを開く
    //pub fn dx_FileRead_open(FilePath: *mut c_char, ASync: i32) -> i32;
    /// ファイルのサイズを得る
    //pub fn dx_FileRead_size() -> i32;
    /// ファイルを閉じる
    pub fn dx_FileRead_close(FileHandle: i32) -> i32;
    /// ファイルポインタの位置を得る
    //pub fn dx_FileRead_tell() -> i32;
    /// ファイルポインタの位置を変更する
    //pub fn dx_FileRead_seek() -> i32;
    /// ファイルからデータを読み込む
    //pub fn dx_FileRead_read(Buffer: *mut c_void, ReadSize: i32, FileHandle: i32) -> i32;
    /// ファイルの終端かどうかを調べる
    pub fn dx_FileRead_eof(FileHandle: i32) -> i32;
    /// ファイルから一行読み出す
    //pub fn dx_FileRead_gets(Buffer:*mut c_char,Num:i32,FileHandle:i32) -> i32;
    /// ファイルから一文字読み出す
    //pub fn dx_FileRead_getc() -> i32;
    /// ファイルから書式付きデータを読み出す
    //pub fn dx_FileRead_scanf() -> i32;

    // ドット単位で画像にアクセスしたい関係

    /// ＣＰＵで扱うイメージの読み込み
    //pub fn dx_LoadSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの読み込み( RGBA8 カラーに変換 )
    //pub fn dx_LoadARGB8ColorSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの読み込み( XGBA8 カラーに変換 )
    //pub fn dx_LoadXRGB8ColorSoftImage() -> i32;
    /// ＣＰＵで扱うイメージのメモリからの読み込み
    //pub fn dx_LoadSoftImageToMem() -> i32;
    /// ＣＰＵで扱うイメージのメモリからの読み込み( RGBA8 カラーに変換 )
    //pub fn dx_LoadARGB8ColorSoftImageToMem() -> i32;
    /// ＣＰＵで扱うイメージのメモリからの読み込み( XGBA8 カラーに変換 )
    //pub fn dx_LoadXRGB8ColorSoftImageToMem() -> i32;
    /// ＣＰＵで扱うイメージの作成( RGBA8 カラー )
    //pub fn dx_MakeARGB8ColorSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの作成( XRGB8 カラー )
    //pub fn dx_MakeXRGB8ColorSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの作成( パレット２５６色 カラー )
    //pub fn dx_MakePAL8ColorSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの解放
    //pub fn dx_DeleteSoftImage() -> i32;
    /// ＣＰＵで扱うイメージを全て解放
    //pub fn dx_InitSoftImage() -> i32;
    /// ＣＰＵで扱うイメージのサイズを取得する
    //pub fn dx_GetSoftImageSize() -> i32;
    /// ＣＰＵで扱うイメージを指定色で塗りつぶす(各色要素は０～２５５)
    //pub fn dx_FillSoftImage() -> i32;
    /// ＣＰＵで扱うイメージのパレットをセットする(各色要素は０～２５５)
    //pub fn dx_SetPaletteSoftImage() -> i32;
    /// ＣＰＵで扱うイメージのパレットを取得する(各色要素は０～２５５)
    //pub fn dx_GetPaletteSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの指定座標にドットを描画する(パレット画像用、有効値は０～２５５)
    //pub fn dx_DrawPixelPalCodeSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの指定座標の色コードを取得する(パレット画像用、戻り値は０～２５５)
    //pub fn dx_GetPixelPalCodeSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの指定座標にドットを描画する(各色要素は０～２５５)
    //pub fn dx_DrawPixelSoftImage() -> i32;
    /// ＣＰＵで扱うイメージの指定座標の色を取得する(各色要素は０～２５５)
    //pub fn dx_GetPixelSoftImage() -> i32;
    /// ＣＰＵで扱うイメージを別のイメージ上に転送する
    //pub fn dx_BltSoftImage() -> i32;
    /// ＣＰＵで扱うイメージを画面に描画する
    //pub fn dx_DrawSoftImage() -> i32;
    /// ＣＰＵで扱うイメージからグラフィックハンドルを作成する
    //pub fn dx_CreateGraphFromSoftImage() -> i32;
    /// ＣＰＵで扱うイメージから分割グラフィックハンドルを作成する
    //pub fn dx_CreateDivGraphFromSoftImage() -> i32;

    // 非同期読み込み関係

    /// 非同期読み込みを行うかどうかを設定する
    pub fn dx_SetUseASyncLoadFlag(Flag: i32) -> i32;
    /// ハンドルが非同期読み込み中かどうかを取得する
    pub fn dx_CheckHandleASyncLoad(Handle: i32) -> i32;
    /// 行っている非同期読み込みの数を取得する
    pub fn dx_GetASyncLoadNum() -> i32;

    // 文字関係関数

    /// 文字列の引数の文字コードを設定する
    pub fn dx_SetUseCharCodeFormat(char_code_format: i32) -> i32;
    /// 文字列の先頭の文字のバイト数を取得する
    //pub fn dx_GetCharBytes() -> i32;

    // マイナー関数

    /// ウインドウがアクティブではない状態でも処理を続行するか、フラグをセットする
    pub fn dx_SetAlwaysRunFlag(Flag: i32) -> i32;
    /// ログ出力を行うか否かのセット
    pub fn dx_SetOutApplicationLogValidFlag(Flag: i32) -> i32;
    /// ＤＸアーカイブファイルの読み込み機能を使うかどうかを設定する
    pub fn dx_SetUseDXArchiveFlag(Flag: i32) -> i32;
    /// 検索するＤＸアーカイブファイルの拡張子を変更する
    //pub fn dx_SetDXArchiveExtension(Extension: *const c_char) -> i32;
    /// ＤＸアーカイブファイルの鍵文字列を設定する
    //pub fn dx_SetDXArchiveKeyString() -> i32;
    /// ６４０ｘ４８０の画面で３２０ｘ２４０の画面解像度にするかどうかのフラグをセットする
    //pub fn dx_SetEmulation320x240() -> i32;
    /// ３Ｄ機能を使うか、のフラグをセット
    pub fn dx_SetUse3DFlag(Flag: i32) -> i32;
    /// ScreenFlip関数実行時にＣＲＴの垂直同期信号待ちをするかのフラグセット
    //pub fn dx_SetWaitVSyncFlag() -> i32;
    /// 必要ならグラフィックの分割を行うか否かを設定する
    //pub fn dx_SetUseDivGraphFlag() -> i32;
    /// フォーカスが他のソフトに移っているときにバックグラウンドに表示するグラフィックを設定する
    //pub fn dx_LoadPauseGraph() -> i32;
    /// 裏画面の内容を表画面にコピーする
    //pub fn dx_ScreenCopy() -> i32;
    /// 画面の色ビット数を得る
    //pub fn dx_GetColorBitDepth() -> i32;
    /// 現在描画対象になっている画面をＢＭＰ形式で保存する
    //pub fn dx_SaveDrawScreen(x1: i32, y1: i32, x2: i32, y2: i32, FileName: *const c_char) -> i32;
    /// 使用可能なフォントの名前を列挙する
    //pub fn dx_EnumFontName() -> i32;
    /// 文字列を縦に描画する
    //pub fn dx_DrawVString() -> i32;
    /// フォントハンドルを使用して文字列を縦に描画する
    //pub fn dx_DrawVStringToHandle() -> i32;
    /// メモリ上の画像ファイルイメージからグラフィックハンドルを作成する
    //pub fn dx_CreateGraphFromMem() -> i32;
    /// メモリ上の画像ファイルイメージから既存のグラフィックハンドルにデータを転送する
    //pub fn dx_ReCreateGraphFromMem() -> i32;
    /// 画像ファイルから作成したグラフィックハンドルに再度画像ファイルから画像を読み込む
    //pub fn dx_ReloadFileGraphAll() -> i32;
    /// グラフィックハンドル復元関数を登録する
    //pub fn dx_SetRestoreGraphCallback() -> i32;
    /// 作成する音データの再生形式を設定する
    //pub fn dx_SetCreateSoundDataType() -> i32;
    /// メモリ上の音ファイルイメージからサウンドハンドルを作成する
    //pub fn dx_LoadSoundMemByMemImage() -> i32;
    /// ＭＩＤＩの演奏形態をセットする
    pub fn dx_SelectMidiMode(mode: i32) -> i32;
    pub fn dx_SetKeyInputCursorBrinkFlag(Flag: i32) -> i32;
    pub fn dx_SetDragFileValidFlag(Flag: i32) -> i32;
    pub fn dx_GetDragFilePath(FilePathBuffer: *mut u16) -> i32;
    pub fn dx_GetDragFileNum() -> i32;
}

extern "cdecl" {
    // pub unsafe fn  dx_printfDx(fotmat:*const c_char, args: ...) -> i32;
}
/*wrapped function*/
mod hidden {
    use crate::dxlib_common::*;
    use std::os::raw::c_char;
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "stdcall" {
        pub fn dx_ClearDrawScreen(ClearRect: *mut RECT) -> i32;
        pub fn dx_LoadGraph(FileName: *const c_char, NotUse3DFlag: i32) -> i32;
        pub fn dx_PlaySoundFile(FileName: *const c_char, PlayType: i32) -> i32;
        pub fn dx_LoadSoundMem(FileName: *const c_char) -> i32;
        pub fn dx_PlaySoundMem(SoundHandle: i32, PlayType: i32) -> i32;
        pub fn dx_DrawString(x: i32, y: i32, String: *const c_char, Color: Color) -> i32;
        pub fn dx_MV1LoadModel(FileName: *const c_char) -> i32;
        pub fn dx_ChangeFont(FileName: *const c_char) -> i32;
        pub fn dx_CreateFontToHandle(
            FontName: *const c_char,
            Size: i32,
            Thick: i32,
            FontType: i32,
        ) -> i32;
        pub fn dx_DrawStringToHandle(
            x: i32,
            y: i32,
            String: *const c_char,
            Color: Color,
            FontHandle: i32,
        ) -> i32;
        pub fn dx_LoadFontDataToHandle(FileName: *const c_char, EdgeSize: i32) -> i32;
        pub fn dx_SetDXArchiveExtension(Extension: *const c_char) -> i32;
        pub fn dx_SaveDrawScreen(
            x1: i32,
            y1: i32,
            x2: i32,
            y2: i32,
            FileName: *const c_char,
        ) -> i32;
        pub fn dx_SetMainWindowText(WindowText: *const c_char) -> i32;

        pub fn dx_FileRead_gets(Buffer: *mut c_char, Num: i32, FileHandle: i32) -> i32;
        pub fn dx_FileRead_open(FilePath: *const c_char, ASync: i32) -> i32;
        pub fn dx_LoadDivGraph(
            FileName: *const c_char,
            AllNum: i32,
            XNum: i32,
            YNum: i32,
            XSize: i32,
            YSize: i32,
            HandleBuf: *mut i32,
        ) -> i32;

    }
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "cdecl" {
        //pub fn dx_DrawFormatString(x:i32,y:i32,ck\olor:\\\);
    }
}

/*wrap function*/
pub fn dx_ClearDrawScreen() -> i32 {
    let mut tmp = RECT {
        left: -1,
        right: -1,
        top: -1,
        bottom: -1,
    };
    unsafe { hidden::dx_ClearDrawScreen(&mut tmp) }
}

pub fn dx_LoadGraph(FileName: &str) -> i32 {
    unsafe {
        return hidden::dx_LoadGraph(FileName.to_cstring().as_ptr(), FALSE);
    }
}

pub fn dx_PlaySoundFile(FileName: &str, PlayType: i32) -> i32 {
    unsafe {
        return hidden::dx_PlaySoundFile(FileName.to_cstring().as_ptr(), PlayType);
    }
}

pub fn dx_LoadSoundMem(FileName: &str) -> i32 {
    unsafe {
        return hidden::dx_LoadSoundMem(FileName.to_cstring().as_ptr());
    }
}

pub fn dx_PlaySoundMem(SoundHandle: i32, PlayType: i32) -> i32 {
    unsafe {
        return hidden::dx_PlaySoundMem(SoundHandle, PlayType);
    }
}
pub fn dx_DrawString(x: i32, y: i32, String: &str, Color: Color) -> i32 {
    unsafe {
        return hidden::dx_DrawString(x, y, String.to_cstring().as_ptr(), Color);
    }
}
pub fn dx_MV1LoadModel(FileName: &str) -> i32 {
    unsafe {
        return hidden::dx_MV1LoadModel(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_ChangeFont(FileName: &str) -> i32 {
    unsafe {
        return hidden::dx_ChangeFont(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_CreateFontToHandle(FontName: &str, Size: i32, Thick: i32, FontType: i32) -> i32 {
    unsafe {
        return hidden::dx_CreateFontToHandle(
            FontName.to_cstring().as_ptr(),
            Size,
            Thick,
            FontType,
        );
    }
}
pub fn dx_DrawStringToHandle(x: i32, y: i32, String: &str, Color: Color, FontHandle: i32) -> i32 {
    unsafe {
        return hidden::dx_DrawStringToHandle(
            x,
            y,
            String.to_cstring().as_ptr(),
            Color,
            FontHandle,
        );
    }
}
pub fn dx_LoadFontDataToHandle(FileName: &str, EdgeSize: i32) -> i32 {
    unsafe {
        return hidden::dx_LoadFontDataToHandle(FileName.to_cstring().as_ptr(), EdgeSize);
    }
}
pub fn dx_SetDXArchiveExtension(Extension: &str) -> i32 {
    unsafe {
        return hidden::dx_SetDXArchiveExtension(Extension.to_cstring().as_ptr());
    }
}
pub fn dx_SaveDrawScreen(x1: i32, y1: i32, x2: i32, y2: i32, FileName: &str) -> i32 {
    unsafe {
        return hidden::dx_SaveDrawScreen(x1, y1, x2, y2, FileName.to_cstring().as_ptr());
    }
}
pub fn dx_SetMainWindowText(WindowText: &str) -> i32 {
    unsafe {
        return hidden::dx_SetMainWindowText(WindowText.to_cstring().as_ptr());
    }
}
/*pub fn dx_FileRead_gets(Buffer: &mut String, Num: i32, FileHandle: i32) -> i32 {*/
/*unsafe { */
/*let mut vec = Vec::with_capacity(Num as usize);*/
/*let mut c_str =CString::from_vec_unchecked(vec) ;*/
/*let mut c_str_ptr = c_str.as_ptr() as *mut c_char;*/
/*let result = hidden::dx_FileRead_gets(c_str_ptr,Num,FileHandle);*/
/*// ポインタからCStrを作成し、&strに変換する*/
/*let c_str = unsafe { CStr::from_ptr(c_str_ptr) };*/
/*let str_slice = c_str.to_str().unwrap();*/
/*let mut str_result = str_slice.to_string();*/
/**Buffer = str_result;*/
/*return result;*/
/*}*/
pub fn dx_FileRead_gets(Buffer: &mut String, Num: i32, FileHandle: i32) -> i32 {
    // ベクターからCStringを作成する際にヌル終端をチェックする
    let mut vec = Vec::with_capacity(Num as usize);
    //vec.resize(Num as usize); // ベクターを0で埋める
    //vec.push(0); // ヌル終端を追加する l
    let c_str = CString::new(vec).expect("CString::new failed");
    let mut c_str_ptr = c_str.into_raw();
    // 生のポインタに変換する
    let result = unsafe { hidden::dx_FileRead_gets(c_str_ptr, Num, FileHandle) };
    // 生のポインタからCStringに戻す l
    let c_str = unsafe { CString::from_raw(c_str_ptr) };
    // CStringから&strに変換する
    let str_slice = c_str.to_str().expect("CString::to_str failed");
    // &strからStringに変換する
    let str_result = str_slice.to_string();
    *Buffer = str_result;
    return result;
}

pub fn dx_FileRead_open(FilePath: &str, ASync: i32) -> i32 {
    unsafe {
        return hidden::dx_FileRead_open(FilePath.to_cstring().as_ptr(), ASync);
    }
}

pub fn dx_LoadDivGraph(
    FileName: &str,
    AllNum: i32,
    XNum: i32,
    YNum: i32,
    XSize: i32,
    YSize: i32,
    HandleBuf: *mut i32,
) -> i32 {
    unsafe {
        return hidden::dx_LoadDivGraph(
            FileName.to_cstring().as_ptr(),
            AllNum,
            XNum,
            YNum,
            XSize,
            YSize,
            HandleBuf,
        );
    }
}
