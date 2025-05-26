use crate::database::Database;
use crate::models::*;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

type DbState = Arc<Mutex<Database>>;

#[tauri::command]
pub async fn get_organization(db: State<'_, DbState>) -> Result<Option<Organization>, String> {
    let db = db.lock().await;
    db.get_organization().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_organization(
    db: State<'_, DbState>,
    organization: CreateOrganization,
) -> Result<Organization, String> {
    let db = db.lock().await;
    db.update_organization(&organization)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_members(db: State<'_, DbState>) -> Result<Vec<MemberWithAddress>, String> {
    let db = db.lock().await;
    db.get_members().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_member(
    db: State<'_, DbState>,
    member: CreateMember,
) -> Result<Member, String> {
    let db = db.lock().await;
    db.create_member(&member).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_member(
    db: State<'_, DbState>,
    id: i64,
    member: CreateMember,
) -> Result<Member, String> {
    let db = db.lock().await;
    db.update_member(id, &member)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_member(db: State<'_, DbState>, id: i64) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_member(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_households(db: State<'_, DbState>) -> Result<Vec<Household>, String> {
    let db = db.lock().await;
    db.get_households().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_household(
    db: State<'_, DbState>,
    household: CreateHousehold,
) -> Result<Household, String> {
    let db = db.lock().await;
    db.create_household(&household)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_membership_fees(db: State<'_, DbState>) -> Result<Vec<MembershipFee>, String> {
    let db = db.lock().await;
    db.get_membership_fees().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_membership_fee(
    db: State<'_, DbState>,
    fee: CreateMembershipFee,
) -> Result<MembershipFee, String> {
    let db = db.lock().await;
    db.create_membership_fee(&fee)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_membership_fee(
    db: State<'_, DbState>,
    id: i64,
    fee: CreateMembershipFee,
) -> Result<MembershipFee, String> {
    let db = db.lock().await;
    db.update_membership_fee(id, &fee)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invoices(db: State<'_, DbState>) -> Result<Vec<InvoiceWithDetails>, String> {
    let db = db.lock().await;
    db.get_invoices().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_invoice_for_year(
    db: State<'_, DbState>,
    year: i32,
) -> Result<Vec<Invoice>, String> {
    let db = db.lock().await;
    db.create_invoice_for_year(year)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn mark_invoice_paid(
    db: State<'_, DbState>,
    id: i64,
    payment_date: String,
) -> Result<Invoice, String> {
    let payment_date = chrono::NaiveDate::parse_from_str(&payment_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))?;

    let db = db.lock().await;
    db.mark_invoice_paid(id, payment_date)
        .await
        .map_err(|e| e.to_string())
}