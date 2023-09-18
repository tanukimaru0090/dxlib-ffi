#![allow(non_snake_case)]
extern crate encoding_rs;
use self::encoding_rs::SHIFT_JIS;
pub use crate::dxlib_common::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::os::raw::*;
use std::vec::Vec;

//エンコーディング変換
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

/// dxlib function extern declaration (based on Ver3.24b)
/// see: https://dxlib.xsrv.jp/dxfunc.html
#[link(name = "DxLib_x64")]
#[no_mangle]
extern "stdcall" {
    // 使用必須関数

    /// ライブラリの初期化
    pub fn dx_DxLib_Init() -> c_int;
    /// ライブラリ使用の終了関数
    pub fn dx_DxLib_End() -> c_int;
    /// ウインドウズのメッセージを処理する
    pub fn dx_ProcessMessage() -> c_int;

    //算術演算関数
    pub fn dx_VGet(x: f32, y: f32, z: f32) -> VECTOR;

    // ３Ｄ関係関数
    pub fn dx_MV1AttachAnim(
        MHandle: c_int,
        AnimIndex: c_int,
        AnimSrcMHandle: c_int,
        NameCheck: c_int,
    ) -> c_int;

    pub fn dx_MV1GetAnimNum(MHandle: c_int) -> c_int;
    pub fn dx_MV1DetachAnim(MHandle: c_int, AttachIndex: c_int) -> c_int;
    pub fn dx_MV1SetAttachAnimTime(MHandle: c_int, AttachIndex: c_int, Time: f32) -> c_int;
    pub fn dx_MV1GetAttachAnimTotalTime(MHandle: c_int, AttachIndex: c_int) -> f32;
    pub fn dx_MV1GetAttachAnimTime(MHandle: c_int, AttachIndex: c_int) -> f32;
    pub fn dx_MV1SetAttachAnimBlendRate(MHandle: c_int, AttachIndex: c_int, Rate: f32) -> c_int;
    pub fn dx_SetCameraPositionAndTarget_UpVecY(Position: VECTOR, Target: VECTOR) -> c_int;
    pub fn dx_SetCameraNearFar(Near: f32, Far: f32) -> c_int;
    // Zバッファを使うかどうかのフラグ
    pub fn dx_SetUseZBuffer3D(Flag: c_int) -> c_int;
    // Zバッファへの書き込みするかどうかのフラグ
    pub fn dx_SetWriteZBuffer3D(Flag: c_int) -> c_int;

    // モデルの読み込み
    //pub fn dx_MV1LoadModel(FileName: *const c_char) -> c_int;
    // モデルの描画
    pub fn dx_MV1DrawModel(MHandle: c_int) -> c_int;
    pub fn dx_MV1DeleteModel(MHandle: c_int) -> c_int;
    //　モデルの位置の指定
    pub fn dx_MV1SetPosition(MHandle: c_int, Position: VECTOR) -> c_int;
    pub fn dx_MV1SetScale(MHandle: c_int, Scale: VECTOR) -> c_int;

    pub fn dx_DrawLine3D(Pos1: VECTOR, Pos2: VECTOR, Color: Color) -> c_int;
    pub fn dx_DrawTriangle3D(
        Pos1: VECTOR,
        Pos2: VECTOR,
        Pos3: VECTOR,
        Color: Color,
        FillFlag: c_int,
    ) -> c_int;

    pub fn dx_DrawSphere3D(
        CenterPos: VECTOR,
        r: f32,
        DivNum: c_int,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: c_int,
    ) -> c_int;

    pub fn dx_DrawCapsule3D(
        Pos1: VECTOR,
        Pos2: VECTOR,
        r: f32,
        DivNum: c_int,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: c_int,
    ) -> c_int;

    pub fn dx_DrawCone3D(
        TopPos: VECTOR,
        BottomPos: VECTOR,
        r: f32,
        DivNum: c_int,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: c_int,
    ) -> c_int;

    pub fn dx_DrawBillboard3D(
        Pos: VECTOR,
        cx: f32,
        cy: f32,
        Size: f32,
        Angle: f32,
        GrHandle: c_int,
        TransFlag: c_int,
    ) -> c_int;
    pub fn dx_DrawPolygon3D(
        Vertex: *mut VERTEX3D,
        PolygonNum: c_int,
        GrHandle: c_int,
        TransFlag: c_int,
    ) -> c_int;

    // 図形描画関数

    /// 線を描画
    pub fn dx_DrawLine(x1: c_int, y1: c_int, x2: c_int, y2: c_int, color: Color) -> c_int;
    /// 線を描画(アンチエイリアス効果付き)
    pub fn dx_DrawLineAA(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) -> c_int;
    /// 四角を描画
    pub fn dx_DrawBox(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        color: Color,
        fill_flag: c_int,
    ) -> c_int;
    /// 四角を描画(アンチエイリアス効果付き)
    pub fn dx_DrawBoxAA(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: Color,
        fill_flag: c_int,
    ) -> c_int;
    /// 円の描画
    pub fn dx_DrawCircle(x: c_int, y: c_int, r: c_int, color: Color, fill_flag: c_int) -> c_int;
    /// 円の描画(アンチエイリアス効果付き)
    pub fn dx_DrawCircleAA(
        x: f32,
        y: f32,
        r: f32,
        pos_num: c_int,
        color: Color,
        fill_flag: c_int,
    ) -> c_int;
    /// 楕円の描画
    pub fn dx_DrawOval(
        x: c_int,
        y: c_int,
        rx: c_int,
        ry: c_int,
        Color: Color,
        FillFlag: c_int,
    ) -> c_int;
    /// 楕円の描画(アンチエイリアス効果付き)
    pub fn dx_DrawOvalAA(
        x: f32,
        y: f32,
        rx: f32,
        ry: f32,
        posnum: c_int,
        Color: Color,
        FillFlag: c_int,
    ) -> c_int;
    /// 三角形の描画
    pub fn dx_DrawTriangle(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        x3: c_int,
        y3: c_int,
        Color: Color,
        FillFlag: c_int,
    ) -> c_int;
    /// 三角形の描画(アンチエイリアス効果付き)
    pub fn dx_DrawTriangleAA(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        Color: Color,
        FillFlag: c_int,
    ) -> c_int;
    /// 点を描画する
    pub fn dx_DrawPixel(x: c_int, y: c_int, Color: Color) -> c_int;
    /// 指定点の色を取得
    pub fn dx_GetPixel(x: c_int, y: c_int) -> c_int;

    // グラフィックデータ制御関数

    /// 画像ファイルを読みこんで画面に表示する
    //pub fn dx_LoadGraphScreen() -> c_int;
    /// 画像ファイルのメモリへの読みこみ、及び動画ファイルのロード
    //pub fn dx_LoadGraph() -> c_int;
    /// 画像ファイルのメモリへの分割読みこみ
    //pub fn dx_LoadDivGraph(FileName:*const c_char,AllNum:c_int,XNum:c_int,YNum:c_int,XSize:c_int,YSize:c_int,HandleBuf:*mut c_int) -> c_int;
    /// 空のグラフィックを作成する
    pub fn dx_MakeGraph(SizeX: c_int, SizeY: c_int) -> c_int;
    /// 描画対象にできるグラフィックを作成する
    pub fn dx_MakeScreen(SizeX: c_int, SizeY: c_int, UseAlphaChannel: c_int) -> c_int;
    /// 描画対象にできるグラフィックのマルチサンプリング設定を行う
    pub fn dx_SetCreateDrawValidGraphMultiSample(Samples: c_int, Quality: c_int) -> c_int;
    /// 作成するグラフィックのビット深度を設定する
    pub fn dx_SetCreateGraphColorBitDepth(BitDepth: c_int) -> c_int;
    /// 描画可能な浮動小数点型のグラフィックを作成するかどうかの設定を行う
    pub fn dx_SetDrawValidFloatTypeGraphCreateFlag(Flag: c_int) -> c_int;
    /// 作成する描画可能なグラフィックのチャンネル数の設定を行う
    pub fn dx_SetCreateDrawValidGraphChannelNum(ChannelNum: c_int) -> c_int;
    /// 読み込み時に画像を乗算済みα画像に変換するかを設定する
    pub fn dx_SetUsePremulAlphaConvertLoad(UseFlag: c_int) -> c_int;
    /// メモリに読みこんだグラフィックの描画
    pub fn dx_DrawGraph(x: c_int, y: c_int, GrHandle: c_int, TransFlag: c_int) -> c_int;
    // メモリに読み込んだグラフィックの描画　(float版)
    pub fn dx_DrawGraphF(x: f32, y: f32, GrHandle: c_int, TransFlag: c_int) -> c_int;
    /// メモリに読みこんだグラフィックのＬＲ反転描画
    pub fn dx_DrawTurnGraph(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        GrHandle: c_int,
        TransFlag: c_int,
    ) -> c_int;
    /// メモリに読みこんだグラフィックの拡大縮小描画
    pub fn dx_DrawExtendGraph(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        GrHandle: c_int,
        TransFlag: c_int,
    ) -> c_int;
    /// メモリに読みこんだグラフィックの回転描画
    pub fn dx_DrawRotaGraph(
        x: c_int,
        y: c_int,
        ex_rate: f64,
        angle: f64,
        gr_handle: c_int,
        trans_flag: c_int,
        turn_flag: c_int,
    ) -> c_int;
    /// メモリに読みこんだグラフィックの回転描画(回転中心指定あり)
    pub fn dx_DrawRotaGraph2(
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        ExtRate: f64,
        Angle: f64,
        GrHandle: c_int,
        TransFlag: c_int,
        TurnFlag: c_int,
    ) -> c_int;
    /// メモリに読みこんだグラフィックの回転描画(回転中心指定あり、縦横拡大率別指定)
    pub fn dx_DrawRotaGraph3(
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GrHandle: c_int,
        TransFlag: c_int,
        TurnFlag: c_int,
    ) -> c_int;
    /// メモリに読みこんだグラフィックの自由変形描画
    pub fn dx_DrawModiGraph(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        x3: c_int,
        y3: c_int,
        x4: c_int,
        y4: c_int,
        GrHandle: c_int,
        TransFlag: c_int,
    ) -> c_int;
    /// グラフィックの指定矩形部分のみを描画
    pub fn dx_DrawRectGraph(
        DestX: c_int,
        DestY: c_int,
        SrcX: c_int,
        SrcY: c_int,
        Width: c_int,
        Height: c_int,
        GraphHandle: c_int,
        TransFlag: c_int,
        TurnFlag: c_int,
    ) -> c_int;
    /// 指定のグラフィックの指定部分だけを抜き出して新たなグラフィックを作成する
    pub fn dx_DerivationGraph(
        SrcX: c_int,
        SrcY: c_int,
        Width: c_int,
        Height: c_int,
        SrcGraphHandle: c_int,
    ) -> c_int;
    /// 描画先に設定されているグラフィック領域から指定領域のグラフィックを読みこむ
    pub fn dx_GetDrawScreenGraph(
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        GrHandle: c_int,
    ) -> c_int;
    /// グラフィックのサイズを得る
    //pub fn dx_GetGraphSize() -> c_int;
    /// 読みこんだグラフィックデータをすべて削除する
    pub fn dx_InitGraph() -> c_int;
    /// 指定のグラフィックをメモリ上から削除する
    pub fn dx_DeleteGraph(GrHandle: c_int) -> c_int;
    /// 描画モードをセットする
    pub fn dx_SetDrawMode(DrawMode: c_int) -> c_int;
    /// 描画の際のブレンドモードをセットする
    pub fn dx_SetDrawBlendMode(blend_mode: c_int, pal: c_int) -> c_int;
    /// 描画輝度をセット
    pub fn dx_SetDrawBright(RedBright: c_int, GreenBright: c_int, BlueBright: c_int) -> c_int;
    /// グラフィックに設定する透過色をセットする
    pub fn dx_SetTransColor(Red: c_int, Green: c_int, Blue: c_int) -> c_int;
    // 画像ファイルからブレンド画像を読み込む
    //pub fn dx_LoadBlendGraph() -> c_int;
    /// ブレンド画像と通常画像を合成して描画する
    pub fn dx_DrawBlendGraph(
        x: c_int,
        y: c_int,
        GrHandle: c_int,
        TransFlag: c_int,
        BlendGraph: c_int,
        BorderParam: c_int,
        BorderRange: c_int,
    ) -> c_int;
    /// 画像にフィルター処理を施す
    //pub fn dx_GraphFilter() -> c_int;
    /// 画像にフィルター処理を施す( 出力先画像指定版 )
    //pub fn dx_GraphFilterBlt() -> c_int;
    /// 画像にフィルター処理を施す( 出力先画像、使用矩形指定版 )
    //pub fn dx_GraphFilterRectBlt() -> c_int;
    /// 二つの画像を特殊効果付きでブレンドする
    //pub fn dx_GraphBlend() -> c_int;
    /// 二つの画像を特殊効果付きでブレンドする( 出力先画像指定版 )
    //pub fn dx_GraphBlendBlt() -> c_int;
    /// 二つの画像を特殊効果付きでブレンドする( 出力先画像、使用矩形指定版 )
    //pub fn dx_GraphBlendRectBlt() -> c_int;

    // 文字描画関係関数

    /// 文字列を描画する
    //  pub fn dx_DrawString(x: c_int, y: c_int, string: *const c_char, color: Color) -> c_int;
    /// 書式付き文字列を描画する
    //pub fn dx_DrawFormatString(x:c_int,y:c_int,color:Color,format:*const c_char, ...) -> c_int;
    /// DrawString で描画される文字列の幅(ドット単位)を得る
    //pub fn dx_GetDrawStringWidth() -> c_int;
    /// DrawFormatString 関数書式付き文字列の描画幅(ドット単位)を得る
    //pub fn dx_GetDrawFormatStringWidth() -> c_int;
    /// 描画する文字列のサイズをセットする
    pub fn dx_SetFontSize(size: c_int) -> c_int;
    /// 描画する文字列の文字の太さをセットする
    pub fn dx_SetFontThickness(TinckPal: c_int) -> c_int;
    /// 描画するフォントを変更する
    //pub fn dx_ChangeFont(name: *const c_char) -> c_int;
    /// 文字列描画に使用するフォントのタイプを変更する
    pub fn dx_ChangeFontType(FontType: c_int) -> c_int;
    /// 新しいフォントデータを作成
    //pub fn dx_CreateFontToHandle(FontName: *const c_char,Size: c_int,Thick: c_int,FontType: c_int) -> c_int;
    /// ＤＸフォントデータファイルを読み込む
    // pub fn dx_LoadFontDataToHandle(FileName: *const c_char, EdgeSize: c_int) -> c_int;
    /// フォントデータを削除する
    pub fn dx_DeleteFontToHandle(FontHandle: c_int) -> c_int;
    /// 作成するフォントデータを『乗算済みα』用にするかどうかを設定する
    pub fn dx_SetFontCacheUsePremulAlphaFlag(Flag: c_int) -> c_int;
    /// 指定のフォントデータで文字列を描画する
    // pub fn dx_DrawStringToHandle(x: c_int,y: c_int,String: *const c_char,Color: Color,FontHandle: c_int,) -> c_int;
    /// 指定のフォントデータで書式付き文字列を描画する
    //pub fn dx_DrawFormatStringToHandle() -> c_int;
    /// 指定のフォントデータで描画する文字列の幅を得る
    //pub fn dx_GetDrawStringWidthToHandle(String:*const c_char,StrLen:c_int, FontHandle:c_int) -> c_int;
    /// 指定のフォントデータで書式付き文字列の描画幅を得る
    //pub fn dx_GetDrawFormatStringWidthToHandle() -> c_int;
    /// 指定のフォントデータの情報を得る
    //pub fn dx_GetFontStateToHandle(FontName:*const c_char,Size:*mut c_int,Thick:*mut c_int,FontHandle:c_int) -> c_int;
    /// フォントデータを全て初期化する
    pub fn dx_InitFontToHandle() -> c_int;

    // 簡易画面出力関数

    //pub fn dx_() -> c_int;
    // 簡易文字列描画

    //pub unsafe fn  dx_printfDx(arg: T, arg2: U, mut args: ...) -> c_int;
    /// 簡易画面出力履歴をクリアする
    //pub fn dx_clsDx() -> c_int;

    // その他画面操作系関数

    /// 画面モードの変更
    pub fn dx_SetGraphMode(
        SizeX: c_int,
        SizeY: c_int,
        ColorBitNum: c_int,
        RefreshRate: c_int,
    ) -> c_int;
    /// フルスクリーンモード時の解像度モードを設定する
    pub fn dx_SetFullScreenResolutionMode(ResolutionMode:c_int) -> c_int;
    /// フルスクリーンモード時の画面拡大モードを設定する
    pub fn dx_SetFullScreenScalingMode(ScalingMode:c_int) -> c_int;
    /// 現在の画面の大きさとカラービット数を得る
    //pub fn dx_GetScreenState(SizeX:*mut c_int,SizeY:*mut c_int,ColorBitDepth:*mut c_int) -> c_int;
    /// 描画可能領域のセット
    //pub fn dx_SetDrawArea() -> c_int;
    /// 画面に描かれたものを消去する
    //pub fn dx_ClearDrawScreen() -> c_int;
    /// 画面の背景色を設定する
    //pub fn dx_SetBackgroundColor() -> c_int;
    /// 色コードを取得する
    pub fn dx_GetColor(Red: c_int, Green: c_int, Blue: c_int) -> Color;
    pub fn dx_GetColorU8(Red: c_int, Green: c_int, Blue: c_int, Alpha: c_int) -> COLOR_U8;
    /// 描画先グラフィック領域の指定
    pub fn dx_SetDrawScreen(DrawScreen: c_int) -> c_int;
    /// フリップ関数、画面の裏ページ(普段は表示されていない)の内容を表ページ(普段表示されている)に反映する
    pub fn dx_ScreenFlip() -> c_int;
    /// 画面のフルスクリーンアンチエイリアスモードの設定をする
    //pub fn dx_SetFullSceneAntiAliasingMode() -> c_int;

    // 動画関係関数

    /// 動画ファイルの再生
    //pub fn dx_PlayMovie(FileName:*const c_char,ExRate:c_int,PlayType:c_int) -> c_int;
    /// ムービーグラフィックの動画の再生を開始する
    //pub fn dx_PlayMovieToGraph() -> c_int;
    /// ムービーグラフィックの動画再生を一時停止する
    //pub fn dx_PauseMovieToGraph() -> c_int;
    /// ムービーグラフィックの動画の再生位置を変更する
    //pub fn dx_SeekMovieToGraph() -> c_int;
    /// ムービーグラフィックの動画の再生位置を得る
    //pub fn dx_TellMovieToGraph() -> c_int;
    /// ムービーグラフィックの動画の再生状態を得る
    //pub fn dx_GetMovieStateToGraph() -> c_int;

    // マスク関係関数

    /// マスク画面を作成する
    //pub fn dx_CreateMaskScreen() -> c_int;
    /// マスク画面を削除する
    //pub fn dx_DeleteMaskScreen() -> c_int;
    /// マスクデータを画像ファイル(BMP.JPEG.PNG)から構築する
    //pub fn dx_LoadMask() -> c_int;
    /// マスクデータを画像ファイル(BMP.JPEG.PNG)から分割構築する
    //pub fn dx_LoadDivMask() -> c_int;
    /// マスクデータをマスク画面に描画する
    //pub fn dx_DrawMask() -> c_int;
    /// 指定のマスク画面領域を指定のマスクデータをタイル上に並べて埋める
    //pub fn dx_DrawFillMask() -> c_int;
    /// マスクデータを削除
    //pub fn dx_DeleteMask() -> c_int;
    /// マスクデータを初期化する
    //pub fn dx_InitMask() -> c_int;
    /// マスク画面を指定の色で塗りつぶす
    //pub fn dx_FillMaskScreen() -> c_int;
    /// マスク画面の有効の有無を変更
    //pub fn dx_SetUseMaskScreenFlag() -> c_int;
    /// 空のマスクデータの作成
    //pub fn dx_MakeMask() -> c_int;
    /// マスクデータの大きさを得る
    //pub fn dx_GetMaskSize() -> c_int;
    /// マスクのデータをマスクデータ領域に転送する
    //pub fn dx_SetDataToMask() -> c_int;
    /// マスクのデータをマスク画面に直接描画する
    //pub fn dx_DrawMaskToDirectData() -> c_int;
    /// マスクのデータをタイル上に並べた形で直接マスク画面全体に描画する
    //pub fn dx_DrawFillMaskToDirectData() -> c_int;

    // 入力関係の関数

    /// ジョイパッドが接続されている数を取得する
    pub fn dx_GetJoypadNum() -> c_int;
    /// ジョイパッドの入力状態を得る
    pub fn dx_GetJoypadInputState(InputType: c_int) -> c_int;
    /// ジョイパッドのアナログ的なレバー入力情報を得る
    pub fn dx_GetJoypadAnalogInput(XBuf: *mut c_int, YBuf: *mut c_int, InputType: c_int) -> c_int;
    /// ジョイパッドのDirectInputから取得できる情報を得る
    //pub fn dx_GetJoypadDirectInputState() -> c_int;
    /// ジョイパッドのXInputから取得できる情報を得る
    //pub fn dx_GetJoypadXInputState() -> c_int;
    /// ジョイパッドの方向入力の無効範囲を設定する
    //pub fn dx_SetJoypadDeadZone() -> c_int;
    /// ジョイパッドの振動を開始する
    pub fn dx_StartJoypadVibration(
        InputType: c_int,
        Power: c_int,
        Time: c_int,
        EffectIndex: c_int,
    ) -> c_int;
    /// ジョイパッドの振動を停止する
    //pub fn dx_StopJoypadVibration() -> c_int;

    /// マウスカーソルの表示設定フラグのセット
    pub fn dx_SetMouseDispFlag(DispFlag: c_int) -> c_int;
    /// マウスカーソルの位置を取得する
    pub fn dx_GetMousePoint(XBuf: *mut c_int, YBuf: *mut c_int) -> c_int;
    /// マウスカーソルの位置をセットする
    //pub fn dx_SetMousePoint() -> c_int;
    /// マウスのボタンの状態を得る
    //pub fn dx_GetMouseInput() -> c_int;
    /// マウスのボタンが押されたり離されたりした履歴を取得する
    //pub fn dx_GetMouseInputLog2() -> c_int;
    /// マウスホイールの回転量を得る
    //pub fn dx_GetMouseWheelRotVol() -> c_int;

    /// タッチされている箇所の数を取得する
    //pub fn dx_GetTouchInputNum() -> c_int;
    /// タッチされている箇所の情報を取得する
    //pub fn dx_GetTouchInput() -> c_int;

    /// すべてのキーの押下状態を取得
    pub fn dx_CheckHitKeyAll(check_type: c_int) -> c_int;
    /// 特定キーの入力状態を得る
    pub fn dx_CheckHitKey(key_code: c_int) -> c_int;
    /// キーボードのすべてのキーの押下状態を取得する
    pub fn dx_GetHitKeyStateAll(key_state_buf: *mut c_char) -> c_int;

    /// 文字入力バッファに溜まった文字データから１文字取得する
    //pub fn dx_GetInputChar() -> c_int;
    /// 文字入力バッファに溜まった文字データから１文字取得する、バッファになにも文字コードがない場合はキーが押されるまで待つ
    //pub fn dx_GetInputCharWait() -> c_int;
    /// 文字入力バッファをクリアする
    //pub fn dx_ClearInputCharBuf() -> c_int;

    /// キーボードによる文字列の入力
    pub fn dx_KeyInputString(
        x: c_int,
        y: c_int,
        CharMaxLength: c_int,
        StrBuffer: *mut c_char,
        CancelValidFlag: c_int,
    ) -> c_int;
    /// キーボードによる半角文字列のみの入力
    pub fn dx_KeyInputSingleCharString(
        x: c_int,
        y: c_int,
        CharMaxLength: c_int,
        StrBuffer: *mut c_char,
        CancelValidFlag: c_int,
    ) -> c_int;
    /// キーボードによる数値の入力
    pub fn dx_KeyInputNumber(
        x: c_int,
        y: c_int,
        MaxNum: c_int,
        MinNum: c_int,
        CancelValidFlag: c_int,
    ) -> c_int;
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
    ) -> c_int;
    /// 新しいキー入力データの作成
    pub fn dx_MakeKeyInput(
        MaxStrLength: c_int,
        CancelValidFlag: c_int,
        SingleCharOnlyFlag: c_int,
        NumCharOnlyFlag: c_int,
    ) -> c_int;
    /// キー入力データの削除
    pub fn dx_DeleteKeyInput(InputHandle: c_int) -> c_int;
    /// すべてのキー入力データの削除
    pub fn dx_InitKeyInput() -> c_int;
    /// 指定のキー入力をアクティブにする
    pub fn dx_SetActiveKeyInput(InputHandle: c_int) -> c_int;
    /// 入力が終了しているか取得する
    pub fn dx_CheckKeyInput(InputHandle: c_int) -> c_int;
    /// キー入力中データの描画
    pub fn dx_DrawKeyInputString(x: c_int, y: c_int, InputHandle: c_int) -> c_int;
    /// 入力モード文字列を描画する
    pub fn dx_DrawKeyInputModeString(x: c_int, y: c_int) -> c_int;
    /// キー入力データに指定の文字列をセットする
    //pub fn dx_SetKeyInputString() -> c_int;
    /// キー入力データに指定の数値を文字に置き換えてセットする
    pub fn dx_SetKeyInputNumber(Number: c_int, InputHandle: c_int) -> c_int;
    /// 入力データの文字列を取得する
    //pub fn dx_GetKeyInputString() -> c_int;
    /// 入力データの文字列を数値として取得する
    pub fn dx_GetKeyInputNumber(InputHandle: c_int) -> c_int;

    // 音利用関数

    /// 音ファイルを再生する
    //pub fn dx_PlaySoundFile(FileName:&str,PlayType:c_int) -> c_int;
    /// 音ファイルが再生中か調べる
    //pub fn dx_CheckSoundFile() -> c_int;
    /// 音ファイルの再生を止める
    //pub fn dx_StopSoundFile() -> c_int;
    /// 音ファイルをメモリに読みこむ
    //pub fn dx_LoadSoundMem() -> c_int;
    /// メモリに読みこんだ音データを再生する
    //pub fn dx_PlaySoundMem() -> c_int;
    /// メモリに読みこんだ音データが再生中か調べる
    pub fn dx_CheckSoundMem(SoundHandle: c_int) -> c_int;
    /// メモリに読み込んだ音データの再生を止める
    pub fn dx_StopSoundMem(SoundHandle: c_int) -> c_int;
    /// メモリに読みこんだ音データを削除する
    pub fn dx_DeleteSoundMem(SoundHandle: c_int) -> c_int;
    /// メモリに読みこんだ音データをすべて消去する
    pub fn dx_InitSoundMem() -> c_int;
    /// メモリに読みこんだ音データの再生にパンを設定する
    //pub fn dx_ChangePanSoundMem() -> c_int;
    /// メモリに読みこんだ音データの再生にボリュームを設定する
    //pub fn dx_ChangeVolumeSoundMem() -> c_int;
    /// メモリに読みこんだ音データの次の再生にのみ使用するパンを設定する
    //pub fn dx_ChangeNextPlayPanSoundMem() -> c_int;
    /// メモリに読みこんだ音データの次の再生にのみ使用するボリュームを設定する
    //pub fn dx_ChangeNextPlayVolumeSoundMem() -> c_int;
    /// メモリに読み込んだ音データの再生周波数を設定する
    //pub fn dx_SetFrequencySoundMem() -> c_int;
    /// メモリに読み込んだ音データのループ位置を設定する
    //pub fn dx_SetLoopPosSoundMem() -> c_int;
    /// メモリに読み込んだ音データのループ位置を設定する(サンプル位置指定)
    //pub fn dx_SetLoopSamplePosSoundMem() -> c_int;
    /// メモリに読み込んだ音データの再生位置をサンプル単位で変更する
    //pub fn dx_SetCurrentPositionSoundMem() -> c_int;
    /// 既にメモリに読み込んである音データを使用するサウンドハンドルを新たに作成する( 非ストリームサウンドのみ )
    //pub fn dx_DuplicateSoundMem() -> c_int;
    /// 作成するメモリに読み込んだ音データのピッチ( 音の長さを変えずに音程を変更する )レートを設定する
    //pub fn dx_SetCreateSoundPitchRate() -> c_int;
    /// 作成するメモリに読み込んだ音データのタイムストレッチ( 音程を変えずに音の長さを変更する )レートを設定する
    //pub fn dx_SetCreateSoundTimeStretchRate() -> c_int;
    /// メモリに読み込んだ音データの３Ｄサウンド用の再生位置を設定する
    //pub fn dx_Set3DPositionSoundMem() -> c_int;
    /// メモリに読み込んだ音データの３Ｄサウンド用の音が聞こえる距離を設定する
    //pub fn dx_Set3DRadiusSoundMem() -> c_int;
    /// メモリに読み込んだ音データの３Ｄサウンド用の移動速度を設定する
    //pub fn dx_Set3DVelocitySoundMem() -> c_int;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の再生位置を設定する
    //pub fn dx_SetNextPlay3DPositionSoundMem() -> c_int;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の音が聞こえる距離を設定する
    //pub fn dx_SetNextPlay3DRadiusSoundMem() -> c_int;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の移動速度を設定する
    //pub fn dx_SetNextPlay3DVelocitySoundMem() -> c_int;
    /// メモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータを設定する
    //pub fn dx_Set3DReverbParamSoundMem() -> c_int;
    /// メモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータをプリセットを使用して設定する
    //pub fn dx_Set3DPresetReverbParamSoundMem() -> c_int;
    /// ３Ｄサウンド用のプリセットのリバーブエフェクトパラメータを取得する
    //pub fn dx_Get3DPresetReverbParamSoundMem() -> c_int;
    /// 全てのメモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータを設定する
    //pub fn dx_Set3DReverbParamSoundMemAll() -> c_int;
    /// 全てのメモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータをプリセットを使用して設定する
    //pub fn dx_Set3DPresetReverbParamSoundMemAll() -> c_int;
    /// 次に作成するメモリに読み込む音データを３Ｄサウンド用にするかどうかを設定する
    //pub fn dx_SetCreate3DSoundFlag() -> c_int;
    /// サウンドの再生にXAudio2を使用するかどうかを設定する
    //pub fn dx_SetEnableXAudioFlag() -> c_int;
    /// ３Ｄ空間の１メートルに相当する距離を設定する
    //pub fn dx_Set3DSoundOneMetre() -> c_int;
    /// ３Ｄサウンドのリスナーの位置とリスナーの前方位置を設定する
    //pub fn dx_Set3DSoundListenerPosAndFrontPos_UpVecY() -> c_int;
    /// ３Ｄサウンドのリスナーの位置とリスナーの前方位置とリスナーの上方向を設定する
    //pub fn dx_Set3DSoundListenerPosAndFrontPosAndUpVec() -> c_int;
    /// ３Ｄサウンドのリスナーの移動速度を設定する
    //pub fn dx_Set3DSoundListenerVelocity() -> c_int;

    // 音楽再生関数

    /// ＭＩＤＩ又はＭＰ３ファイルを演奏(再生)する
    //pub fn dx_PlayMusic() -> c_int;
    /// ＭＩＤＩ又はＭＰ３ファイルが演奏(再生)中かの情報を取得する
    //pub fn dx_CheckMusic() -> c_int;
    /// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)停止
    //pub fn dx_StopMusic() -> c_int;
    /// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)の音量を設定する
    //pub fn dx_SetVolumeMusic() -> c_int;

    // ウエイト関係の関数

    /// 指定の時間だけ処理をとめる
    pub fn dx_WaitTimer(WaitTime: c_int) -> c_int;
    /// ディスプレイの垂直同期信号を指定回数待つ
    //pub fn dx_WaitVSync() -> c_int;
    /// キーの入力待ち
    pub fn dx_WaitKey() -> c_int;

    // 時間関係の関数

    pub fn dx_SetWaitVSyncFlag(Flag: c_int) -> c_int;
    /// ミリ秒単位の精度を持つカウンタの現在値を得る
    pub fn dx_GetNowCount() -> c_int;
    /// GetNowCountの高精度バージョン
    pub fn dx_GetNowHiPerformanceCount() -> i64;
    /// 現在時刻を取得する
    //pub fn dx_GetDateTime() -> c_int;

    // 乱数取得関数

    /// 乱数を取得する
    pub fn dx_GetRand(rand_max: c_int) -> c_int;
    /// 乱数の初期値を設定する
    pub fn dx_SRand(seed: c_int) -> c_int;

    // ウインドウモード関係

    /// ウインドウモード・フルスクリーンモードの変更を行う
    pub fn dx_ChangeWindowMode(flag: c_int) -> c_int;
    /// ウインドウのタイトルを変更する
    //pub fn dx_SetMainWindowText(WindowText: *const c_char) -> c_int;

    /// ウインドウのアイコンを変更する
    pub fn dx_SetWindowIconID(ID: c_int) -> c_int;
    /// ウインドウモードの時にウインドウのサイズを自由に変更出来るようにするかどうかを設定する
    pub fn dx_SetWindowSizeChangeEnableFlag(Flag: c_int) -> c_int;
    /// ウインドウモードの時のウインドウの大きさと描画画面の大きさの比率を設定する
    pub fn dx_SetWindowSizeExtendRate(ExRateX: f64, ExRateY: f64) -> c_int;
    // 通信関係

    /// 他のマシンに接続する
    //pub fn dx_ConnectNetWork() -> c_int;
    /// 接続を終了する
    //pub fn dx_CloseNetWork() -> c_int;
    /// 接続を受け付けられる状態にする
    //pub fn dx_PreparationListenNetWork() -> c_int;
    /// 接続を受け付けている状態を解除する
    //pub fn dx_StopListenNetWork() -> c_int;
    /// データを送信する
    //pub fn dx_NetWorkSend() -> c_int;
    /// 受信データ一時記憶バッファに溜まっているデータの量を得る
    //pub fn dx_GetNetWorkDataLength() -> c_int;
    /// 未送信のデータの量を得る
    //pub fn dx_GetNetWorkSendDataLength() -> c_int;
    /// 受信データ一時記憶バッファに溜まっているデータを取得する
    //pub fn dx_NetWorkRecv() -> c_int;
    /// 受信したデータを読み込む、読み込んだデータはバッファから削除されない
    //pub fn dx_NetWorkRecvToPeek() -> c_int;
    /// 新たに確立した接続を示すネットワークハンドルを得る
    //pub fn dx_GetNewAcceptNetWork() -> c_int;
    /// 新たに破棄された接続を示すネットワークハンドルを得る
    //pub fn dx_GetLostNetWork() -> c_int;
    /// 接続状態を取得する
    //pub fn dx_GetNetWorkAcceptState() -> c_int;
    /// 接続先のＩＰアドレスを得る
    //pub fn dx_GetNetWorkIP() -> c_int;
    /// ＵＤＰを使用して通信するためのソケットを作成する
    //pub fn dx_MakeUDPSocket() -> c_int;
    /// ＵＤＰを使用して通信するためのソケットを削除する
    //pub fn dx_DeleteUDPSocket() -> c_int;
    /// ＵＤＰを使用して他のマシンにデータを送信する
    //pub fn dx_NetWorkSendUDP() -> c_int;
    /// ＵＤＰを使用して他のマシンからのデータを受信する
    //pub fn dx_NetWorkRecvUDP() -> c_int;
    /// ＵＤＰを使用した他のマシンから受信データがあるかどうかを取得する
    //pub fn dx_CheckNetWorkRecvUDP() -> c_int;

    // ファイル読み込み関係

    /// ファイルを開く
    //pub fn dx_FileRead_open(FilePath: *mut c_char, ASync: c_int) -> c_int;
    /// ファイルのサイズを得る
    //pub fn dx_FileRead_size() -> c_int;
    /// ファイルを閉じる
    pub fn dx_FileRead_close(FileHandle: c_int) -> c_int;
    /// ファイルポインタの位置を得る
    //pub fn dx_FileRead_tell() -> c_int;
    /// ファイルポインタの位置を変更する
    //pub fn dx_FileRead_seek() -> c_int;
    /// ファイルからデータを読み込む
    //pub fn dx_FileRead_read(Buffer: *mut c_void, ReadSize: c_int, FileHandle: c_int) -> c_int;
    /// ファイルの終端かどうかを調べる
    pub fn dx_FileRead_eof(FileHandle: c_int) -> c_int;
    /// ファイルから一行読み出す
    //pub fn dx_FileRead_gets(Buffer:*mut c_char,Num:c_int,FileHandle:c_int) -> c_int;
    /// ファイルから一文字読み出す
    //pub fn dx_FileRead_getc() -> c_int;
    /// ファイルから書式付きデータを読み出す
    //pub fn dx_FileRead_scanf() -> c_int;

    // ドット単位で画像にアクセスしたい関係

    /// ＣＰＵで扱うイメージの読み込み
    //pub fn dx_LoadSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの読み込み( RGBA8 カラーに変換 )
    //pub fn dx_LoadARGB8ColorSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの読み込み( XGBA8 カラーに変換 )
    //pub fn dx_LoadXRGB8ColorSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージのメモリからの読み込み
    //pub fn dx_LoadSoftImageToMem() -> c_int;
    /// ＣＰＵで扱うイメージのメモリからの読み込み( RGBA8 カラーに変換 )
    //pub fn dx_LoadARGB8ColorSoftImageToMem() -> c_int;
    /// ＣＰＵで扱うイメージのメモリからの読み込み( XGBA8 カラーに変換 )
    //pub fn dx_LoadXRGB8ColorSoftImageToMem() -> c_int;
    /// ＣＰＵで扱うイメージの作成( RGBA8 カラー )
    //pub fn dx_MakeARGB8ColorSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの作成( XRGB8 カラー )
    //pub fn dx_MakeXRGB8ColorSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの作成( パレット２５６色 カラー )
    //pub fn dx_MakePAL8ColorSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの解放
    //pub fn dx_DeleteSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージを全て解放
    //pub fn dx_InitSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージのサイズを取得する
    //pub fn dx_GetSoftImageSize() -> c_int;
    /// ＣＰＵで扱うイメージを指定色で塗りつぶす(各色要素は０～２５５)
    //pub fn dx_FillSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージのパレットをセットする(各色要素は０～２５５)
    //pub fn dx_SetPaletteSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージのパレットを取得する(各色要素は０～２５５)
    //pub fn dx_GetPaletteSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの指定座標にドットを描画する(パレット画像用、有効値は０～２５５)
    //pub fn dx_DrawPixelPalCodeSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの指定座標の色コードを取得する(パレット画像用、戻り値は０～２５５)
    //pub fn dx_GetPixelPalCodeSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの指定座標にドットを描画する(各色要素は０～２５５)
    //pub fn dx_DrawPixelSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージの指定座標の色を取得する(各色要素は０～２５５)
    //pub fn dx_GetPixelSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージを別のイメージ上に転送する
    //pub fn dx_BltSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージを画面に描画する
    //pub fn dx_DrawSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージからグラフィックハンドルを作成する
    //pub fn dx_CreateGraphFromSoftImage() -> c_int;
    /// ＣＰＵで扱うイメージから分割グラフィックハンドルを作成する
    //pub fn dx_CreateDivGraphFromSoftImage() -> c_int;

    // 非同期読み込み関係

    /// 非同期読み込みを行うかどうかを設定する
    pub fn dx_SetUseASyncLoadFlag(Flag: c_int) -> c_int;
    /// ハンドルが非同期読み込み中かどうかを取得する
    pub fn dx_CheckHandleASyncLoad(Handle: c_int) -> c_int;
    /// 行っている非同期読み込みの数を取得する
    pub fn dx_GetASyncLoadNum() -> c_int;

    // 文字関係関数

    /// 文字列の引数の文字コードを設定する
    pub fn dx_SetUseCharCodeFormat(char_code_format: c_int) -> c_int;
    /// 文字列の先頭の文字のバイト数を取得する
    //pub fn dx_GetCharBytes() -> c_int;

    // マイナー関数

    /// ウインドウがアクティブではない状態でも処理を続行するか、フラグをセットする
    pub fn dx_SetAlwaysRunFlag(Flag: c_int) -> c_int;
    /// ログ出力を行うか否かのセット
    pub fn dx_SetOutApplicationLogValidFlag(Flag: c_int) -> c_int;
    /// ＤＸアーカイブファイルの読み込み機能を使うかどうかを設定する
    pub fn dx_SetUseDXArchiveFlag(Flag: c_int) -> c_int;
    /// 検索するＤＸアーカイブファイルの拡張子を変更する
    //pub fn dx_SetDXArchiveExtension(Extension: *const c_char) -> c_int;
    /// ＤＸアーカイブファイルの鍵文字列を設定する
    //pub fn dx_SetDXArchiveKeyString() -> c_int;
    /// ６４０ｘ４８０の画面で３２０ｘ２４０の画面解像度にするかどうかのフラグをセットする
    //pub fn dx_SetEmulation320x240() -> c_int;
    /// ３Ｄ機能を使うか、のフラグをセット
    pub fn dx_SetUse3DFlag(Flag: c_int) -> c_int;
    /// ScreenFlip関数実行時にＣＲＴの垂直同期信号待ちをするかのフラグセット
    //pub fn dx_SetWaitVSyncFlag() -> c_int;
    /// 必要ならグラフィックの分割を行うか否かを設定する
    //pub fn dx_SetUseDivGraphFlag() -> c_int;
    /// フォーカスが他のソフトに移っているときにバックグラウンドに表示するグラフィックを設定する
    //pub fn dx_LoadPauseGraph() -> c_int;
    /// 裏画面の内容を表画面にコピーする
    //pub fn dx_ScreenCopy() -> c_int;
    /// 画面の色ビット数を得る
    //pub fn dx_GetColorBitDepth() -> c_int;
    /// 現在描画対象になっている画面をＢＭＰ形式で保存する
    //pub fn dx_SaveDrawScreen(x1: c_int, y1: c_int, x2: c_int, y2: c_int, FileName: *const c_char) -> c_int;
    /// 使用可能なフォントの名前を列挙する
    //pub fn dx_EnumFontName() -> c_int;
    /// 文字列を縦に描画する
    //pub fn dx_DrawVString() -> c_int;
    /// フォントハンドルを使用して文字列を縦に描画する
    //pub fn dx_DrawVStringToHandle() -> c_int;
    /// メモリ上の画像ファイルイメージからグラフィックハンドルを作成する
    //pub fn dx_CreateGraphFromMem() -> c_int;
    /// メモリ上の画像ファイルイメージから既存のグラフィックハンドルにデータを転送する
    //pub fn dx_ReCreateGraphFromMem() -> c_int;
    /// 画像ファイルから作成したグラフィックハンドルに再度画像ファイルから画像を読み込む
    //pub fn dx_ReloadFileGraphAll() -> c_int;
    /// グラフィックハンドル復元関数を登録する
    //pub fn dx_SetRestoreGraphCallback() -> c_int;
    /// 作成する音データの再生形式を設定する
    //pub fn dx_SetCreateSoundDataType() -> c_int;
    /// メモリ上の音ファイルイメージからサウンドハンドルを作成する
    //pub fn dx_LoadSoundMemByMemImage() -> c_int;
    /// ＭＩＤＩの演奏形態をセットする
    pub fn dx_SelectMidiMode(mode: c_int) -> c_int;
    pub fn dx_SetKeyInputCursorBrinkFlag(Flag: c_int) -> c_int;
    pub fn dx_SetDragFileValidFlag(Flag: c_int) -> c_int;
    pub fn dx_GetDragFilePath(FilePathBuffer: *mut u16) -> c_int;
    pub fn dx_GetDragFileNum() -> c_int;
}

extern "cdecl" {
    // pub unsafe fn  dx_printfDx(fotmat:*const c_char, args: ...) -> c_int;
}
/*wrapped function*/
mod hidden {
    use crate::dxlib_common::*;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::os::raw::*;
    use std::vec::Vec;
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "stdcall" {
        pub fn dx_ClearDrawScreen(ClearRect: *mut RECT) -> c_int;
        pub fn dx_LoadGraph(FileName: *const c_char, NotUse3DFlag: c_int) -> c_int;
        pub fn dx_PlaySoundFile(FileName: *const c_char, PlayType: c_int) -> c_int;
        pub fn dx_LoadSoundMem(FileName: *const c_char) -> c_int;
        pub fn dx_PlaySoundMem(SoundHandle: c_int, PlayType: c_int) -> c_int;
        pub fn dx_DrawString(x: c_int, y: c_int, String: *const c_char, Color: Color) -> c_int;
        pub fn dx_MV1LoadModel(FileName: *const c_char) -> c_int;
        pub fn dx_ChangeFont(FileName: *const c_char) -> c_int;
        pub fn dx_CreateFontToHandle(
            FontName: *const c_char,
            Size: c_int,
            Thick: c_int,
            FontType: c_int,
        ) -> c_int;
        pub fn dx_DrawStringToHandle(
            x: c_int,
            y: c_int,
            String: *const c_char,
            Color: Color,
            FontHandle: c_int,
        ) -> c_int;
        pub fn dx_LoadFontDataToHandle(FileName: *const c_char, EdgeSize: c_int) -> c_int;
        pub fn dx_SetDXArchiveExtension(Extension: *const c_char) -> c_int;
        pub fn dx_SaveDrawScreen(
            x1: c_int,
            y1: c_int,
            x2: c_int,
            y2: c_int,
            FileName: *const c_char,
        ) -> c_int;
        pub fn dx_SetMainWindowText(WindowText: *const c_char) -> c_int;

        pub fn dx_FileRead_gets(Buffer: *mut c_char, Num: c_int, FileHandle: c_int) -> c_int;
        pub fn dx_FileRead_open(FilePath: *const c_char, ASync: c_int) -> c_int;
        pub fn dx_LoadDivGraph(
            FileName: *const c_char,
            AllNum: c_int,
            XNum: c_int,
            YNum: c_int,
            XSize: c_int,
            YSize: c_int,
            HandleBuf: *mut c_int,
        ) -> c_int;
        pub fn dx_LoadGraphScreen(
            x: c_int,
            y: c_int,
            GraphName: *const c_char,
            TransFlag: c_int,
        ) -> c_int;
        pub fn dx_GetGraphSize(
            GrHandle: c_int,
            SizeXBuf: *mut c_int,
            SizeYBuf: *mut c_int,
        ) -> c_int;
        pub fn dx_LoadBlendGraph(FileName: *const c_char) -> c_int;
        pub fn dx_GetDateTime(DateBuf: *mut DATEDATA) -> c_int;
        pub fn dx_GetDrawStringWidth(String: *const c_char, StrLen: c_int) -> c_int;
        pub fn dx_GetDrawStringWidthToHandle(
            String: *const c_char,
            StrLen: c_int,
            FontHandle: c_int,
        ) -> c_int;

        pub fn dx_GetFontStateToHandle(
            FontName: *const c_char,
            Size: *mut c_int,
            Thick: *mut c_int,
            FontHandle: c_int,
        ) -> c_int;

        pub fn dx_GetScreenState(SizeX:*mut c_int,SizeY:*mut c_int,ColorBitDepth:*mut c_int) -> c_int;
    }
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "cdecl" {
        //pub fn dx_DrawFormatString(x:c_int,y:c_int,ck\olor:\\\);
    }
}

/*wrap function*/
pub fn dx_ClearDrawScreen() -> c_int {
    let mut tmp = RECT {
        left: -1,
        right: -1,
        top: -1,
        bottom: -1,
    };
    unsafe { hidden::dx_ClearDrawScreen(&mut tmp) }
}

pub fn dx_LoadGraph(FileName: &str) -> c_int {
    unsafe {
        return hidden::dx_LoadGraph(FileName.to_cstring().as_ptr(), FALSE);
    }
}

pub fn dx_PlaySoundFile(FileName: &str, PlayType: c_int) -> c_int {
    unsafe {
        return hidden::dx_PlaySoundFile(FileName.to_cstring().as_ptr(), PlayType);
    }
}

pub fn dx_LoadSoundMem(FileName: &str) -> c_int {
    unsafe {
        return hidden::dx_LoadSoundMem(FileName.to_cstring().as_ptr());
    }
}

pub fn dx_PlaySoundMem(SoundHandle: c_int, PlayType: c_int) -> c_int {
    unsafe {
        return hidden::dx_PlaySoundMem(SoundHandle, PlayType);
    }
}
pub fn dx_DrawString(x: c_int, y: c_int, String: &str, Color: Color) -> c_int {
    unsafe {
        return hidden::dx_DrawString(x, y, String.to_cstring().as_ptr(), Color);
    }
}
pub fn dx_MV1LoadModel(FileName: &str) -> c_int {
    unsafe {
        return hidden::dx_MV1LoadModel(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_ChangeFont(FileName: &str) -> c_int {
    unsafe {
        return hidden::dx_ChangeFont(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_CreateFontToHandle(FontName: &str, Size: c_int, Thick: c_int, FontType: c_int) -> c_int {
    unsafe {
        return hidden::dx_CreateFontToHandle(
            FontName.to_cstring().as_ptr(),
            Size,
            Thick,
            FontType,
        );
    }
}
pub fn dx_DrawStringToHandle(
    x: c_int,
    y: c_int,
    String: &str,
    Color: Color,
    FontHandle: c_int,
) -> c_int {
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
pub fn dx_LoadFontDataToHandle(FileName: &str, EdgeSize: c_int) -> c_int {
    unsafe {
        return hidden::dx_LoadFontDataToHandle(FileName.to_cstring().as_ptr(), EdgeSize);
    }
}
pub fn dx_SetDXArchiveExtension(Extension: &str) -> c_int {
    unsafe {
        return hidden::dx_SetDXArchiveExtension(Extension.to_cstring().as_ptr());
    }
}
pub fn dx_SaveDrawScreen(x1: c_int, y1: c_int, x2: c_int, y2: c_int, FileName: &str) -> c_int {
    unsafe {
        return hidden::dx_SaveDrawScreen(x1, y1, x2, y2, FileName.to_cstring().as_ptr());
    }
}
pub fn dx_SetMainWindowText(WindowText: &str) -> c_int {
    unsafe {
        return hidden::dx_SetMainWindowText(WindowText.to_cstring().as_ptr());
    }
}
pub fn dx_LoadDivGraph(
    FileName: &str,
    AllNum: c_int,
    XNum: c_int,
    YNum: c_int,
    XSize: c_int,
    YSize: c_int,
    HandleBuf: *mut c_int,
) -> c_int {
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
pub fn dx_LoadGraphScreen(x: c_int, y: c_int, GraphName: &str, TransFlag: c_int) -> c_int {
    unsafe {
        return hidden::dx_LoadGraphScreen(x, y, GraphName.to_cstring().as_ptr(), TransFlag);
    }
}
pub fn dx_GetGraphSize(GrHandle: c_int, SizeXBuf: &mut c_int, SizeYBuf: &mut c_int) -> c_int {
    unsafe {
        return hidden::dx_GetGraphSize(
            GrHandle,
            &mut *SizeXBuf as *mut c_int,
            &mut *SizeYBuf as *mut c_int,
        );
    }
}
pub fn dx_LoadBlendGraph(FileName: &str) -> c_int {
    unsafe {
        return hidden::dx_LoadBlendGraph(FileName.to_cstring().as_ptr());
    }
}

pub fn dx_GetDateTime(DateBuf: &mut DATEDATA) -> c_int {
    unsafe {
        return hidden::dx_GetDateTime(&mut *DateBuf as *mut DATEDATA);
    }
}
pub fn dx_GetDrawStringWidth(String: &str, StrLen: c_int) -> c_int {
    unsafe {
        return hidden::dx_GetDrawStringWidth(String.to_cstring().as_ptr(), StrLen);
    }
}
pub fn dx_GetDrawStringWidthToHandle(String: &str, StrLen: c_int, FontHandle: c_int) -> c_int {
    unsafe {
        return hidden::dx_GetDrawStringWidthToHandle(
            String.to_cstring().as_ptr(),
            StrLen,
            FontHandle,
        );
    }
}
pub fn dx_GetFontStateToHandle(
    FontName: &str,
    Size: &mut c_int,
    Thick: &mut c_int,
    FontHandle: c_int,
) -> c_int {
    unsafe {
        return hidden::dx_GetFontStateToHandle(
            FontName.to_cstring().as_ptr(),
            &mut *Size as *mut c_int,
            &mut *Thick as *mut c_int,
            FontHandle,
        );
    }
}
pub fn dx_GetScreenState(SizeX:&mut c_int,SizeY:&mut c_int,ColorBitDepth:&mut c_int) -> c_int{
    unsafe{
        return hidden::dx_GetScreenState(&mut *SizeX as *mut c_int,&mut *SizeY as *mut c_int ,&mut *ColorBitDepth as *mut c_int);
    }
}
