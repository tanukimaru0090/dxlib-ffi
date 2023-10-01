@REM cargo check 
@cargo check
@REM Cargo Checkが成功したら
@if %errorlevel% equ 0 (
	@git add . && git commit -m Update && git push 
	@REM git コマンドが失敗したら
	@REM index.lockを削除してもう一度実行
	@if %errorlevel% equ 1 (
		echo git commandに失敗しました
		@rm "./.git/index.lock" 
		echo index.lockを削除しました
		echo 再度git commandを試します。
		@git add . && git commit -m Update && git push 
	)
) else (
		echo cargo checkに失敗しました
)
