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

## フォークさせてもらった、ライブラリ
elipmoc/rust_dxlib
## License
MIT License

Copyright (c) 2018 らいパン粉

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


## License
MIT
