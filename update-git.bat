@echo off
REM cargo check 
cargo check
REM git command
git add . && git commit -m Update && git push 
