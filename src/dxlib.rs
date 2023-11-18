#![allow(non_snake_case)]
pub use crate::c_encode::*;
pub use crate::dxlib_common::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;
use std::os::raw::*;
use std::vec::Vec;

/// dxlib function extern declaration (based on Ver3.24b)
/// see: https://dxlib.xsrv.jp/dxfunc.html
#[link(name = "DxLib_x64")]
#[no_mangle]
extern "stdcall" {
    // 使用必須関数

    /// ライブラリの初期化
    pub fn dx_DxLib_Init() -> CInt;
    /// ライブラリ使用の終了関数
    pub fn dx_DxLib_End() -> CInt;
    /// ウインドウズのメッセージを処理する
    pub fn dx_ProcessMessage() -> CInt;

    // Live2D 関係関数
    pub fn dx_Live2D_SetCubism4CoreDLLPath(CoreDLLFilePath: *const CChar);
    pub fn dx_Live2D_RenderBegin();
    pub fn dx_Live2D_RenderEnd();
    pub fn dx_Live2D_LoadModel(FilePath: *const CChar);
    pub fn dx_Live2D_DeleteModel(Live2DModelHandle: CInt);
    pub fn dx_Live2D_Model_Update(Live2DModelHandle: CInt, DeltaTimeSeconds: CFloat);
    pub fn dx_Live2D_Model_Draw(Live2DModelHandle: CInt);
    pub fn dx_Live2D_Model_SetTranslate(Live2DModelHandle: CInt, x: CFloat, y: CFloat) -> CInt;
    pub fn dx_Live2D_Model_SetExtendRate(
        Live2DModelHandle: CInt,
        ExRateX: CFloat,
        ExRateY: CFloat,
    ) -> CInt;
    pub fn dx_Live2D_Model_SetRotate(Live2DModelHandle: CInt, RotAngle: CFloat) -> CInt;
    pub fn dx_Live2D_Model_StartMotion(
        Live2DModelHandle: CInt,
        group: *const CChar,
        no: CInt,
    ) -> CInt;

    pub fn dx_Live2D_Model_IsMotionFinished(Live2DModelHandle: CInt) -> CInt;
    pub fn dx_Live2D_Model_SetExpression(
        Live2DModelHandle: CInt,
        expressionID: *const CChar,
    ) -> CInt;

    // ３Ｄ関係関数

    //３Ｄ図形描画関係関数
    pub fn dx_DrawLine3D(Pos1: VECTOR, Pos2: VECTOR, Color: Color) -> CInt;
    pub fn dx_DrawTriangle3D(
        Pos1: VECTOR,
        Pos2: VECTOR,
        Pos3: VECTOR,
        Color: Color,
        FillFlag: CInt,
    ) -> CInt;

    pub fn dx_DrawSphere3D(
        CenterPos: VECTOR,
        r: CFloat,
        DivNum: CInt,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: CInt,
    ) -> CInt;

    pub fn dx_DrawCapsule3D(
        Pos1: VECTOR,
        Pos2: VECTOR,
        r: CFloat,
        DivNum: CInt,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: CInt,
    ) -> CInt;

    pub fn dx_DrawCone3D(
        TopPos: VECTOR,
        BottomPos: VECTOR,
        r: CFloat,
        DivNum: CInt,
        DifColor: u32,
        SpcColor: u32,
        FillFlag: CInt,
    ) -> CInt;

    pub fn dx_DrawBillboard3D(
        Pos: VECTOR,
        cx: CFloat,
        cy: CFloat,
        Size: CFloat,
        Angle: CFloat,
        GrHandle: CInt,
        TransFlag: CInt,
    ) -> CInt;
    pub fn dx_DrawPolygonIndexed3D(
        Vertex: *mut VERTEX3D,
        VertexNum: CInt,
        Indices: *mut u16,
        PolygonNum: CInt,
        GrHandle: CInt,
        TransFlag: CInt,
    ) -> CInt;
    pub fn dx_DrawPolygon3D(
        Vertex: *mut VERTEX3D,
        PolygonNum: CInt,
        GrHandle: CInt,
        TransFlag: CInt,
    ) -> CInt;

    pub fn dx_SetMaterialUseVertDifColor(UseFlag: CInt) -> CInt;
    pub fn dx_SetMaterialUseVertSpcColor(UseFlag: CInt) -> CInt;
    pub fn dx_SetMaterialParam(Material: MATERIALPARAM) -> CInt;

    // Zバッファを使うかどうかのフラグ
    pub fn dx_SetUseZBuffer3D(Flag: CInt) -> CInt;
    // Zバッファへの書き込みするかどうかのフラグ
    pub fn dx_SetWriteZBuffer3D(Flag: CInt) -> CInt;
    pub fn dx_SetUseBackCulling(Flag: CInt) -> CInt;
    pub fn dx_SetTextureAddressModeUV(ModeU: CInt, ModeV: CInt) -> CInt;
    pub fn dx_SetFogEnable(Flag: CInt) -> CInt;
    pub fn dx_SetFogColor(Red: CInt, Green: CInt, Blue: CInt) -> CInt;
    pub fn dx_SetFogStartEnd(start: CFloat, end: CFloat) -> CInt;
    pub fn dx_GetColorF(Red: CFloat, Green: CFloat, Blue: CFloat, Alpha: CFloat) -> COLOR_F;
    pub fn dx_GetColorU8(Red: CInt, Green: CInt, Blue: CInt, Alpha: CInt) -> COLOR_U8;

    // カメラ関数
    pub fn dx_SetCameraPositionAndTarget_UpVecY(Position: VECTOR, Target: VECTOR) -> CInt;
    pub fn dx_SetCameraNearFar(Near: CFloat, Far: CFloat) -> CInt;
    pub fn dx_SetCameraPositionAndTargetAndUpVec(
        Position: VECTOR,
        Target: VECTOR,
        Up: VECTOR,
    ) -> CInt;
    pub fn dx_SetCameraPositionAndAngle(
        Position: VECTOR,
        VRotate: CFloat,
        HRotate: CFloat,
        TRotate: CFloat,
    ) -> CInt;
    pub fn dx_SetCameraViewMatrix(ViewMatrix: MATRIX) -> CInt;
    pub fn dx_SetupCamera_Perspective(Fov: CFloat) -> CInt;
    pub fn dx_SetupCamera_Ortho(Size: CFloat) -> CInt;
    pub fn dx_SetupCamera_ProjectionMatrix(ProjectionMatrix: MATRIX) -> CInt;
    pub fn dx_SetCameraDotAspect(DotAspect: CFloat) -> CInt;
    pub fn dx_ConvWorldPosToScreenPos(WorldPos: VECTOR) -> VECTOR;
    pub fn dx_ConvScreenPosToWorldPos(ScreenPos: VECTOR) -> VECTOR;
    pub fn dx_SetCameraScreenCenter(x: CFloat, y: CFloat) -> CInt;
    pub fn dx_CheckCameraViewClip(CheckPos: VECTOR) -> CInt;
    pub fn dx_CheckCameraViewClip_Box(BoxPos1: VECTOR, BoxPos2: VECTOR) -> CInt;
    pub fn dx_GetCameraViewMatrix() -> MATRIX;
    pub fn dx_GetCameraProjectionMatrix() -> MATRIX;
    // ライト関数
    pub fn dx_SetUseLighting(Flag: CInt) -> CInt;
    pub fn dx_SetGlobalAmbientLight(Color: COLOR_F) -> CInt;

    //標準ライト関数
    pub fn dx_ChangeLightTypeDir(Direction: VECTOR) -> CInt;
    pub fn dx_ChangeLightTypePoint(
        Position: VECTOR,
        Range: CFloat,
        Atten0: CFloat,
        Atten1: CFloat,
        Atten2: CFloat,
    ) -> CInt;

    pub fn dx_ChangeLightTypeSpot(
        Position: VECTOR,
        Direction: VECTOR,
        OutAngle: CFloat,
        InAngle: CFloat,
        Range: CFloat,
        Atten0: CFloat,
        Atten1: CFloat,
        Atten2: CFloat,
    ) -> CInt;
    pub fn dx_SetLightEnable(EnableFlag: CInt) -> CInt;
    pub fn dx_SetLightDifColor(Color: COLOR_F) -> CInt;

    pub fn dx_SetLightSpcColor(Color: COLOR_F) -> CInt;
    pub fn dx_SetLightAmbColor(Color: COLOR_F) -> CInt;
    pub fn dx_SetLightDirection(Direction: VECTOR) -> CInt;
    pub fn dx_SetLightPosition(Position: VECTOR) -> CInt;
    pub fn dx_SetLightRangeAtten(
        Range: CFloat,
        Atten0: CFloat,
        Atten1: CFloat,
        Atten2: CFloat,
    ) -> CInt;

    pub fn dx_SetLightAngle(OutAngle: CFloat, InAngle: CFloat) -> CInt;

    pub fn dx_GetLightType() -> CInt;

    pub fn dx_GetLightEnable() -> CInt;
    pub fn dx_GetLightDifColor() -> COLOR_F;
    pub fn dx_GetLightSpcColor() -> COLOR_F;
    pub fn dx_GetLightAmbColor() -> COLOR_F;
    pub fn dx_GetLightDirection() -> VECTOR;
    pub fn dx_GetLightPosition() -> VECTOR;
    pub fn dx_GetLightRangeAtten(
        Range: *mut CFloat,
        Atten0: *mut CFloat,
        Atten1: *mut CFloat,
        Atten2: *mut CFloat,
    ) -> CInt;
    pub fn dx_GetLightAngle(OutAngle: *mut CFloat, InAngle: *mut CFloat) -> CInt;

    //ライトハンドル関数

    pub fn dx_CreateDirLightHandle(Direction: VECTOR) -> CInt;

    pub fn dx_CreatePointLightHandle(
        Position: VECTOR,
        Range: CFloat,
        Atten0: CFloat,
        Atten1: CFloat,
        Atten2: CFloat,
    ) -> CInt;

    pub fn dx_CreateSpotLightHandle(
        Position: VECTOR,
        Direction: VECTOR,
        OutAngle: CFloat,
        InAngle: CFloat,
        Range: CFloat,
        Atten0: CFloat,
        Atten1: CFloat,
        Atten2: CFloat,
    ) -> CInt;
    pub fn dx_DeleteLightHandle(LHandle: CInt) -> CInt;

    pub fn dx_DeleteLightHandleAll() -> CInt;
    pub fn dx_SetLightTypeHandle(LHandle: CInt, LightType: CInt) -> CInt;
    pub fn dx_SetLightEnableHandle(LHandle: CInt, EnableFlag: CInt) -> CInt;
    pub fn dx_SetLightDifColorHandle(LHandle: CInt, Color: COLOR_F) -> CInt;
    pub fn dx_SetLightSpcColorHandle(LHandle: CInt, Color: COLOR_F) -> CInt;
    pub fn dx_SetLightAmbColorHandle(LHandle: CInt, Color: COLOR_F) -> CInt;
    pub fn dx_SetLightDirectionHandle(LHandle: CInt, Direction: VECTOR) -> CInt;

    pub fn dx_SetLightPositionHandle(LHandle: CInt, Position: VECTOR) -> CInt;

    pub fn dx_SetLightRangeAttenHandle(
        LHandle: CInt,
        Range: CFloat,
        Atten0: CFloat,
        Atten1: CFloat,
        Atten2: CFloat,
    ) -> CInt;

    pub fn dx_SetLightAngleHandle(LHandle: CInt, OutAngle: CFloat, InAngle: CFloat) -> CInt;
    pub fn dx_GetLightTypeHandle(LHandle: CInt) -> CInt;

    pub fn dx_GetLightEnableHandle(LHandle: CInt) -> CInt;
    pub fn dx_GetLightDifColorHandle(LHandle: CInt) -> COLOR_F;
    pub fn dx_GetLightSpcColorHandle(LHandle: CInt) -> COLOR_F;
    pub fn dx_GetLightAmbColorHandle(LHandle: CInt) -> COLOR_F;

    pub fn dx_GetLightDirectionHandle(LHandle: CInt) -> VECTOR;

    pub fn dx_GetLightPositionHandle(LHandle: CInt) -> VECTOR;

    pub fn dx_GetLightRangeAttenHandle(
        LHandle: CInt,
        Range: *mut CFloat,
        Atten0: *mut CFloat,
        Atten1: *mut CFloat,
        Atten2: *mut CFloat,
    ) -> CInt;

    pub fn dx_GetLightAngleHandle(
        LHandle: CInt,
        OutAngle: *mut CFloat,
        InAngle: *mut CFloat,
    ) -> CInt;

    pub fn dx_GetEnableLightHandleNum() -> CInt;
    pub fn dx_GetEnableLightHandle(Index: CInt) -> CInt;

    //算術演算関数
    pub fn dx_VGet(x: CFloat, y: CFloat, z: CFloat) -> VECTOR;
    pub fn dx_VAdd(In1: VECTOR, In2: VECTOR) -> VECTOR;
    pub fn dx_VSub(In1: VECTOR, In2: VECTOR) -> VECTOR;
    pub fn dx_VDot(In1: VECTOR, In2: VECTOR) -> CFloat;
    pub fn dx_VCross(In1: VECTOR, In2: VECTOR) -> VECTOR;
    pub fn dx_VScale(In: VECTOR, Scale: CFloat) -> VECTOR;
    pub fn dx_VSize(In: VECTOR) -> CFloat;
    pub fn dx_VSquareSize(In: VECTOR) -> CFloat;
    pub fn dx_VNorm(In: VECTOR) -> VECTOR;
    pub fn dx_VTransform(InV: VECTOR, InM: MATRIX) -> VECTOR;
    pub fn dx_VTransformSR(InV: VECTOR, InM: MATRIX) -> VECTOR;

    pub fn dx_MGetIdent() -> MATRIX;
    pub fn dx_MGetScale(Scale: VECTOR) -> MATRIX;
    pub fn dx_MGetTranslate(Trans: VECTOR) -> MATRIX;
    pub fn dx_MGetRotX(XAxisRotate: CFloat) -> MATRIX;

    pub fn dx_MGetRotY(YAxisRotate: CFloat) -> MATRIX;
    pub fn dx_MGetRotZ(ZAxisRotate: CFloat) -> MATRIX;

    pub fn dx_MGetRotAxis(RotateAxis: VECTOR, Rotate: CFloat) -> MATRIX;
    pub fn dx_MGetRotVec2(In1: VECTOR, In2: VECTOR) -> MATRIX;

    pub fn dx_MGetAxis1(XAxis: VECTOR, YAxis: VECTOR, ZAxis: VECTOR, Pos: VECTOR) -> MATRIX;

    pub fn dx_MGetAxis2(XAxis: VECTOR, YAxis: VECTOR, ZAxis: VECTOR, Pos: VECTOR) -> MATRIX;
    pub fn dx_MAdd(In1: MATRIX, In2: MATRIX) -> MATRIX;
    pub fn dx_MMult(In1: MATRIX, In2: MATRIX) -> MATRIX;
    pub fn dx_MScale(InM: MATRIX, Scale: CFloat) -> MATRIX;
    pub fn dx_MTranspose(InM: MATRIX) -> MATRIX;
    pub fn dx_MInverse(InM: MATRIX) -> MATRIX;

    // 衝突検出系関数
    pub fn dx_Segment_Segment_MinLength(
        SegmentAPos1: VECTOR,
        SegmentAPos2: VECTOR,
        SegmentBPos1: VECTOR,
        SegmentBPos2: VECTOR,
    ) -> CFloat;
    pub fn dx_Segment_Triangle_MinLength(
        SegmentPos1: VECTOR,
        SegmentPos2: VECTOR,
        TrianglePos1: VECTOR,
        TrianglePos2: VECTOR,
        TrianglePos3: VECTOR,
    ) -> CFloat;
    pub fn dx_Segment_Point_MinLength(
        SegmentPos1: VECTOR,
        SegmentPos2: VECTOR,
        PointPos: VECTOR,
    ) -> CFloat;

    pub fn dx_HitCheck_Line_Triangle(
        LinePos1: VECTOR,
        LinePos2: VECTOR,
        TrianglePos1: VECTOR,
        TrianglePos2: VECTOR,
        TrianglePos3: VECTOR,
    ) -> HITRESULT_LINE;

    // ３Ｄモデル関係の関数

    // モデルの読み込み・複製関係の関数

    // モデルの読み込み
    //pub fn dx_MV1LoadModel(FileName: *const CChar) -> CInt;
    pub fn dx_MV1DuplicateModel(SrcMHandle: CInt) -> CInt;
    pub fn dx_MV1DeleteModel(MHandle: CInt) -> CInt;
    pub fn dx_MV1SetLoadModelUsePhysicsMode(PhysicsMode: CInt) -> CInt;
    pub fn dx_MV1SetLoadModelPhysicsWorldGravity(Gravity: CFloat) -> CInt;

    // モデル描画関数

    // モデルの描画
    pub fn dx_MV1DrawModel(MHandle: CInt) -> CInt;
    pub fn dx_MV1DrawFrame(MHandle: CInt, FrameIndex: CInt) -> CInt;
    pub fn dx_MV1DrawMesh(MHandle: CInt, MeshIndex: CInt) -> CInt;
    pub fn dx_MV1DrawTriangleList(MHandle: CInt, TriangleListIndex: CInt) -> CInt;

    // モデル描画設定関数
    pub fn dx_MV1SetUseOrigShader(UseFlag: CInt) -> CInt;

    // モデル基本制御関数

    // モデルの位置の指定
    pub fn dx_MV1SetPosition(MHandle: CInt, Position: VECTOR) -> CInt;

    pub fn dx_MV1GetPosition(MHandle: CInt) -> VECTOR;
    pub fn dx_MV1SetScale(MHandle: CInt, Scale: VECTOR) -> CInt;

    pub fn dx_MV1GetScale(MHandle: CInt) -> VECTOR;

    pub fn dx_MV1SetRotationXYZ(MHandle: CInt, Rotate: VECTOR) -> CInt;
    pub fn dx_MV1GetRotationXYZ(MHandle: CInt) -> VECTOR;
    pub fn dx_MV1SetRotationZYAxis(
        MHandle: CInt,
        ZAxis: VECTOR,
        YAxis: VECTOR,
        ZTwist: CFloat,
    ) -> CInt;
    pub fn dx_MV1SetMatrix(MHandle: CInt, Matrix: MATRIX) -> CInt;
    pub fn dx_MV1GetMatrix(MHandle: CInt) -> MATRIX;
    pub fn dx_MV1SetVisible(MHandle: CInt, VisibleFlag: CInt) -> CInt;
    pub fn dx_MV1GetVisible(MHandle: CInt) -> CInt;
    pub fn dx_MV1SetDifColorScale(MHandle: CInt, Scale: COLOR_F) -> CInt;

    pub fn dx_MV1SetUseVertDifColor(MHandle: CInt, UseFlag: CInt) -> CInt;
    pub fn dx_MV1SetUseVertSpcColor(MHandle: CInt, UseFlag: CInt) -> CInt;
    pub fn dx_MV1PhysicsCalculation(MHandle: CInt, MillisecondTime: CFloat) -> CInt;

    // アニメーション関数
    pub fn dx_MV1AttachAnim(
        MHandle: CInt,
        AnimIndex: CInt,
        AnimSrcMHandle: CInt,
        NameCheck: CInt,
    ) -> CInt;
    pub fn dx_MV1GetAnimNum(MHandle: CInt) -> CInt;
    pub fn dx_MV1DetachAnim(MHandle: CInt, AttachIndex: CInt) -> CInt;
    pub fn dx_MV1SetAttachAnimTime(MHandle: CInt, AttachIndex: CInt, Time: CFloat) -> CInt;
    pub fn dx_MV1GetAttachAnimTotalTime(MHandle: CInt, AttachIndex: CInt) -> CFloat;
    pub fn dx_MV1GetAttachAnimTime(MHandle: CInt, AttachIndex: CInt) -> CFloat;
    pub fn dx_MV1SetAttachAnimBlendRate(MHandle: CInt, AttachIndex: CInt, Rate: CFloat) -> CInt;

    // シェイプ関数

    pub fn dx_MV1SetShapeRate(MHandle: CInt, ShapeIndex: CInt, Rate: CFloat) -> CInt;

    // 図形描画関数

    /// 線を描画
    pub fn dx_DrawLine(x1: CInt, y1: CInt, x2: CInt, y2: CInt, color: Color) -> CInt;
    /// 線を描画(アンチエイリアス効果付き)
    pub fn dx_DrawLineAA(x1: CFloat, y1: CFloat, x2: CFloat, y2: CFloat, color: Color) -> CInt;
    /// 四角を描画
    pub fn dx_DrawBox(
        x1: CInt,
        y1: CInt,
        x2: CInt,
        y2: CInt,
        color: Color,
        fill_flag: CInt,
    ) -> CInt;
    /// 四角を描画(アンチエイリアス効果付き)
    pub fn dx_DrawBoxAA(
        x1: CFloat,
        y1: CFloat,
        x2: CFloat,
        y2: CFloat,
        color: Color,
        fill_flag: CInt,
    ) -> CInt;
    /// 円の描画
    pub fn dx_DrawCircle(x: CInt, y: CInt, r: CInt, color: Color, fill_flag: CInt) -> CInt;
    /// 円の描画(アンチエイリアス効果付き)
    pub fn dx_DrawCircleAA(
        x: CFloat,
        y: CFloat,
        r: CFloat,
        pos_num: CInt,
        color: Color,
        fill_flag: CInt,
    ) -> CInt;
    /// 楕円の描画
    pub fn dx_DrawOval(x: CInt, y: CInt, rx: CInt, ry: CInt, Color: Color, FillFlag: CInt) -> CInt;
    /// 楕円の描画(アンチエイリアス効果付き)
    pub fn dx_DrawOvalAA(
        x: CFloat,
        y: CFloat,
        rx: CFloat,
        ry: CFloat,
        posnum: CInt,
        Color: Color,
        FillFlag: CInt,
    ) -> CInt;
    /// 三角形の描画
    pub fn dx_DrawTriangle(
        x1: CInt,
        y1: CInt,
        x2: CInt,
        y2: CInt,
        x3: CInt,
        y3: CInt,
        Color: Color,
        FillFlag: CInt,
    ) -> CInt;
    /// 三角形の描画(アンチエイリアス効果付き)
    pub fn dx_DrawTriangleAA(
        x1: CFloat,
        y1: CFloat,
        x2: CFloat,
        y2: CFloat,
        x3: CFloat,
        y3: CFloat,
        Color: Color,
        FillFlag: CInt,
    ) -> CInt;
    /// 点を描画する
    pub fn dx_DrawPixel(x: CInt, y: CInt, Color: Color) -> CInt;
    /// 指定点の色を取得
    pub fn dx_GetPixel(x: CInt, y: CInt) -> CInt;

    // グラフィックデータ制御関数

    /// 画像ファイルを読みこんで画面に表示する
    //pub fn dx_LoadGraphScreen() -> CInt;
    /// 画像ファイルのメモリへの読みこみ、及び動画ファイルのロード
    //pub fn dx_LoadGraph() -> CInt;
    /// 画像ファイルのメモリへの分割読みこみ
    //pub fn dx_LoadDivGraph(FileName:*const CChar,AllNum:CInt,XNum:CInt,YNum:CInt,XSize:CInt,YSize:CInt,HandleBuf:*mut CInt) -> CInt;
    /// 空のグラフィックを作成する
    pub fn dx_MakeGraph(SizeX: CInt, SizeY: CInt) -> CInt;
    /// 描画対象にできるグラフィックを作成する
    pub fn dx_MakeScreen(SizeX: CInt, SizeY: CInt, UseAlphaChannel: CInt) -> CInt;
    /// 描画対象にできるグラフィックのマルチサンプリング設定を行う
    pub fn dx_SetCreateDrawValidGraphMultiSample(Samples: CInt, Quality: CInt) -> CInt;
    /// 作成するグラフィックのビット深度を設定する
    pub fn dx_SetCreateGraphColorBitDepth(BitDepth: CInt) -> CInt;
    /// 描画可能な浮動小数点型のグラフィックを作成するかどうかの設定を行う
    pub fn dx_SetDrawValidFloatTypeGraphCreateFlag(Flag: CInt) -> CInt;
    /// 作成する描画可能なグラフィックのチャンネル数の設定を行う
    pub fn dx_SetCreateDrawValidGraphChannelNum(ChannelNum: CInt) -> CInt;
    /// 読み込み時に画像を乗算済みα画像に変換するかを設定する
    pub fn dx_SetUsePremulAlphaConvertLoad(UseFlag: CInt) -> CInt;
    /// メモリに読みこんだグラフィックの描画
    pub fn dx_DrawGraph(x: CInt, y: CInt, GrHandle: CInt, TransFlag: CInt) -> CInt;
    // メモリに読み込んだグラフィックの描画　(float版)
    pub fn dx_DrawGraphF(x: CFloat, y: CFloat, GrHandle: CInt, TransFlag: CInt) -> CInt;
    /// メモリに読みこんだグラフィックのＬＲ反転描画
    pub fn dx_DrawTurnGraph(
        x1: CInt,
        y1: CInt,
        x2: CInt,
        y2: CInt,
        GrHandle: CInt,
        TransFlag: CInt,
    ) -> CInt;
    /// メモリに読みこんだグラフィックの拡大縮小描画
    pub fn dx_DrawExtendGraph(
        x1: CInt,
        y1: CInt,
        x2: CInt,
        y2: CInt,
        GrHandle: CInt,
        TransFlag: CInt,
    ) -> CInt;
    /// メモリに読みこんだグラフィックの回転描画
    pub fn dx_DrawRotaGraph(
        x: CInt,
        y: CInt,
        ex_rate: f64,
        angle: f64,
        gr_handle: CInt,
        trans_flag: CInt,
        turn_flag: CInt,
    ) -> CInt;
    /// メモリに読みこんだグラフィックの回転描画(回転中心指定あり)
    pub fn dx_DrawRotaGraph2(
        x: CInt,
        y: CInt,
        cx: CInt,
        cy: CInt,
        ExtRate: f64,
        Angle: f64,
        GrHandle: CInt,
        TransFlag: CInt,
        TurnFlag: CInt,
    ) -> CInt;
    /// メモリに読みこんだグラフィックの回転描画(回転中心指定あり、縦横拡大率別指定)
    pub fn dx_DrawRotaGraph3(
        x: CInt,
        y: CInt,
        cx: CInt,
        cy: CInt,
        ExtRateX: f64,
        ExtRateY: f64,
        Angle: f64,
        GrHandle: CInt,
        TransFlag: CInt,
        TurnFlag: CInt,
    ) -> CInt;
    /// メモリに読みこんだグラフィックの自由変形描画
    pub fn dx_DrawModiGraph(
        x1: CInt,
        y1: CInt,
        x2: CInt,
        y2: CInt,
        x3: CInt,
        y3: CInt,
        x4: CInt,
        y4: CInt,
        GrHandle: CInt,
        TransFlag: CInt,
    ) -> CInt;
    /// グラフィックの指定矩形部分のみを描画
    pub fn dx_DrawRectGraph(
        DestX: CInt,
        DestY: CInt,
        SrcX: CInt,
        SrcY: CInt,
        Width: CInt,
        Height: CInt,
        GraphHandle: CInt,
        TransFlag: CInt,
        TurnFlag: CInt,
    ) -> CInt;
    /// 指定のグラフィックの指定部分だけを抜き出して新たなグラフィックを作成する
    pub fn dx_DerivationGraph(
        SrcX: CInt,
        SrcY: CInt,
        Width: CInt,
        Height: CInt,
        SrcGraphHandle: CInt,
    ) -> CInt;
    /// 描画先に設定されているグラフィック領域から指定領域のグラフィックを読みこむ
    pub fn dx_GetDrawScreenGraph(x1: CInt, y1: CInt, x2: CInt, y2: CInt, GrHandle: CInt) -> CInt;
    /// グラフィックのサイズを得る
    //pub fn dx_GetGraphSize() -> CInt;
    /// 読みこんだグラフィックデータをすべて削除する
    pub fn dx_InitGraph() -> CInt;
    /// 指定のグラフィックをメモリ上から削除する
    pub fn dx_DeleteGraph(GrHandle: CInt) -> CInt;
    /// 描画モードをセットする
    pub fn dx_SetDrawMode(DrawMode: CInt) -> CInt;
    /// 描画の際のブレンドモードをセットする
    pub fn dx_SetDrawBlendMode(blend_mode: CInt, pal: CInt) -> CInt;
    /// 描画輝度をセット
    pub fn dx_SetDrawBright(RedBright: CInt, GreenBright: CInt, BlueBright: CInt) -> CInt;
    /// グラフィックに設定する透過色をセットする
    pub fn dx_SetTransColor(Red: CInt, Green: CInt, Blue: CInt) -> CInt;
    // 画像ファイルからブレンド画像を読み込む
    //pub fn dx_LoadBlendGraph() -> CInt;
    /// ブレンド画像と通常画像を合成して描画する
    pub fn dx_DrawBlendGraph(
        x: CInt,
        y: CInt,
        GrHandle: CInt,
        TransFlag: CInt,
        BlendGraph: CInt,
        BorderParam: CInt,
        BorderRange: CInt,
    ) -> CInt;
    /// 画像にフィルター処理を施す
    //pub fn dx_GraphFilter() -> CInt;
    /// 画像にフィルター処理を施す( 出力先画像指定版 )
    //pub fn dx_GraphFilterBlt() -> CInt;
    /// 画像にフィルター処理を施す( 出力先画像、使用矩形指定版 )
    //pub fn dx_GraphFilterRectBlt() -> CInt;
    /// 二つの画像を特殊効果付きでブレンドする
    //pub fn dx_GraphBlend() -> CInt;
    /// 二つの画像を特殊効果付きでブレンドする( 出力先画像指定版 )
    //pub fn dx_GraphBlendBlt() -> CInt;
    /// 二つの画像を特殊効果付きでブレンドする( 出力先画像、使用矩形指定版 )
    //pub fn dx_GraphBlendRectBlt() -> CInt;

    // 文字描画関係関数

    /// 文字列を描画する
    //  pub fn dx_DrawString(x: CInt, y: CInt, string: *const CChar, color: Color) -> CInt;
    /// 書式付き文字列を描画する
    //pub fn dx_DrawFormatString(x:CInt,y:CInt,color:Color,format:*const CChar, ...) -> CInt;
    /// DrawString で描画される文字列の幅(ドット単位)を得る
    //pub fn dx_GetDrawStringWidth() -> CInt;
    /// DrawFormatString 関数書式付き文字列の描画幅(ドット単位)を得る
    //pub fn dx_GetDrawFormatStringWidth() -> CInt;
    /// 描画する文字列のサイズをセットする
    pub fn dx_SetFontSize(size: CInt) -> CInt;
    /// 描画する文字列の文字の太さをセットする
    pub fn dx_SetFontThickness(TinckPal: CInt) -> CInt;
    /// 描画するフォントを変更する
    //pub fn dx_ChangeFont(name: *const CChar) -> CInt;
    /// 文字列描画に使用するフォントのタイプを変更する
    pub fn dx_ChangeFontType(FontType: CInt) -> CInt;
    /// 新しいフォントデータを作成
    //pub fn dx_CreateFontToHandle(FontName: *const CChar,Size: CInt,Thick: CInt,FontType: CInt) -> CInt;
    /// ＤＸフォントデータファイルを読み込む
    // pub fn dx_LoadFontDataToHandle(FileName: *const CChar, EdgeSize: CInt) -> CInt;
    /// フォントデータを削除する
    pub fn dx_DeleteFontToHandle(FontHandle: CInt) -> CInt;
    /// 作成するフォントデータを『乗算済みα』用にするかどうかを設定する
    pub fn dx_SetFontCacheUsePremulAlphaFlag(Flag: CInt) -> CInt;
    /// 指定のフォントデータで文字列を描画する
    // pub fn dx_DrawStringToHandle(x: CInt,y: CInt,String: *const CChar,Color: Color,FontHandle: CInt,) -> CInt;
    /// 指定のフォントデータで書式付き文字列を描画する
    //pub fn dx_DrawFormatStringToHandle() -> CInt;
    /// 指定のフォントデータで描画する文字列の幅を得る
    //pub fn dx_GetDrawStringWidthToHandle(String:*const CChar,StrLen:CInt, FontHandle:CInt) -> CInt;
    /// 指定のフォントデータで書式付き文字列の描画幅を得る
    //pub fn dx_GetDrawFormatStringWidthToHandle() -> CInt;
    /// 指定のフォントデータの情報を得る
    //pub fn dx_GetFontStateToHandle(FontName:*const CChar,Size:*mut CInt,Thick:*mut CInt,FontHandle:CInt) -> CInt;
    /// フォントデータを全て初期化する
    pub fn dx_InitFontToHandle() -> CInt;

    // 簡易画面出力関数

    //pub fn dx_() -> CInt;
    // 簡易文字列描画

    //pub unsafe fn  dx_printfDx(arg: T, arg2: U, mut args: ...) -> CInt;
    /// 簡易画面出力履歴をクリアする
    //pub fn dx_clsDx() -> CInt;

    // その他画面操作系関数

    /// 画面モードの変更
    pub fn dx_SetGraphMode(SizeX: CInt, SizeY: CInt, ColorBitNum: CInt, RefreshRate: CInt) -> CInt;
    /// フルスクリーンモード時の解像度モードを設定する
    pub fn dx_SetFullScreenResolutionMode(ResolutionMode: CInt) -> CInt;
    /// フルスクリーンモード時の画面拡大モードを設定する
    pub fn dx_SetFullScreenScalingMode(ScalingMode: CInt) -> CInt;
    /// 現在の画面の大きさとカラービット数を得る
    //pub fn dx_GetScreenState(SizeX:*mut CInt,SizeY:*mut CInt,ColorBitDepth:*mut CInt) -> CInt;
    /// 描画可能領域のセット
    pub fn dx_SetDrawArea(x1: CInt, y1: CInt, x2: CInt, y2: CInt) -> CInt;
    /// 画面に描かれたものを消去する
    //pub fn dx_ClearDrawScreen() -> CInt;
    /// 画面の背景色を設定する
    pub fn dx_SetBackgroundColor(Red: CInt, Green: CInt, Blue: CInt) -> CInt;
    /// 色コードを取得する
    pub fn dx_GetColor(Red: CInt, Green: CInt, Blue: CInt) -> Color;
    /// 描画先グラフィック領域の指定
    pub fn dx_SetDrawScreen(DrawScreen: CInt) -> CInt;
    /// フリップ関数、画面の裏ページ(普段は表示されていない)の内容を表ページ(普段表示されている)に反映する
    pub fn dx_ScreenFlip() -> CInt;
    /// 画面のフルスクリーンアンチエイリアスモードの設定をする
    pub fn dx_SetFullSceneAntiAliasingMode(Samples: CInt, Quality: CInt) -> CInt;

    // 動画関係関数

    /// 動画ファイルの再生
    //pub fn dx_PlayMovie(FileName:*const CChar,ExRate:CInt,PlayType:CInt) -> CInt;
    /// ムービーグラフィックの動画の再生を開始する
    pub fn dx_PlayMovieToGraph(GraphHandle: CInt) -> CInt;
    /// ムービーグラフィックの動画再生を一時停止する
    pub fn dx_PauseMovieToGraph(GraphHandle: CInt) -> CInt;
    /// ムービーグラフィックの動画の再生位置を変更する
    pub fn dx_SeekMovieToGraph(GraphHandle: CInt, Time: CInt) -> CInt;
    /// ムービーグラフィックの動画の再生位置を得る
    pub fn dx_TellMovieToGraph(GraphHandle: CInt) -> CInt;
    /// ムービーグラフィックの動画の再生状態を得る
    pub fn dx_GetMovieStateToGraph(GraphHandle: CInt) -> CInt;

    // マスク関係関数

    /// マスク画面を作成する
    pub fn dx_CreateMaskScreen() -> CInt;
    /// マスク画面を削除する
    pub fn dx_DeleteMaskScreen() -> CInt;
    /// マスクデータを画像ファイル(BMP.JPEG.PNG)から構築する
    //pub fn dx_LoadMask(FileName: *const CChar) -> CInt;
    /// マスクデータを画像ファイル(BMP.JPEG.PNG)から分割構築する
    //pub fn dx_LoadDivMask() -> CInt;
    /// マスクデータをマスク画面に描画する
    pub fn dx_DrawMask(x: CInt, y: CInt, MaskHandle: CInt, TransMode: CInt) -> CInt;
    /// 指定のマスク画面領域を指定のマスクデータをタイル上に並べて埋める
    pub fn dx_DrawFillMask(x1: CInt, y1: CInt, x2: CInt, y2: CInt, MaskHandle: CInt) -> CInt;
    /// マスクデータを削除
    pub fn dx_DeleteMask(MaskHandle: CInt) -> CInt;
    /// マスクデータを初期化する
    pub fn dx_InitMask() -> CInt;
    /// マスク画面を指定の色で塗りつぶす
    pub fn dx_FillMaskScreen(Flag: CInt) -> CInt;
    /// マスク画面の有効の有無を変更
    pub fn dx_SetUseMaskScreenFlag(ValidFlag: CInt) -> CInt;
    /// 空のマスクデータの作成
    pub fn dx_MakeMask(Width: CInt, Height: CInt) -> CInt;
    /// マスクデータの大きさを得る
    //pub fn dx_GetMaskSize(WidthBuf:*mut CInt,HeightBuf:*mut CInt,MaskHandle:CInt) -> CInt;
    /// マスクのデータをマスクデータ領域に転送する
    pub fn dx_SetDataToMask(
        Width: CInt,
        Height: CInt,
        MaskData: *mut c_void,
        MaskHandle: CInt,
    ) -> CInt;
    /// マスクのデータをマスク画面に直接描画する
    pub fn dx_DrawMaskToDirectData(
        x: CInt,
        y: CInt,
        Width: CInt,
        Height: CInt,
        MaskData: *mut c_void,
        TransMode: CInt,
    ) -> CInt;
    /// マスクのデータをタイル上に並べた形で直接マスク画面全体に描画する
    pub fn dx_DrawFillMaskToDirectData(
        x1: CInt,
        y1: CInt,
        x2: CInt,
        y2: CInt,
        Width: CInt,
        Height: CInt,
        MaskData: *mut c_void,
    ) -> CInt;

    // 入力関係の関数

    /// ジョイパッドが接続されている数を取得する
    pub fn dx_GetJoypadNum() -> CInt;
    /// ジョイパッドの入力状態を得る
    pub fn dx_GetJoypadInputState(InputType: CInt) -> CInt;
    /// ジョイパッドのアナログ的なレバー入力情報を得る
    pub fn dx_GetJoypadAnalogInput(XBuf: *mut CInt, YBuf: *mut CInt, InputType: CInt) -> CInt;
    /// ジョイパッドのDirectInputから取得できる情報を得る
    //pub fn dx_GetJoypadDirectInputState() -> CInt;
    /// ジョイパッドのXInputから取得できる情報を得る
    //pub fn dx_GetJoypadXInputState() -> CInt;
    /// ジョイパッドの方向入力の無効範囲を設定する
    //pub fn dx_SetJoypadDeadZone() -> CInt;
    /// ジョイパッドの振動を開始する
    pub fn dx_StartJoypadVibration(
        InputType: CInt,
        Power: CInt,
        Time: CInt,
        EffectIndex: CInt,
    ) -> CInt;
    /// ジョイパッドの振動を停止する
    //pub fn dx_StopJoypadVibration() -> CInt;

    /// マウスカーソルの表示設定フラグのセット
    pub fn dx_SetMouseDispFlag(DispFlag: CInt) -> CInt;
    /// マウスカーソルの位置を取得する
    //pub fn dx_GetMousePoint(XBuf: *mut CInt, YBuf: *mut CInt) -> CInt;
    /// マウスカーソルの位置をセットする
    pub fn dx_SetMousePoint(PointX: CInt, PointY: CInt) -> CInt;
    /// マウスのボタンの状態を得る
    pub fn dx_GetMouseInput() -> CInt;
    /// マウスのボタンが押されたり離されたりした履歴を取得する
    //pub fn dx_GetMouseInputLog2(Button:*mut CInt,ClickX:*mut CInt,ClickY:*mut CInt,LogType:*mut CInt, LogDelete:CInt ) -> CInt;
    /// マウスホイールの回転量を得る
    pub fn dx_GetMouseWheelRotVol() -> CInt;

    /// タッチされている箇所の数を取得する
    pub fn dx_GetTouchInputNum() -> CInt;
    /// タッチされている箇所の情報を取得する
    //pub fn dx_GetTouchInput(InputNo:CInt,PositionX:*mut CInt,PositionY:*mut CInt,ID:*mut CInt,Device:*mut CInt) -> CInt;

    /// すべてのキーの押下状態を取得
    pub fn dx_CheckHitKeyAll(check_type: CInt) -> CInt;
    /// 特定キーの入力状態を得る
    pub fn dx_CheckHitKey(key_code: CInt) -> CInt;
    /// キーボードのすべてのキーの押下状態を取得する
    pub fn dx_GetHitKeyStateAll(key_state_buf: *mut CChar) -> CInt;

    /// 文字入力バッファに溜まった文字データから１文字取得する
    pub fn dx_GetInputChar(DeleteFlag: CInt) -> CInt;
    /// 文字入力バッファに溜まった文字データから１文字取得する、バッファになにも文字コードがない場合はキーが押されるまで待つ
    pub fn dx_GetInputCharWait(DeleteFlag: CInt) -> CInt;
    /// 文字入力バッファをクリアする
    pub fn dx_ClearInputCharBuf() -> CInt;

    /// キーボードによる文字列の入力
    pub fn dx_KeyInputString(
        x: CInt,
        y: CInt,
        CharMaxLength: CInt,
        StrBuffer: *mut CChar,
        CancelValidFlag: CInt,
    ) -> CInt;
    /// キーボードによる半角文字列のみの入力
    pub fn dx_KeyInputSingleCharString(
        x: CInt,
        y: CInt,
        CharMaxLength: CInt,
        StrBuffer: *mut CChar,
        CancelValidFlag: CInt,
    ) -> CInt;
    /// キーボードによる数値の入力
    pub fn dx_KeyInputNumber(
        x: CInt,
        y: CInt,
        MaxNum: CInt,
        MinNum: CInt,
        CancelValidFlag: CInt,
    ) -> CInt;
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
    ) -> CInt;
    /// 新しいキー入力データの作成
    pub fn dx_MakeKeyInput(
        MaxStrLength: CInt,
        CancelValidFlag: CInt,
        SingleCharOnlyFlag: CInt,
        NumCharOnlyFlag: CInt,
    ) -> CInt;
    /// キー入力データの削除
    pub fn dx_DeleteKeyInput(InputHandle: CInt) -> CInt;
    /// すべてのキー入力データの削除
    pub fn dx_InitKeyInput() -> CInt;
    /// 指定のキー入力をアクティブにする
    pub fn dx_SetActiveKeyInput(InputHandle: CInt) -> CInt;
    /// 入力が終了しているか取得する
    pub fn dx_CheckKeyInput(InputHandle: CInt) -> CInt;
    /// キー入力中データの描画
    pub fn dx_DrawKeyInputString(x: CInt, y: CInt, InputHandle: CInt) -> CInt;
    /// 入力モード文字列を描画する
    pub fn dx_DrawKeyInputModeString(x: CInt, y: CInt) -> CInt;
    /// キー入力データに指定の文字列をセットする
    //pub fn dx_SetKeyInputString() -> CInt;
    /// キー入力データに指定の数値を文字に置き換えてセットする
    pub fn dx_SetKeyInputNumber(Number: CInt, InputHandle: CInt) -> CInt;
    /// 入力データの文字列を取得する
    //pub fn dx_GetKeyInputString() -> CInt;
    /// 入力データの文字列を数値として取得する
    pub fn dx_GetKeyInputNumber(InputHandle: CInt) -> CInt;

    // 音利用関数

    /// 音ファイルを再生する
    //pub fn dx_PlaySoundFile(FileName:&str,PlayType:CInt) -> CInt;
    /// 音ファイルが再生中か調べる
    pub fn dx_CheckSoundFile() -> CInt;
    /// 音ファイルの再生を止める
    pub fn dx_StopSoundFile() -> CInt;
    /// 音ファイルをメモリに読みこむ
    //pub fn dx_LoadSoundMem() -> CInt;
    /// メモリに読みこんだ音データを再生する
    pub fn dx_PlaySoundMem(SoundHandle: CInt, PlayType: CInt, TopPositionFlag: CInt) -> CInt;
    /// メモリに読みこんだ音データが再生中か調べる
    pub fn dx_CheckSoundMem(SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの再生を止める
    pub fn dx_StopSoundMem(SoundHandle: CInt) -> CInt;
    /// メモリに読みこんだ音データを削除する
    pub fn dx_DeleteSoundMem(SoundHandle: CInt) -> CInt;
    /// メモリに読みこんだ音データをすべて消去する
    pub fn dx_InitSoundMem() -> CInt;
    /// メモリに読みこんだ音データの再生にパンを設定する
    pub fn dx_ChangePanSoundMem(PanPal: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読みこんだ音データの再生にボリュームを設定する
    pub fn dx_ChangeVolumeSoundMem(VolumePal: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読みこんだ音データの次の再生にのみ使用するパンを設定する
    pub fn dx_ChangeNextPlayPanSoundMem(PanPal: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読みこんだ音データの次の再生にのみ使用するボリュームを設定する
    pub fn dx_ChangeNextPlayVolumeSoundMem(VolumePal: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの再生周波数を設定する
    pub fn dx_SetFrequencySoundMem(FrequencyPal: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データのループ位置を設定する
    pub fn dx_SetLoopPosSoundMem(LoopTime: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データのループ位置を設定する(サンプル位置指定)
    pub fn dx_SetLoopSamplePosSoundMem(LoopSamplePosition: CInt, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの再生位置をサンプル単位で変更する
    pub fn dx_SetCurrentPositionSoundMem(SamplePosition: CInt, SoundHandle: CInt) -> CInt;
    /// 既にメモリに読み込んである音データを使用するサウンドハンドルを新たに作成する( 非ストリームサウンドのみ )
    pub fn dx_DuplicateSoundMem(SrcSoundHandle: CInt) -> CInt;
    /// 作成するメモリに読み込んだ音データのピッチ( 音の長さを変えずに音程を変更する )レートを設定する
    pub fn dx_SetCreateSoundPitchRate(Cents: CFloat) -> CInt;
    /// 作成するメモリに読み込んだ音データのタイムストレッチ( 音程を変えずに音の長さを変更する )レートを設定する
    pub fn dx_SetCreateSoundTimeStretchRate(Rate: CFloat) -> CInt;
    /// メモリに読み込んだ音データの３Ｄサウンド用の再生位置を設定する
    pub fn dx_Set3DPositionSoundMem(Position: VECTOR, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの３Ｄサウンド用の音が聞こえる距離を設定する
    pub fn dx_Set3DRadiusSoundMem(Radius: CFloat, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの３Ｄサウンド用の移動速度を設定する
    pub fn dx_Set3DVelocitySoundMem(Velocity: VECTOR, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の再生位置を設定する
    pub fn dx_SetNextPlay3DPositionSoundMem(Position: VECTOR, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の音が聞こえる距離を設定する
    pub fn dx_SetNextPlay3DRadiusSoundMem(Radius: CFloat, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの次の再生のみに使用する３Ｄサウンド用の移動速度を設定する
    pub fn dx_SetNextPlay3DVelocitySoundMem(Velocity: VECTOR, SoundHandle: CInt) -> CInt;
    /// メモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータを設定する
    pub fn dx_Set3DReverbParamSoundMem(
        SOUND3D_REVERB_PARAM: *const XAUDIO2FX_REVERB_PARAMETERS,
        SoundHandle: CInt,
    ) -> CInt;
    /// メモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータをプリセットを使用して設定する
    //pub fn dx_Set3DPresetReverbParamSoundMem() -> CInt;
    /// ３Ｄサウンド用のプリセットのリバーブエフェクトパラメータを取得する
    //pub fn dx_Get3DPresetReverbParamSoundMem() -> CInt;
    /// 全てのメモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータを設定する
    //pub fn dx_Set3DReverbParamSoundMemAll() -> CInt;
    /// 全てのメモリに読み込んだ音データの３Ｄサウンド用のリバーブエフェクトパラメータをプリセットを使用して設定する
    //pub fn dx_Set3DPresetReverbParamSoundMemAll() -> CInt;
    /// 次に作成するメモリに読み込む音データを３Ｄサウンド用にするかどうかを設定する
    //pub fn dx_SetCreate3DSoundFlag() -> CInt;
    /// サウンドの再生にXAudio2を使用するかどうかを設定する
    //pub fn dx_SetEnableXAudioFlag() -> CInt;
    /// ３Ｄ空間の１メートルに相当する距離を設定する
    //pub fn dx_Set3DSoundOneMetre() -> CInt;
    /// ３Ｄサウンドのリスナーの位置とリスナーの前方位置を設定する
    //pub fn dx_Set3DSoundListenerPosAndFrontPos_UpVecY() -> CInt;
    /// ３Ｄサウンドのリスナーの位置とリスナーの前方位置とリスナーの上方向を設定する
    //pub fn dx_Set3DSoundListenerPosAndFrontPosAndUpVec() -> CInt;
    /// ３Ｄサウンドのリスナーの移動速度を設定する
    //pub fn dx_Set3DSoundListenerVelocity() -> CInt;

    // 音楽再生関数
    //pub fn dx_LoadMusicMem(FileName: *const CChar)->CInt;
    pub fn dx_PlayMusicMem(MusicHandle:CInt,PlayType: CInt,TopPositionFlag: CInt)->CInt;
    pub fn dx_DeleteMusicMem(MusicHandle:CInt)->i32;
    /// ＭＩＤＩ又はＭＰ３ファイルを演奏(再生)する
    //pub fn dx_PlayMusic(FileName:*const CChar ,PlayType:CInt) -> CInt;
    /// ＭＩＤＩ又はＭＰ３ファイルが演奏(再生)中かの情報を取得する
    pub fn dx_CheckMusic() -> CInt;
    /// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)停止
    pub fn dx_StopMusic() -> CInt;
    /// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)の音量を設定する
    pub fn dx_SetVolumeMusic(Volume: CInt) -> CInt;

    // ウエイト関係の関数

    /// 指定の時間だけ処理をとめる
    pub fn dx_WaitTimer(WaitTime: CInt) -> CInt;
    /// ディスプレイの垂直同期信号を指定回数待つ
    //pub fn dx_WaitVSync() -> CInt;
    /// キーの入力待ち
    pub fn dx_WaitKey() -> CInt;

    // 時間関係の関数

    /// ミリ秒単位の精度を持つカウンタの現在値を得る
    pub fn dx_GetNowCount() -> CInt;
    /// GetNowCountの高精度バージョン
    pub fn dx_GetNowHiPerformanceCount() -> i64;
    /// 現在時刻を取得する
    //pub fn dx_GetDateTime() -> CInt;

    // 乱数取得関数

    /// 乱数を取得する
    pub fn dx_GetRand(rand_max: CInt) -> CInt;
    /// 乱数の初期値を設定する
    pub fn dx_SRand(seed: CInt) -> CInt;

    // ウインドウモード関係

    /// ウインドウモード・フルスクリーンモードの変更を行う
    pub fn dx_ChangeWindowMode(flag: CInt) -> CInt;
    /// ウインドウのタイトルを変更する
    //pub fn dx_SetMainWindowText(WindowText: *const CChar) -> CInt;

    /// ウインドウのアイコンを変更する
    pub fn dx_SetWindowIconID(ID: CInt) -> CInt;
    /// ウインドウモードの時にウインドウのサイズを自由に変更出来るようにするかどうかを設定する
    pub fn dx_SetWindowSizeChangeEnableFlag(Flag: CInt) -> CInt;
    /// ウインドウモードの時のウインドウの大きさと描画画面の大きさの比率を設定する
    pub fn dx_SetWindowSizeExtendRate(ExRateX: f64, ExRateY: f64) -> CInt;
    // 通信関係

    /// 他のマシンに接続する
    pub fn dx_ConnectNetWork(IPData: IPDATA, Port: CInt) -> CInt;
    /// 接続を終了する
    pub fn dx_CloseNetWork(NetHandle: CInt) -> CInt;
    /// 接続を受け付けられる状態にする
    pub fn dx_PreparationListenNetWork(Port: CInt) -> CInt;
    /// 接続を受け付けている状態を解除する
    pub fn dx_StopListenNetWork() -> CInt;
    /// データを送信する
    pub fn dx_NetWorkSend(NetHandle: CInt, Buffer: *mut c_void, Length: CInt) -> CInt;
    /// 受信データ一時記憶バッファに溜まっているデータの量を得る
    //pub fn dx_GetNetWorkDataLength() -> CInt;
    /// 未送信のデータの量を得る
    //pub fn dx_GetNetWorkSendDataLength() -> CInt;
    /// 受信データ一時記憶バッファに溜まっているデータを取得する
    //pub fn dx_NetWorkRecv() -> CInt;
    /// 受信したデータを読み込む、読み込んだデータはバッファから削除されない
    //pub fn dx_NetWorkRecvToPeek() -> CInt;
    /// 新たに確立した接続を示すネットワークハンドルを得る
    //pub fn dx_GetNewAcceptNetWork() -> CInt;
    /// 新たに破棄された接続を示すネットワークハンドルを得る
    //pub fn dx_GetLostNetWork() -> CInt;
    /// 接続状態を取得する
    //pub fn dx_GetNetWorkAcceptState() -> CInt;
    /// 接続先のＩＰアドレスを得る
    //pub fn dx_GetNetWorkIP() -> CInt;
    /// ＵＤＰを使用して通信するためのソケットを作成する
    //pub fn dx_MakeUDPSocket() -> CInt;
    /// ＵＤＰを使用して通信するためのソケットを削除する
    //pub fn dx_DeleteUDPSocket() -> CInt;
    /// ＵＤＰを使用して他のマシンにデータを送信する
    //pub fn dx_NetWorkSendUDP() -> CInt;
    /// ＵＤＰを使用して他のマシンからのデータを受信する
    //pub fn dx_NetWorkRecvUDP() -> CInt;
    /// ＵＤＰを使用した他のマシンから受信データがあるかどうかを取得する
    //pub fn dx_CheckNetWorkRecvUDP() -> CInt;

    // ファイル読み込み関係

    /// ファイルを開く
    //pub fn dx_FileRead_open(FilePath: *mut CChar, ASync: CInt) -> CInt;
    /// ファイルのサイズを得る
    //pub fn dx_FileRead_size() -> CInt;
    /// ファイルを閉じる
    pub fn dx_FileRead_close(FileHandle: CInt) -> CInt;
    /// ファイルポインタの位置を得る
    //pub fn dx_FileRead_tell() -> CInt;
    /// ファイルポインタの位置を変更する
    //pub fn dx_FileRead_seek() -> CInt;
    /// ファイルからデータを読み込む
    pub fn dx_FileRead_read(Buffer: *mut c_void, ReadSize: CInt, FileHandle: CInt) -> CInt;
    /// ファイルの終端かどうかを調べる
    pub fn dx_FileRead_eof(FileHandle: CInt) -> CInt;
    /// ファイルから一行読み出す
    //pub fn dx_FileRead_gets(Buffer:*mut CChar,Num:CInt,FileHandle:CInt) -> CInt;
    /// ファイルから一文字読み出す
    //pub fn dx_FileRead_getc() -> CInt;
    /// ファイルから書式付きデータを読み出す
    //pub fn dx_FileRead_scanf() -> CInt;

    // ドット単位で画像にアクセスしたい関係

    /// ＣＰＵで扱うイメージの読み込み
    //pub fn dx_LoadSoftImage(FileName:*const CChar) -> CInt;
    /// ＣＰＵで扱うイメージの読み込み( RGBA8 カラーに変換 )
    //pub fn dx_LoadARGB8ColorSoftImage(FileName:*const CChar) -> CInt;
    /// ＣＰＵで扱うイメージの読み込み( XGBA8 カラーに変換 )
    //puFileName:*const CCharb fn dx_LoadXRGB8ColorSoftImage(FileName:*const CChar) -> CInt;
    /// ＣＰＵで扱うイメージのメモリからの読み込み
    //pub fn dx_LoadSoftImageToMem() -> CInt;
    /// ＣＰＵで扱うイメージのメモリからの読み込み( RGBA8 カラーに変換 )
    //pub fn dx_LoadARGB8ColorSoftImageToMem() -> CInt;
    /// ＣＰＵで扱うイメージのメモリからの読み込み( XGBA8 カラーに変換 )
    //pub fn dx_LoadXRGB8ColorSoftImageToMem() -> CInt;
    /// ＣＰＵで扱うイメージの作成( RGBA8 カラー )
    //pub fn dx_MakeARGB8ColorSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの作成( XRGB8 カラー )
    //pub fn dx_MakeXRGB8ColorSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの作成( パレット２５６色 カラー )
    //pub fn dx_MakePAL8ColorSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの解放
    //pub fn dx_DeleteSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージを全て解放
    //pub fn dx_InitSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージのサイズを取得する
    //pub fn dx_GetSoftImageSize() -> CInt;
    /// ＣＰＵで扱うイメージを指定色で塗りつぶす(各色要素は０～２５５)
    //pub fn dx_FillSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージのパレットをセットする(各色要素は０～２５５)
    //pub fn dx_SetPaletteSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージのパレットを取得する(各色要素は０～２５５)
    //pub fn dx_GetPaletteSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの指定座標にドットを描画する(パレット画像用、有効値は０～２５５)
    //pub fn dx_DrawPixelPalCodeSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの指定座標の色コードを取得する(パレット画像用、戻り値は０～２５５)
    //pub fn dx_GetPixelPalCodeSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの指定座標にドットを描画する(各色要素は０～２５５)
    //pub fn dx_DrawPixelSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージの指定座標の色を取得する(各色要素は０～２５５)
    //pub fn dx_GetPixelSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージを別のイメージ上に転送する
    //pub fn dx_BltSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージを画面に描画する
    //pub fn dx_DrawSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージからグラフィックハンドルを作成する
    //pub fn dx_CreateGraphFromSoftImage() -> CInt;
    /// ＣＰＵで扱うイメージから分割グラフィックハンドルを作成する
    //pub fn dx_CreateDivGraphFromSoftImage() -> CInt;

    // 非同期読み込み関係

    /// 非同期読み込みを行うかどうかを設定する
    pub fn dx_SetUseASyncLoadFlag(Flag: CInt) -> CInt;
    /// ハンドルが非同期読み込み中かどうかを取得する
    pub fn dx_CheckHandleASyncLoad(Handle: CInt) -> CInt;
    /// 行っている非同期読み込みの数を取得する
    pub fn dx_GetASyncLoadNum() -> CInt;

    // 文字関係関数

    /// 文字列の引数の文字コードを設定する
    pub fn dx_SetUseCharCodeFormat(char_code_format: CInt) -> CInt;
    /// 文字列の先頭の文字のバイト数を取得する
    //pub fn dx_GetCharBytes() -> CInt;

    // ツールバー    
    pub fn dx_SetDisplayMenuFlag(Flag: CInt) -> CInt; // メニューを表示するかどうかをセットする

    pub fn dx_GetDisplayMenuFlag() -> CInt; // メニューを表示しているかどうかを取得する

    pub fn dx_GetUseMenuFlag() -> CInt; // メニューを使用しているかどうかを得る

    pub fn dx_SetAutoMenuDisplayFlag(Flag: CInt) -> CInt; // フルスクリーン時にメニューを自動で表示したり非表示にしたりするかどうかのフラグをセットする

    pub fn dx_AddToolBarButton(Type: CInt, State: CInt, ImageIndex: CInt, ID: CInt) -> CInt; // ツールバーにボタンを追加する
    pub fn dx_AddToolBarSep() -> CInt; // ツールバーに隙間を追加する

    pub fn dx_GetToolBarButtonState(ID: CInt) -> CInt; // ツールバーのボタンの状態を取得する

    pub fn dx_SetToolBarButtonState(ID: CInt, State: CInt) -> CInt; // ツールバーのボタンの状態を設定するpub fn dx_DeleteAllToolBarButton()->CInt ;// ツールバーのボタンを全て削除する

    pub fn dx_AddMenuItem(
        AddType: CInt,
        ItemName: *const CChar,
        ItemID: CInt,
        SeparatorFlag: CInt,
        NewItemName: *const CChar,
        NewItemID: CInt,
    ) -> CInt; // メニューに項目を追加する

    pub fn dx_DeleteMenuItem(ItemName: *const CChar, ItemID: CInt) -> CInt; // メニューから選択項目を削除する

    pub fn dx_CheckMenuItemSelect(ItemName: *const CChar, ItemID: CInt) -> CInt; // メニューが選択されたかどうかを取得する

    pub fn dx_SetMenuItemEnable(ItemName: *const CChar, ItemID: CInt, EnableFlag: CInt) -> CInt; // メニューの項目を選択出来るかどうかを設定する
    pub fn dx_SetMenuItemMark(ItemName: *const CChar, ItemID: CInt, Mark: CInt) -> CInt; // メニューの項目にチェックマークやラジオボタンを表示するかどうかを設定する
    pub fn dx_SetUseMenuFlag(Flag: CInt) -> CInt; // メニューを有効にするかどうかを設定する

    pub fn dx_CheckMenuItemSelectAll() -> CInt; // メニューの項目がどれか選択されたかどうかを取得する

    pub fn dx_AddMenuItem_Name(ParentItemName: *const CChar, NewItemName: *const CChar) -> CInt; // メニューに選択項目を追加する

    pub fn dx_AddMenuLine_Name(ParentItemName: *const CChar) -> CInt; // メニューのリストに区切り線を追加する

    pub fn dx_InsertMenuItem_Name(ItemName: *const CChar, NewItemName: *const CChar) -> CInt; // 指定の項目と、指定の項目の一つ上の項目との間に新しい項目を追加する

    pub fn dx_InsertMenuLine_Name(ItemName: *const CChar) -> CInt; // 指定の項目と、指定の項目の一つ上の項目との間に区切り線を追加する

    pub fn dx_DeleteMenuItem_Name(ItemName: *const CChar) -> CInt; // メニューから選択項目を削除する
    pub fn dx_CheckMenuItemSelect_Name(ItemName: *const CChar) -> CInt; // メニューが選択されたかどうかを取得する

    pub fn dx_SetMenuItemEnable_Name(ItemName: *const CChar, EnableFlag: CInt) -> CInt; // メニューの項目を選択出来るかどうかを設定する

    pub fn dx_SetMenuItemMark_Name(ItemName: *const CChar, Mark: CInt) -> CInt; // メニューの項目にチェックマークやラジオボタンを表示するかどうかを設定する

    pub fn dx_AddMenuItem_ID(ParentItemID: CInt, NewItemName: *const CChar, NewItemID: CInt) -> CInt; // メニューに選択項目を追加する

    pub fn dx_AddMenuLine_ID(ParentItemID: CInt) -> CInt; // メニューのリストに区切り線を追加する

    pub fn dx_InsertMenuItem_ID(ItemID: CInt, NewItemID: CInt) -> CInt; // 指定の項目と、指定の項目の一つ上の項目との間に新しい項目を追加する

    pub fn dx_InsertMenuLine_ID(ItemID: CInt, NewItemID: CInt) -> CInt; // 指定の項目と、指定の項目の一つ上の項目との間に区切り線を追加する

    pub fn dx_DeleteMenuItem_ID(ItemID: CInt) -> CInt; // メニューから選択項目を削除する

    pub fn dx_CheckMenuItemSelect_ID(ItemID: CInt) -> CInt; // メニューが選択されたかどうかを取得する

    pub fn dx_SetMenuItemEnable_ID(ItemID: CInt, EnableFlag: CInt) -> CInt; // メニューの項目を選択出来るかどうかを設定する

    pub fn dx_SetMenuItemMark_ID(ItemID: CInt, Mark: CInt) -> CInt; // メニューの項目にチェックマークやラジオボタンを表示するかどうかを設定する

    pub fn dx_DeleteMenuItemAll() -> CInt; // メニューの全ての選択項目を削除する

    pub fn dx_ClearMenuItemSelect() -> CInt; // メニューが選択されたかどうかの情報をリセット

    pub fn dx_GetMenuItemID(ItemName: CInt) -> CInt; // メニューの項目名から項目識別番号を取得する

    pub fn dx_GetMenuItemName(ItemID: CInt, NameBuffer: *mut CChar) -> CInt; // メニューの項目識別番号から項目名を取得する

    pub fn dx_LoadMenuResource(MenuResourceID: CInt) -> CInt; // メニューをリソースから読み込む

    // クリップボード

    // クリップボードに格納されているテキストデータを読み出す
    pub fn dx_GetClipboardText(DestBuffer: *mut CChar) -> CInt;
    pub fn dx_SetClipboardText(Text: *const CChar) -> CInt; // クリップボードにテキストデータを格納する

    // ログ関係
    pub fn dx_ErrorLogAdd(ErrorStr: *const CChar) -> CInt; // ログファイル( Log.txt ) に文字列を出力する

    // マイナー関数
    pub fn dx_SetWindowUserCloseEnableFlag(Flag:CInt)->CInt;

    pub fn dx_GetWindowUserCloseFlag(StateResetFlag:CInt)->CInt;
    pub fn dx_SetUseBackBufferTransColorFlag(Flag: CInt) -> CInt;
    /// ウインドウがアクティブではない状態でも処理を続行するか、フラグをセットする
    pub fn dx_SetAlwaysRunFlag(Flag: CInt) -> CInt;
    /// ログ出力を行うか否かのセット
    pub fn dx_SetOutApplicationLogValidFlag(Flag: CInt) -> CInt;
    /// ＤＸアーカイブファイルの読み込み機能を使うかどうかを設定する
    pub fn dx_SetUseDXArchiveFlag(Flag: CInt) -> CInt;
    /// 検索するＤＸアーカイブファイルの拡張子を変更する
    //pub fn dx_SetDXArchiveExtension(Extension: *const CChar) -> CInt;
    /// ＤＸアーカイブファイルの鍵文字列を設定する
    //pub fn dx_SetDXArchiveKeyString() -> CInt;
    /// ６４０ｘ４８０の画面で３２０ｘ２４０の画面解像度にするかどうかのフラグをセットする
    pub fn dx_SetEmulation320x240(Flag: CInt) -> CInt;
    /// ３Ｄ機能を使うか、のフラグをセット
    pub fn dx_SetUse3DFlag(Flag: CInt) -> CInt;
    /// ScreenFlip関数実行時にＣＲＴの垂直同期信号待ちをするかのフラグセット
    pub fn dx_SetWaitVSyncFlag(Flag: CInt) -> CInt;
    // 　描画の際のブレンドモードを詳細に設定する
    pub fn SetDrawCustomBlendMode(
        BlendEnable: CInt,
        SrcBlendRGB: CInt,
        DestBlendRGB: CInt,
        BlendOpRGB: CInt,
        SrcBlendA: CInt,
        DestBlendA: CInt,
        BlendOpA: CInt,
        BlendParam: CInt,
    );
    /// 必要ならグラフィックの分割を行うか否かを設定する
    pub fn dx_SetUseDivGraphFlag(Flag: CInt) -> CInt;
    /// フォーカスが他のソフトに移っているときにバックグラウンドに表示するグラフィックを設定する
    //pub fn dx_LoadPauseGraph(FileName: *const CChar) -> CInt;
    /// 裏画面の内容を表画面にコピーする
    pub fn dx_ScreenCopy() -> CInt;
    /// 画面の色ビット数を得る
    pub fn dx_GetColorBitDepth() -> CInt;
    /// 現在描画対象になっている画面をＢＭＰ形式で保存する
    //pub fn dx_SaveDrawScreen(x1: CInt, y1: CInt, x2: CInt, y2: CInt, FileName: *const CChar) -> CInt;
    /// 使用可能なフォントの名前を列挙する
    //pub fn dx_EnumFontName() -> CInt;
    /// 文字列を縦に描画する
    //pub fn dx_DrawVString() -> CInt;
    /// フォントハンドルを使用して文字列を縦に描画する
    //pub fn dx_DrawVStringToHandle() -> CInt;
    /// メモリ上の画像ファイルイメージからグラフィックハンドルを作成する
    //pub fn dx_CreateGraphFromMem() -> CInt;
    /// メモリ上の画像ファイルイメージから既存のグラフィックハンドルにデータを転送する
    //pub fn dx_ReCreateGraphFromMem() -> CInt;
    /// 画像ファイルから作成したグラフィックハンドルに再度画像ファイルから画像を読み込む
    //pub fn dx_ReloadFileGraphAll() -> CInt;
    /// グラフィックハンドル復元関数を登録する
    //pub fn dx_SetRestoreGraphCallback() -> CInt;
    /// 作成する音データの再生形式を設定する
    //pub fn dx_SetCreateSoundDataType() -> CInt;
    /// メモリ上の音ファイルイメージからサウンドハンドルを作成する
    //pub fn dx_LoadSoundMemByMemImage() -> CInt;
    /// ＭＩＤＩの演奏形態をセットする
    pub fn dx_SelectMidiMode(mode: CInt) -> CInt;
    /// カーソルを点滅させるかどうかを設定する
    pub fn dx_SetKeyInputCursorBrinkFlag(Flag: CInt) -> CInt;
    /// ドラッグアンドドロップを有効化するかどうか設定する。
    pub fn dx_SetDragFileValidFlag(Flag: CInt) -> CInt;
    /// ドラッグアンドドロップされたファイルをひとつ読み出す。
    pub fn dx_GetDragFilePath(FilePathBuffer: *mut CChar) -> CInt;
    /// ドラッグアンドドロップされたファイルの数を取得する。
    pub fn dx_GetDragFileNum() -> CInt;
    // ウィンドウの見た目を変える
    pub fn dx_SetWindowStyleMode(Mode: CInt) -> CInt;
}

extern "cdecl" {
    // pub unsafe fn  dx_printfDx(fotmat:*const CChar, args: ...) -> CInt;
}
/*wrapped function*/
mod hidden {
    use crate::dxlib_common::*;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::os::raw::*;
    use std::vec::Vec;
    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "stdcall" {
        pub fn dx_ClearDrawScreen(ClearRect: *mut RECT) -> CInt;
        pub fn dx_LoadGraph(FileName: *const CChar, NotUse3DFlag: CInt) -> CInt;
        pub fn dx_PlaySoundFile(FileName: *const CChar, PlayType: CInt) -> CInt;
        pub fn dx_LoadSoundMem(FileName: *const CChar) -> CInt;
        pub fn dx_DrawString(x: CInt, y: CInt, String: *const CChar, Color: Color) -> CInt;
        pub fn dx_LoadMusicMem(FileName: *const CChar)->CInt;
        pub fn dx_MV1LoadModel(FileName: *const CChar) -> CInt;
        pub fn dx_ChangeFont(FileName: *const CChar) -> CInt;
        pub fn dx_CreateFontToHandle(
            FontName: *const CChar,
            Size: CInt,
            Thick: CInt,
            FontType: CInt,
        ) -> CInt;
        pub fn dx_DrawStringToHandle(
            x: CInt,
            y: CInt,
            String: *const CChar,
            Color: Color,
            FontHandle: CInt,
        ) -> CInt;
        pub fn dx_LoadFontDataToHandle(FileName: *const CChar, EdgeSize: CInt) -> CInt;
        pub fn dx_SetDXArchiveExtension(Extension: *const CChar) -> CInt;

        pub fn dx_SetDXArchiveKeyString(KeyString: *const CChar) -> CInt;
        pub fn dx_SaveDrawScreen(
            x1: CInt,
            y1: CInt,
            x2: CInt,
            y2: CInt,
            FileName: *const CChar,
        ) -> CInt;
        pub fn dx_SetMainWindowText(WindowText: *const CChar) -> CInt;

        pub fn dx_FileRead_gets(Buffer: *mut CChar, Num: CInt, FileHandle: CInt) -> CInt;
        pub fn dx_FileRead_open(FilePath: *const CChar, ASync: CInt) -> CInt;
        pub fn dx_LoadDivGraph(
            FileName: *const CChar,
            AllNum: CInt,
            XNum: CInt,
            YNum: CInt,
            XSize: CInt,
            YSize: CInt,
            HandleBuf: *mut CInt,
        ) -> CInt;
        pub fn dx_LoadGraphScreen(
            x: CInt,
            y: CInt,
            GraphName: *const CChar,
            TransFlag: CInt,
        ) -> CInt;
        pub fn dx_GetGraphSize(GrHandle: CInt, SizeXBuf: *mut CInt, SizeYBuf: *mut CInt) -> CInt;
        pub fn dx_LoadBlendGraph(FileName: *const CChar) -> CInt;
        pub fn dx_GetDateTime(DateBuf: *mut DATEDATA) -> CInt;
        pub fn dx_GetDrawStringWidth(String: *const CChar, StrLen: CInt) -> CInt;
        pub fn dx_GetDrawStringWidthToHandle(
            String: *const CChar,
            StrLen: CInt,
            FontHandle: CInt,
        ) -> CInt;

        pub fn dx_GetFontStateToHandle(
            FontName: *const CChar,
            Size: *mut CInt,
            Thick: *mut CInt,
            FontHandle: CInt,
        ) -> CInt;

        pub fn dx_GetScreenState(
            SizeX: *mut CInt,
            SizeY: *mut CInt,
            ColorBitDepth: *mut CInt,
        ) -> CInt;
        pub fn dx_PlayMovie(FileName: *const CChar, ExRate: CInt, PlayType: CInt) -> CInt;
        pub fn dx_LoadMask(FileName: *const CChar) -> CInt;
        pub fn dx_LoadDivMask(
            FileName: *const CChar,
            AllNum: CInt,
            XNum: CInt,
            YNum: CInt,
            XSize: CInt,
            YSize: CInt,
            HandleBuf: *mut CInt,
        ) -> CInt;
        pub fn dx_GetMaskSize(WidthBuf: *mut CInt, HeightBuf: *mut CInt, MaskHandle: CInt) -> CInt;
        pub fn dx_GetMousePoint(XBuf: *mut CInt, YBuf: *mut CInt) -> CInt;
        pub fn dx_PlayMusic(FileName: *const CChar, PlayType: CInt) -> CInt;

        pub fn dx_LoadPauseGraph(FileName: *const CChar) -> CInt;
        pub fn dx_LoadSoftImage(FileName: *const CChar) -> CInt;
        pub fn dx_LoadARGB8ColorSoftImage(FileName: *const CChar) -> CInt;
        pub fn dx_LoadXRGB8ColorSoftImage(FileName: *const CChar) -> CInt;

    }

    #[link(name = "DxLib_x64")]
    #[no_mangle]
    extern "cdecl" {
        //pub fn dx_DrawFormatString(x:CInt,y:CInt,ck\olor:\\\);
    }
}

/*wrap function*/
pub fn dx_ClearDrawScreen() -> CInt {
    let mut tmp = RECT {
        left: -1,
        right: -1,
        top: -1,
        bottom: -1,
    };
    unsafe { hidden::dx_ClearDrawScreen(&mut tmp) }
}

pub fn dx_LoadGraph(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadGraph(FileName.to_cstring().as_ptr(), FALSE);
    }
}

pub fn dx_PlaySoundFile(FileName: &str, PlayType: CInt) -> CInt {
    unsafe {
        return hidden::dx_PlaySoundFile(FileName.to_cstring().as_ptr(), PlayType);
    }
}

pub fn dx_LoadSoundMem(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadSoundMem(FileName.to_cstring().as_ptr());
    }
}

pub fn dx_DrawString(x: CInt, y: CInt, String: &str, Color: Color) -> CInt {
    unsafe {
        return hidden::dx_DrawString(x, y, String.to_cstring().as_ptr(), Color);
    }
}
pub fn dx_MV1LoadModel(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_MV1LoadModel(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_ChangeFont(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_ChangeFont(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_CreateFontToHandle(FontName: &str, Size: CInt, Thick: CInt, FontType: CInt) -> CInt {
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
    x: CInt,
    y: CInt,
    String: &str,
    Color: Color,
    FontHandle: CInt,
) -> CInt {
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
pub fn dx_LoadFontDataToHandle(FileName: &str, EdgeSize: CInt) -> CInt {
    unsafe {
        return hidden::dx_LoadFontDataToHandle(FileName.to_cstring().as_ptr(), EdgeSize);
    }
}
pub fn dx_SetDXArchiveExtension(Extension: &str) -> CInt {
    unsafe {
        return hidden::dx_SetDXArchiveExtension(Extension.to_cstring().as_ptr());
    }
}
pub fn dx_SaveDrawScreen(x1: CInt, y1: CInt, x2: CInt, y2: CInt, FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_SaveDrawScreen(x1, y1, x2, y2, FileName.to_cstring().as_ptr());
    }
}
pub fn dx_SetMainWindowText(WindowText: &str) -> CInt {
    unsafe {
        return hidden::dx_SetMainWindowText(WindowText.to_cstring().as_ptr());
    }
}
pub fn dx_LoadDivGraph(
    FileName: &str,
    AllNum: CInt,
    XNum: CInt,
    YNum: CInt,
    XSize: CInt,
    YSize: CInt,
    HandleBuf: *mut CInt,
) -> CInt {
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
pub fn dx_LoadGraphScreen(x: CInt, y: CInt, GraphName: &str, TransFlag: CInt) -> CInt {
    unsafe {
        return hidden::dx_LoadGraphScreen(x, y, GraphName.to_cstring().as_ptr(), TransFlag);
    }
}
pub fn dx_GetGraphSize(GrHandle: CInt, SizeXBuf: &mut CInt, SizeYBuf: &mut CInt) -> CInt {
    unsafe {
        return hidden::dx_GetGraphSize(
            GrHandle,
            &mut *SizeXBuf as *mut CInt,
            &mut *SizeYBuf as *mut CInt,
        );
    }
}
pub fn dx_LoadBlendGraph(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadBlendGraph(FileName.to_cstring().as_ptr());
    }
}

pub fn dx_GetDateTime(DateBuf: &mut DATEDATA) -> CInt {
    unsafe {
        return hidden::dx_GetDateTime(&mut *DateBuf as *mut DATEDATA);
    }
}
pub fn dx_GetDrawStringWidth(String: &str, StrLen: CInt) -> CInt {
    unsafe {
        return hidden::dx_GetDrawStringWidth(String.to_cstring().as_ptr(), StrLen);
    }
}
pub fn dx_GetDrawStringWidthToHandle(String: &str, StrLen: CInt, FontHandle: CInt) -> CInt {
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
    Size: &mut CInt,
    Thick: &mut CInt,
    FontHandle: CInt,
) -> CInt {
    unsafe {
        return hidden::dx_GetFontStateToHandle(
            FontName.to_cstring().as_ptr(),
            &mut *Size as *mut CInt,
            &mut *Thick as *mut CInt,
            FontHandle,
        );
    }
}
pub fn dx_GetScreenState(SizeX: &mut CInt, SizeY: &mut CInt, ColorBitDepth: &mut CInt) -> CInt {
    unsafe {
        return hidden::dx_GetScreenState(
            &mut *SizeX as *mut CInt,
            &mut *SizeY as *mut CInt,
            &mut *ColorBitDepth as *mut CInt,
        );
    }
}

pub fn dx_PlayMovie(FileName: &str, ExRate: CInt, PlayType: CInt) -> CInt {
    unsafe {
        return hidden::dx_PlayMovie(FileName.to_cstring().as_ptr(), ExRate, PlayType);
    }
}
pub fn dx_LoadMask(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadMask(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_LoadDivMask(
    FileName: &str,
    AllNum: CInt,
    XNum: CInt,
    YNum: CInt,
    XSize: CInt,
    YSize: CInt,
    HandleBuf: *mut CInt,
) -> CInt {
    unsafe {
        return hidden::dx_LoadDivMask(
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
pub fn dx_GetMaskSize(WidthBuf: &mut CInt, HeightBuf: &mut CInt, MaskHandle: CInt) -> CInt {
    unsafe {
        return hidden::dx_GetMaskSize(
            &mut *WidthBuf as *mut CInt,
            &mut *HeightBuf as *mut CInt,
            MaskHandle,
        );
    }
}
pub fn dx_GetMousePoint(XBuf: &mut CInt, YBuf: &mut CInt) -> CInt {
    unsafe {
        return hidden::dx_GetMousePoint(&mut *XBuf as *mut CInt, &mut *YBuf as *mut CInt);
    }
}
pub fn dx_PlayMusic(FileName: &str, PlayType: CInt) -> CInt {
    unsafe {
        return hidden::dx_PlayMusic(FileName.to_cstring().as_ptr(), PlayType);
    }
}
pub fn dx_SetDXArchiveKeyString(KeyString: &str) -> CInt {
    unsafe {
        return hidden::dx_SetDXArchiveKeyString(KeyString.to_cstring().as_ptr());
    }
}

pub fn dx_LoadPauseGraph(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadPauseGraph(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_LoadSoftImage(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadSoftImage(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_LoadARGB8ColorSoftImage(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadARGB8ColorSoftImage(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_LoadXRGB8ColorSoftImage(FileName: &str) -> CInt {
    unsafe {
        return hidden::dx_LoadXRGB8ColorSoftImage(FileName.to_cstring().as_ptr());
    }
}
pub fn dx_FileRead_open(FilePath: &str, ASync: CInt) -> CInt {
    unsafe {
        return hidden::dx_FileRead_open(FilePath.to_cstring().as_ptr(), ASync);
    }
}
pub fn dx_LoadMusicMem(FileName: &str)->CInt{
    unsafe{
        return hidden::dx_LoadMusicMem(FileName.to_cstring().as_ptr());
    }
}
