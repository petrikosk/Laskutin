# Development Database Setup Script for Laskutin (PowerShell)
# This script creates a SQLite database with all required tables and sample data

$DB_FILE = "dev-database.db"

Write-Host "Setting up development database: $DB_FILE" -ForegroundColor Green

# Remove existing database if it exists
if (Test-Path $DB_FILE) {
    Write-Host "Removing existing database..." -ForegroundColor Yellow
    Remove-Item $DB_FILE
}

# Create new database and run migrations
Write-Host "Creating database and running migrations..." -ForegroundColor Blue

# Check if sqlite3 is available
if (-not (Get-Command sqlite3 -ErrorAction SilentlyContinue)) {
    Write-Host "❌ sqlite3 command not found!" -ForegroundColor Red
    Write-Host "Please install SQLite3:" -ForegroundColor Yellow
    Write-Host "  - Download from: https://sqlite.org/download.html" -ForegroundColor Yellow
    Write-Host "  - Or use: winget install SQLite.SQLite" -ForegroundColor Yellow
    exit 1
}

try {
    # Apply first migration
    Get-Content "src-tauri/migrations/001_initial.sql" | sqlite3 $DB_FILE
    
    # Apply second migration  
    Get-Content "src-tauri/migrations/002_add_fields.sql" | sqlite3 $DB_FILE
    
    Write-Host "Development database created successfully!" -ForegroundColor Green
    Write-Host "Location: $(Get-Location)\$DB_FILE" -ForegroundColor Cyan
    
    # SQLX metadata will be generated automatically during first compilation
    
    # Check if database was created successfully
    if (Test-Path $DB_FILE) {
        Write-Host "✅ Database setup complete" -ForegroundColor Green
        Write-Host "📊 Sample data included:" -ForegroundColor Cyan
        Write-Host "   - Example organization" -ForegroundColor Gray
        Write-Host "   - 2 households with addresses" -ForegroundColor Gray
        Write-Host "   - 3 members" -ForegroundColor Gray
        Write-Host "   - Membership fees for 2024-2025" -ForegroundColor Gray
        Write-Host "🔧 SQLX metadata will be generated during first compilation" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "You can now run: pnpm run tauri dev" -ForegroundColor Green
    } else {
        Write-Host "❌ Database creation failed" -ForegroundColor Red
        exit 1
    }
} catch {
    Write-Host "❌ Error creating database: $_" -ForegroundColor Red
    exit 1
}