@echo off
REM cargo check 
cargo check
REM Cargo Checkが成功したら
if %errorlevel% equ 0 (
REM git command
git add . && git commit -m Update && git push 
)

