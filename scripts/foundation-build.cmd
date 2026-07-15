@echo off
setlocal

set "FOUNDATION_ROOT=%~dp0.."
cargo run --manifest-path "%FOUNDATION_ROOT%\Cargo.toml" -p foundation-build -- %*
exit /b %ERRORLEVEL%
