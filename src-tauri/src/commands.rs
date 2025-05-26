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
pub async fn create_member_with_address(
    db: State<'_, DbState>,
    member_data: serde_json::Value,
) -> Result<Member, String> {
    let db = db.lock().await;
    
    // Extract data from the frontend payload
    let osoitetyyppi = member_data["osoitetyyppi"].as_str().unwrap_or("oma");
    
    let address_id = match osoitetyyppi {
        "talous" => {
            // Join existing household
            member_data["talous_id"].as_i64().unwrap_or(1)
        },
        "oma" | "uusi" => {
            // Create new household and address
            let household_name = if osoitetyyppi == "uusi" {
                member_data["talouden_nimi"].as_str().map(|s| s.to_string())
            } else {
                Some(format!("{} {}", 
                    member_data["etunimi"].as_str().unwrap_or(""),
                    member_data["sukunimi"].as_str().unwrap_or("")))
            };
            
            let household = CreateHousehold {
                talouden_nimi: household_name,
                laskutusosoite_sama: true,
                laskutusosoite_id: None,
            };
            
            let created_household = db.create_household(&household).await
                .map_err(|e| e.to_string())?;
            
            let address = CreateAddress {
                katuosoite: member_data["katuosoite"].as_str().unwrap_or("").to_string(),
                postinumero: member_data["postinumero"].as_str().unwrap_or("").to_string(),
                postitoimipaikka: member_data["postitoimipaikka"].as_str().unwrap_or("").to_string(),
                talous_id: created_household.id,
            };
            
            let created_address = db.create_address(&address).await
                .map_err(|e| e.to_string())?;
            
            created_address.id
        },
        _ => return Err("Invalid osoitetyyppi".to_string()),
    };
    
    // Parse member type
    let member_type_str = member_data["jasentyyppi"].as_str().unwrap_or("Varsinainen");
    let member_type: MemberType = member_type_str.parse()
        .map_err(|e: String| format!("Invalid member type: {}", e))?;
    
    // Parse dates
    let syntymaaika = member_data["syntymaaika"].as_str()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    
    let liittymispaiva = member_data["liittymispaiva"].as_str()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    
    let member = CreateMember {
        etunimi: member_data["etunimi"].as_str().unwrap_or("").to_string(),
        sukunimi: member_data["sukunimi"].as_str().unwrap_or("").to_string(),
        henkilotunnus: member_data["henkilotunnus"].as_str().map(|s| s.to_string()),
        syntymaaika,
        puhelinnumero: member_data["puhelinnumero"].as_str().map(|s| s.to_string()),
        sahkoposti: member_data["sahkoposti"].as_str().map(|s| s.to_string()),
        osoite_id: address_id,
        liittymispaiva,
        jasentyyppi: member_type,
        aktiivinen: member_data["aktiivinen"].as_bool().unwrap_or(true),
    };
    
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
pub async fn update_member_with_address(
    db: State<'_, DbState>,
    id: i64,
    member_data: serde_json::Value,
) -> Result<Member, String> {
    let db = db.lock().await;
    
    // First update the address if provided
    if let (Some(katuosoite), Some(postinumero), Some(postitoimipaikka)) = (
        member_data["katuosoite"].as_str(),
        member_data["postinumero"].as_str(), 
        member_data["postitoimipaikka"].as_str()
    ) {
        let address_id = db.get_member_address_id(id).await.map_err(|e| e.to_string())?;
        db.update_address(address_id, katuosoite, postinumero, postitoimipaikka)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    // Parse member type
    let member_type_str = member_data["jasentyyppi"].as_str().unwrap_or("Varsinainen");
    let member_type: MemberType = member_type_str.parse()
        .map_err(|e: String| format!("Invalid member type: {}", e))?;
    
    // Parse dates
    let syntymaaika = member_data["syntymaaika"].as_str()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    
    let liittymispaiva = member_data["liittymispaiva"].as_str()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    
    // Get current osoite_id (don't change it for existing members)
    let address_id = db.get_member_address_id(id).await.map_err(|e| e.to_string())?;
    
    let member = CreateMember {
        etunimi: member_data["etunimi"].as_str().unwrap_or("").to_string(),
        sukunimi: member_data["sukunimi"].as_str().unwrap_or("").to_string(),
        henkilotunnus: member_data["henkilotunnus"].as_str().map(|s| s.to_string()),
        syntymaaika,
        puhelinnumero: member_data["puhelinnumero"].as_str().map(|s| s.to_string()),
        sahkoposti: member_data["sahkoposti"].as_str().map(|s| s.to_string()),
        osoite_id: address_id,
        liittymispaiva,
        jasentyyppi: member_type,
        aktiivinen: member_data["aktiivinen"].as_bool().unwrap_or(true),
    };
    
    db.update_member(id, &member).await.map_err(|e| e.to_string())
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