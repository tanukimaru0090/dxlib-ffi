@REM cargo-git.bat
@REM cargo check and git command 
:main

@setlocal
@set git_command=git add . ^& git commit -m Update^&git push

:git_err
	@if %errorlevel% equ 1 (
		echo git commandに失敗しました
		@del "./.git/index.lock" 
		echo index.lockを削除しました
		echo 再度git commandを試します。
		@call %git_command% 
	)
:git_err_eof 

@REM *MainEntry*
	@REM cargo check 
	@cargo check
	@call %git_command%
	@REM git コマンドが失敗したら
	@REM index.lockを削除してもう一度実行
	@if %errorlevel% equ 1 (
	     @REM エラーメッセージ表示をし、git commandを再度行う
	     @call :git_err
	     @goto git_err_eof
	) else (
		echo cargo checkに失敗,またはその他のエラーが発生しました
	)	
@endlocal
:main_eof

@call main
goto main_eof
