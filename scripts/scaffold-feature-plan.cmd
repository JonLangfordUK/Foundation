@echo off
setlocal
powershell -ExecutionPolicy Bypass -File "%~dp0Scaffold-FeaturePlan.ps1" %*
