#!/bin/bash

# Development Database Setup Script for Laskutin
# This script creates a SQLite database with all required tables and sample data

DB_FILE="dev-database.db"

echo "Setting up development database: $DB_FILE"

# Remove existing database if it exists
if [ -f "$DB_FILE" ]; then
    echo "Removing existing database..."
    rm "$DB_FILE"
fi

# Create new database and run migrations
echo "Creating database and running migrations..."

# Apply first migration
sqlite3 "$DB_FILE" < src-tauri/migrations/001_initial.sql

# Apply second migration  
sqlite3 "$DB_FILE" < src-tauri/migrations/002_add_fields.sql

# Apply third migration
sqlite3 "$DB_FILE" < src-tauri/migrations/003_add_youth_member_age_limit.sql

# Apply fourth migration
sqlite3 "$DB_FILE" < src-tauri/migrations/004_update_member_type_constraints.sql

echo "Development database created successfully!"
echo "Location: $(pwd)/$DB_FILE"

# SQLX metadata will be generated automatically during first compilation

# Check if database was created successfully
if [ -f "$DB_FILE" ]; then
    echo "✅ Database setup complete"
    echo "📊 Sample data included:"
    echo "   - Example organization"
    echo "   - 2 households with addresses"
    echo "   - 3 members" 
    echo "   - Membership fees for 2024-2025"
    echo "🔧 SQLX metadata will be generated during first compilation"
    echo ""
    echo "You can now run: pnpm run tauri dev"
else
    echo "❌ Database creation failed"
    exit 1
fi