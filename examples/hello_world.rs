// cargo-deps: dxlib-ffi = {path = "../"}
extern crate dxlib_ffi;
use dxlib_ffi::dxlib::*;

fn main() {
    draw();
}

fn draw() {
    unsafe {
        //ウィンドウモード変更と初期化と裏画面設定
        dx_ChangeWindowMode(TRUE);
        dx_DxLib_Init();
        dx_SetDrawScreen(DX_SCREEN_BACK);
        let mut color = dx_GetColor(255,255,255);
        let mut x = 0;
        let mut y = 0;
        let mut text:String = "hello world!".to_string();
        // while( 裏画面を表画面に反映, メッセージ処理, 画面クリア )
        while (dx_ScreenFlip() == 0 && dx_ProcessMessage() == 0 && dx_ClearDrawScreen() == 0) {
            dx_DrawString(x,y,&text,color);
        }

        dx_DxLib_End(); // DXライブラリ終了処理
    }
}
