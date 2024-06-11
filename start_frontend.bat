@echo off
REM Start the Docker container for frontend only
start /b docker-compose up --build frontend

REM Wait for the frontend to be ready
:waitloop
curl -s http://localhost:8080 >nul
if %ERRORLEVEL% neq 0 (
    echo Waiting for frontend to be ready...
    timeout /t 5 >nul
    goto waitloop
)

REM Open the frontend URL in the default web browser
start http://localhost:8080
