# Laskutin - Finnish Membership Billing Application

A Tauri-based membership billing application with SQLite database and Vue.js interface for Finnish organizations.

## Features

- 🏠 **Household Management** - Organize members by households with flexible billing
- 👥 **Member Management** - Complete CRUD operations for member data
- 💰 **Membership Fees** - Annual fee configuration by member type
- 📄 **Invoice Generation** - Automatic invoice creation per household
- 📄 **PDF Invoices** - Finnish banking standard layout with barcode
- 💳 **Payment Tracking** - Mark invoices as paid with payment dates
- 🏢 **Organization Settings** - Manage organization details for invoices

## Technology Stack

- **Backend**: Rust + Tauri 2.0
- **Frontend**: Vue.js 3 + TypeScript
- **Database**: SQLite with migrations
- **Package Manager**: pnpm
- **Build System**: Vite
- **PDF Generation**: jsPDF + html2canvas
- **Banking Barcode**: JsBarcode (CODE128)
- **Date Picker**: @vuepic/vue-datepicker

## Quick Start

### Prerequisites

- Node.js 20.x
- Rust (stable)
- pnpm

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd Laskutin

# Install dependencies
pnpm install

# Set up development database
./setup-dev-db.sh        # Linux/macOS
# OR
setup-dev-db.bat         # Windows

# Start development server
npm run tauri dev
```

### Development Database Setup

The application requires a SQLite database for development. The database setup scripts create a local development database with sample data:

**Linux/macOS:**
```bash
./setup-dev-db.sh
```

**Windows:**
```batch
setup-dev-db.bat
```

**What the setup script does:**
- Creates `dev-database.db` in the project root
- Runs all database migrations (001_initial.sql, 002_add_fields.sql)
- Populates sample data (organization, households, members, membership fees)
- Required for successful compilation due to SQLX compile-time verification
- SQLX metadata (`.sqlx/` directory) is generated automatically during compilation

**Database Behavior:**
- **Development mode**: Uses `dev-database.db` in project root
- **Production mode**: Uses user data directory (`~/.local/share/laskutin/` on Linux)
- Development database is excluded from Git and production builds

### Building

```bash
# Build for production
npm run tauri build
```

## Database Schema

### Core Tables

- **organization** - Organization details (name, address, bank info)
- **households** - Billing groups with recipient information
- **addresses** - Physical addresses linked to households
- **members** - Individual member records
- **membership_fees** - Annual fees by member type
- **invoices** - Generated invoices per household
- **invoice_lines** - Individual member charges per invoice

## Key Features

### Flexible Member Management

Three membership options:
- **'oma'** - Create new household for single person
- **'talous'** - Join existing household
- **'uusi'** - Create new household that others can join

### Finnish Invoice Standards

- Standard Finnish invoice layout
- Banking barcode generation (Pankkiviivakoodi)
- Reference number generation
- Due date calculation
- Native file save dialogs

### Smart Household Billing

- One invoice per household regardless of member count
- Automatic invoice line generation for each household member
- Empty household cleanup when last member leaves
- Flexible billing address management

## Development

### Database Migrations

Migrations run automatically on startup:
- `migrations/001_initial.sql` - Initial schema
- `migrations/002_add_fields.sql` - Additional fields

### Backend Commands

Key Tauri commands exposed to frontend:
- Organization CRUD operations
- Member and household management
- Invoice generation and payment tracking
- Native file operations for PDF export

### Technical Notes

- SQLite DECIMAL values handled with fallback to INTEGER
- Frontend uses camelCase, backend uses snake_case
- Custom CSS styling (Tailwind removed due to ES module conflicts)

## Troubleshooting

### Database Issues

**Error: "unable to open database file" during compilation or IDE errors**
```bash
# Run the database setup script
./setup-dev-db.sh

# The first compilation will generate SQLX metadata automatically
# If you still get IDE errors, try building once:
pnpm run tauri build --debug
```

**Error: "No such file or directory" when running setup script**
```bash
# Make sure you're in the project root directory
chmod +x setup-dev-db.sh
./setup-dev-db.sh
```

**Missing SQLite on Windows**
- Install SQLite from https://sqlite.org/download.html
- Add SQLite to your PATH

### Graphics Issues (WSL)

If you encounter graphics-related errors in WSL, set:
```bash
export LIBGL_ALWAYS_INDIRECT=1
```

## Environment

Optimized for:
- WSL2 + Ubuntu
- Node.js 20.x
- Rust stable toolchain
- SQLite 3.x

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## License

MIT

## Development Notes

This project was developed with assistance from [Claude Code](https://claude.ai/code), Anthropic's AI coding assistant, which helped with various tasks.

## Contributing

Contributions to Laskutin are welcome! Here's how you can help:

1. **Fork the repository** and create your feature branch
2. **Write your changes** with clear, maintainable code
3. **Add tests** if applicable for new features
4. **Update documentation** to reflect any changes
5. **Submit a pull request** with a clear description of the changes

### Code Style Guidelines

- Follow Rust's official style guide for backend code
- Use Vue.js best practices for frontend components
- Maintain consistent naming conventions
- Add comments for complex logic

### Reporting Issues

Please use the issue tracker to report bugs or suggest features. Include:
- Clear description of the issue
- Steps to reproduce
- Expected vs actual results
- Environment details (OS, browser version, etc.)

### Development Process

1. Pick an existing issue or create a new one
2. Comment on the issue to indicate you're working on it
3. Create a feature branch from `main`
4. Submit a PR referencing the issue number

Your contributions help make Laskutin better for Finnish organizations!
