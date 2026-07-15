@echo off
setlocal

cargo run -p foundation-build -- package %*
exit /b %ERRORLEVEL%
