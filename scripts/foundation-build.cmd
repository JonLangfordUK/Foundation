@echo off
setlocal

cargo run -p foundation-build -- %*
exit /b %ERRORLEVEL%
