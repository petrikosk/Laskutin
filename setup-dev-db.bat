@echo off
REM Development Database Setup Script for Laskutin (Windows)
REM This script creates a SQLite database with all required tables and sample data

set DB_FILE=dev-database.db

echo Setting up development database: %DB_FILE%

REM Remove existing database if it exists
if exist "%DB_FILE%" (
    echo Removing existing database...
    del "%DB_FILE%"
)

REM Create new database and run migrations
echo Creating database and running migrations...

REM Apply first migration
sqlite3 "%DB_FILE%" < src-tauri/migrations/001_initial.sql

REM Apply second migration  
sqlite3 "%DB_FILE%" < src-tauri/migrations/002_add_fields.sql

REM Apply third migration
sqlite3 "%DB_FILE%" < src-tauri/migrations/003_add_youth_member_age_limit.sql

REM Apply fourth migration
sqlite3 "%DB_FILE%" < src-tauri/migrations/004_update_member_type_constraints.sql

echo Development database created successfully!
echo Location: %CD%\%DB_FILE%

REM SQLX metadata will be generated automatically during first compilation

REM Check if database was created successfully
if exist "%DB_FILE%" (
    echo ✅ Database setup complete
    echo 📊 Sample data included:
    echo    - Example organization
    echo    - 2 households with addresses
    echo    - 3 members
    echo    - Membership fees for 2024-2025
    echo 🔧 SQLX metadata will be generated during first compilation
    echo.
    echo You can now run: pnpm run tauri dev
) else (
    echo ❌ Database creation failed
    exit /b 1
)