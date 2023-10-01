@echo off
REM cargo check 
cargo check


REM Cargo Checkが成功したら
if %errorlevel% equ 0 (
git add . && git commit -m Update && git push 
)
REM それ以外は何もしない
else(

)
