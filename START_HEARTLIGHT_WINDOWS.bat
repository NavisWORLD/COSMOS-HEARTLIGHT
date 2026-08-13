@echo off
setlocal
where py >nul 2>nul
if %errorlevel%==0 (
  py -3 run_local.py
) else (
  python run_local.py
)
endlocal
