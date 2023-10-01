@REM cargo-git.vat
@REM cargo check and git command 

@REM cargo check 
@cargo check
@REM Cargo Checkが成功したら
:main
@setlocal
@set git_command=git add . ^& git commit -m Update^&git push

@if %errorlevel% equ 0 (
	@call %git_command%
	@REM git コマンドが失敗したら
	@REM index.lockを削除してもう一度実行
	@if %errorlevel% equ 1 (
		echo git commandに失敗しました
		@del "./.git/index.lock" 
		echo index.lockを削除しました
		echo 再度git commandを試します。
		@call %git_command% 
	)
) else (
		echo cargo checkに失敗,またはその他のエラーが発生しました
)
@endlocal
REM @call :main
