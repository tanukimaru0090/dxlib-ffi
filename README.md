# dxlib-ffi-rs
・DxLibをRust用に呼び出せるようにしたもの
・さらにラップされたものを使いたい場合、このライブラリと一緒に "tanukimaru0090/dxlib-rs"を使ってください。
 また、"tanukimaru0090/dxlib-rs"についてはそのリポジトリ内の "README.md" を参照してください。

#使い方

myproject/Cargo.toml
```toml
[dependencies]
dxlib-ffi = {git = "https://github.com/tanukimaru0090/dxlib-ffi.git"}
```

src/main.rs
```Rust
extern crate dxlib_ffi;
use dxlib_ffi::dxlib;

fn main(){
  unsafe{
      dxlib::dx_DxLib_Init();
      let mut color = dxlib::dx_GetColor(255,255,255);
      while dxlib::dx_ProcesMessage() == 0{
          dxlib::dx_DrawString(0,0,"hello world!",color);
          if dxlib::dx_CheckHitKey(KEY_INPUT_ESCAPE) == TRUE{
               break;
          }
      }
      dxlib::dx_DxLib_End();
  }
}
```

最後に、 "cargo build --release" などでビルドをして、実行ファイルと同じディレクトリに "DxLib_x64.dll" を置くことで実行することができます。
DxLib_x64.dllはDXライブラリの公式サイトのC#版DXライブラリをダウンロードすることで使うことができます。


## License
MIT
