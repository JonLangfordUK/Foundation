@echo off
powershell -ExecutionPolicy Bypass -File "%~dp0Invoke-RustWorkspace.ps1" doc-project %*
