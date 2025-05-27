use crate::database::Database;
use crate::models::*;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use chrono::Datelike;

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
                talouden_nimi: household_name.clone(),
                vastaanottaja: household_name,
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
pub async fn get_households_with_addresses(db: State<'_, DbState>) -> Result<Vec<serde_json::Value>, String> {
    let db = db.lock().await;
    let households_with_addresses = db.get_households_with_addresses().await.map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for (household, address) in households_with_addresses {
        let member_count = db.get_household_member_count(household.id).await.map_err(|e| e.to_string())?;
        
        result.push(serde_json::json!({
            "id": household.id,
            "talouden_nimi": household.talouden_nimi,
            "osoite": format!("{}, {} {}", address.katuosoite, address.postinumero, address.postitoimipaikka),
            "member_count": member_count,
            "created_at": household.created_at,
            "address": {
                "katuosoite": address.katuosoite,
                "postinumero": address.postinumero,
                "postitoimipaikka": address.postitoimipaikka
            }
        }));
    }
    
    Ok(result)
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
pub async fn create_household_with_address(
    db: State<'_, DbState>,
    household_data: serde_json::Value,
) -> Result<Household, String> {
    let db = db.lock().await;
    
    // Create household first
    let household = CreateHousehold {
        talouden_nimi: household_data["talouden_nimi"].as_str().map(|s| s.to_string()),
        vastaanottaja: household_data["vastaanottaja"].as_str().map(|s| s.to_string()),
        laskutusosoite_sama: true,
        laskutusosoite_id: None,
    };
    
    let created_household = db.create_household(&household).await.map_err(|e| e.to_string())?;
    
    // Create address for the household
    let address = CreateAddress {
        katuosoite: household_data["katuosoite"].as_str().unwrap_or("").to_string(),
        postinumero: household_data["postinumero"].as_str().unwrap_or("").to_string(),
        postitoimipaikka: household_data["postitoimipaikka"].as_str().unwrap_or("").to_string(),
        talous_id: created_household.id,
    };
    
    db.create_address(&address).await.map_err(|e| e.to_string())?;
    
    Ok(created_household)
}

#[tauri::command]
pub async fn update_household_with_address(
    db: State<'_, DbState>,
    id: i64,
    household_data: serde_json::Value,
) -> Result<Household, String> {
    let db = db.lock().await;
    
    // Update household name
    let household = CreateHousehold {
        talouden_nimi: household_data["talouden_nimi"].as_str().map(|s| s.to_string()),
        vastaanottaja: household_data["vastaanottaja"].as_str().map(|s| s.to_string()),
        laskutusosoite_sama: true,
        laskutusosoite_id: None,
    };
    
    let updated_household = db.update_household(id, &household).await.map_err(|e| e.to_string())?;
    
    // Update address if provided
    if let (Some(katuosoite), Some(postinumero), Some(postitoimipaikka)) = (
        household_data["katuosoite"].as_str(),
        household_data["postinumero"].as_str(),
        household_data["postitoimipaikka"].as_str(),
    ) {
        db.update_household_address(id, katuosoite, postinumero, postitoimipaikka)
            .await
            .map_err(|e| e.to_string())?;
    }
    
    Ok(updated_household)
}

#[tauri::command]
pub async fn delete_household(
    db: State<'_, DbState>,
    id: i64,
) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_household(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_pdf_file(
    file_path: String,
    data: Vec<u8>,
) -> Result<(), String> {
    use std::fs;
    
    fs::write(&file_path, data)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub async fn show_save_dialog(
    app_handle: tauri::AppHandle,
    default_filename: String,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let file_path = app_handle
        .dialog()
        .file()
        .set_file_name(&default_filename)
        .add_filter("PDF Files", &["pdf"])
        .blocking_save_file();
    
    Ok(file_path.map(|p| p.to_string()))
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
pub async fn validate_invoice_creation(
    db: State<'_, DbState>,
    year: i32,
) -> Result<String, String> {
    let db = db.lock().await;
    db.validate_invoice_creation(year)
        .await
        .map_err(|e| e.to_string())
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

#[tauri::command]
pub async fn delete_invoice(
    db: State<'_, DbState>,
    id: i64,
) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_invoice(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_membership_fee(
    db: State<'_, DbState>,
    id: i64,
) -> Result<(), String> {
    let db = db.lock().await;
    db.delete_membership_fee(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_dashboard_stats(db: State<'_, DbState>) -> Result<DashboardStats, String> {
    let db = db.lock().await;
    
    let current_year = chrono::Utc::now().year();
    
    let total_members = db.get_total_members().await.map_err(|e| e.to_string())?;
    let open_invoices = db.get_open_invoices_count().await.map_err(|e| e.to_string())?;
    let total_receivables = db.get_total_receivables().await.map_err(|e| e.to_string())?;
    let yearly_income = db.get_yearly_income(current_year).await.map_err(|e| e.to_string())?;
    
    Ok(DashboardStats {
        total_members,
        open_invoices,
        total_receivables,
        yearly_income,
    })
}