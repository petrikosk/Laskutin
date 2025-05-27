mod models;
#[path = "database_simple.rs"]
mod database;
mod commands;

use database::Database;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let db = Database::new().await.expect("Failed to initialize database");
        let db_state = Arc::new(Mutex::new(db));

        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_dialog::init())
            .manage(db_state)
            .invoke_handler(tauri::generate_handler![
                commands::get_organization,
                commands::update_organization,
                commands::get_members,
                commands::create_member,
                commands::create_member_with_address,
                commands::update_member,
                commands::update_member_with_address,
                commands::delete_member,
                commands::get_households,
                commands::get_households_with_addresses,
                commands::create_household,
                commands::create_household_with_address,
                commands::update_household_with_address,
                commands::delete_household,
                commands::save_pdf_file,
                commands::show_save_dialog,
                commands::get_membership_fees,
                commands::create_membership_fee,
                commands::update_membership_fee,
                commands::delete_membership_fee,
                commands::get_invoices,
                commands::validate_invoice_creation,
                commands::create_invoice_for_year,
                commands::mark_invoice_paid,
                commands::delete_invoice,
                commands::get_dashboard_stats,
                commands::show_directory_dialog,
                commands::show_file_dialog,
                commands::backup_database,
                commands::restore_database
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
