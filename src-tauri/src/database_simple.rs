use crate::models::*;
use anyhow::Result;
use sqlx::{Pool, Sqlite, Row, migrate::MigrateDatabase};
use std::path::PathBuf;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let app_data_dir = dirs::data_dir()
            .map(|dir| dir.join("laskutin"))
            .unwrap_or_else(|| PathBuf::from("."));

        std::fs::create_dir_all(&app_data_dir)?;

        let db_path = app_data_dir.join("laskutin.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());

        // Create database if it doesn't exist
        if !db_path.exists() {
            sqlx::Sqlite::create_database(&db_url).await?;
        }

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        // Run migrations manually
        sqlx::query(include_str!("../migrations/001_initial.sql"))
            .execute(&pool)
            .await
            .ok(); // Ignore errors if tables already exist

        Ok(Database { pool })
    }

    pub async fn get_organization(&self) -> Result<Option<Organization>> {
        let result = sqlx::query("SELECT * FROM organization LIMIT 1")
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = result {
            Ok(Some(Organization {
                id: row.get("id"),
                nimi: row.get("nimi"),
                katuosoite: row.get("katuosoite"),
                postinumero: row.get("postinumero"),
                postitoimipaikka: row.get("postitoimipaikka"),
                puhelinnumero: row.get("puhelinnumero"),
                sahkoposti: row.get("sahkoposti"),
                y_tunnus: row.get("y_tunnus"),
                pankkitili: row.get("pankkitili"),
                bic: row.get("bic"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create_member(&self, member: &CreateMember) -> Result<Member> {
        let id = sqlx::query(
            "INSERT INTO members (etunimi, sukunimi, henkilotunnus, syntymaaika, 
             puhelinnumero, sahkoposti, osoite_id, liittymispaiva, jasentyyppi, aktiivinen)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&member.etunimi)
        .bind(&member.sukunimi)
        .bind(&member.henkilotunnus)
        .bind(&member.syntymaaika)
        .bind(&member.puhelinnumero)
        .bind(&member.sahkoposti)
        .bind(member.osoite_id)
        .bind(&member.liittymispaiva)
        .bind(member.jasentyyppi.to_string())
        .bind(member.aktiivinen)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        // Return a basic member for now
        Ok(Member {
            id,
            etunimi: member.etunimi.clone(),
            sukunimi: member.sukunimi.clone(),
            henkilotunnus: member.henkilotunnus.clone(),
            syntymaaika: member.syntymaaika,
            puhelinnumero: member.puhelinnumero.clone(),
            sahkoposti: member.sahkoposti.clone(),
            osoite_id: member.osoite_id,
            liittymispaiva: member.liittymispaiva,
            jasentyyppi: member.jasentyyppi.to_string(),
            aktiivinen: member.aktiivinen,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    // Placeholder methods - implement later
    pub async fn get_members(&self) -> Result<Vec<MemberWithAddress>> {
        Ok(vec![])
    }

    pub async fn update_organization(&self, _org: &CreateOrganization) -> Result<Organization> {
        // Placeholder - return dummy data for now
        Ok(Organization {
            id: 1,
            nimi: "Test Org".to_string(),
            katuosoite: "Test Address".to_string(),
            postinumero: "00100".to_string(),
            postitoimipaikka: "Helsinki".to_string(),
            puhelinnumero: None,
            sahkoposti: None,
            y_tunnus: None,
            pankkitili: None,
            bic: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn update_member(&self, _id: i64, _member: &CreateMember) -> Result<Member> {
        // Placeholder
        Ok(Member {
            id: 1,
            etunimi: "Test".to_string(),
            sukunimi: "User".to_string(),
            henkilotunnus: None,
            syntymaaika: None,
            puhelinnumero: None,
            sahkoposti: None,
            osoite_id: 1,
            liittymispaiva: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            jasentyyppi: "varsinainen".to_string(),
            aktiivinen: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn delete_member(&self, _id: i64) -> Result<()> {
        Ok(())
    }

    pub async fn get_households(&self) -> Result<Vec<Household>> {
        Ok(vec![])
    }

    pub async fn create_household(&self, _household: &CreateHousehold) -> Result<Household> {
        Ok(Household {
            id: 1,
            talouden_nimi: None,
            laskutusosoite_sama: true,
            laskutusosoite_id: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn get_membership_fees(&self) -> Result<Vec<MembershipFee>> {
        Ok(vec![])
    }

    pub async fn create_membership_fee(&self, _fee: &CreateMembershipFee) -> Result<MembershipFee> {
        Ok(MembershipFee {
            id: 1,
            vuosi: 2024,
            jasentyyppi: "varsinainen".to_string(),
            summa: 50.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn update_membership_fee(&self, _id: i64, _fee: &CreateMembershipFee) -> Result<MembershipFee> {
        Ok(MembershipFee {
            id: 1,
            vuosi: 2024,
            jasentyyppi: "varsinainen".to_string(),
            summa: 50.0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn get_invoices(&self) -> Result<Vec<InvoiceWithDetails>> {
        Ok(vec![])
    }

    pub async fn create_invoice_for_year(&self, _year: i32) -> Result<Vec<Invoice>> {
        Ok(vec![])
    }

    pub async fn mark_invoice_paid(&self, _id: i64, _payment_date: chrono::NaiveDate) -> Result<Invoice> {
        Ok(Invoice {
            id: 1,
            talous_id: 1,
            luontipaiva: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            erapaiva: chrono::NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            summa: 50.0,
            viitenumero: "12345".to_string(),
            maksettu: true,
            maksupaiva: Some(chrono::NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}