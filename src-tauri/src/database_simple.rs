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

        // Fetch and return the created member
        let row = sqlx::query(
            "SELECT * FROM members WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Member {
            id: row.get("id"),
            etunimi: row.get("etunimi"),
            sukunimi: row.get("sukunimi"),
            henkilotunnus: row.get("henkilotunnus"),
            syntymaaika: row.get("syntymaaika"),
            puhelinnumero: row.get("puhelinnumero"),
            sahkoposti: row.get("sahkoposti"),
            osoite_id: row.get("osoite_id"),
            liittymispaiva: row.get("liittymispaiva"),
            jasentyyppi: row.get("jasentyyppi"),
            aktiivinen: row.get("aktiivinen"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn create_address(&self, address: &CreateAddress) -> Result<Address> {
        let id = sqlx::query(
            "INSERT INTO addresses (katuosoite, postinumero, postitoimipaikka, talous_id)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&address.katuosoite)
        .bind(&address.postinumero)
        .bind(&address.postitoimipaikka)
        .bind(address.talous_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(Address {
            id,
            katuosoite: address.katuosoite.clone(),
            postinumero: address.postinumero.clone(),
            postitoimipaikka: address.postitoimipaikka.clone(),
            talous_id: address.talous_id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn get_members(&self) -> Result<Vec<MemberWithAddress>> {
        let rows = sqlx::query(
            "SELECT 
                m.id, m.etunimi, m.sukunimi, m.henkilotunnus, m.syntymaaika,
                m.puhelinnumero, m.sahkoposti, m.osoite_id, m.liittymispaiva,
                m.jasentyyppi, m.aktiivinen, m.created_at, m.updated_at,
                a.katuosoite, a.postinumero, a.postitoimipaikka, a.talous_id,
                a.created_at as address_created_at, a.updated_at as address_updated_at,
                h.talouden_nimi, h.laskutusosoite_sama, h.laskutusosoite_id,
                h.created_at as household_created_at, h.updated_at as household_updated_at
             FROM members m
             JOIN addresses a ON m.osoite_id = a.id
             JOIN households h ON a.talous_id = h.id
             ORDER BY m.sukunimi, m.etunimi"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut members = Vec::new();
        for row in rows {
            let member = Member {
                id: row.get("id"),
                etunimi: row.get("etunimi"),
                sukunimi: row.get("sukunimi"),
                henkilotunnus: row.get("henkilotunnus"),
                syntymaaika: row.get("syntymaaika"),
                puhelinnumero: row.get("puhelinnumero"),
                sahkoposti: row.get("sahkoposti"),
                osoite_id: row.get("osoite_id"),
                liittymispaiva: row.get("liittymispaiva"),
                jasentyyppi: row.get("jasentyyppi"),
                aktiivinen: row.get("aktiivinen"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            let address = Address {
                id: row.get("osoite_id"),
                katuosoite: row.get("katuosoite"),
                postinumero: row.get("postinumero"),
                postitoimipaikka: row.get("postitoimipaikka"),
                talous_id: row.get("talous_id"),
                created_at: row.get("address_created_at"),
                updated_at: row.get("address_updated_at"),
            };

            let household = Household {
                id: row.get("talous_id"),
                talouden_nimi: row.get("talouden_nimi"),
                laskutusosoite_sama: row.get("laskutusosoite_sama"),
                laskutusosoite_id: row.get("laskutusosoite_id"),
                created_at: row.get("household_created_at"),
                updated_at: row.get("household_updated_at"),
            };

            members.push(MemberWithAddress {
                member,
                address,
                household,
            });
        }

        Ok(members)
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

    pub async fn update_member(&self, id: i64, member: &CreateMember) -> Result<Member> {
        sqlx::query(
            "UPDATE members SET 
             etunimi = ?, sukunimi = ?, henkilotunnus = ?, syntymaaika = ?,
             puhelinnumero = ?, sahkoposti = ?, osoite_id = ?, liittymispaiva = ?,
             jasentyyppi = ?, aktiivinen = ?, updated_at = CURRENT_TIMESTAMP
             WHERE id = ?"
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
        .bind(id)
        .execute(&self.pool)
        .await?;

        // Fetch and return the updated member
        let row = sqlx::query(
            "SELECT * FROM members WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Member {
            id: row.get("id"),
            etunimi: row.get("etunimi"),
            sukunimi: row.get("sukunimi"),
            henkilotunnus: row.get("henkilotunnus"),
            syntymaaika: row.get("syntymaaika"),
            puhelinnumero: row.get("puhelinnumero"),
            sahkoposti: row.get("sahkoposti"),
            osoite_id: row.get("osoite_id"),
            liittymispaiva: row.get("liittymispaiva"),
            jasentyyppi: row.get("jasentyyppi"),
            aktiivinen: row.get("aktiivinen"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn delete_member(&self, _id: i64) -> Result<()> {
        Ok(())
    }

    pub async fn get_households(&self) -> Result<Vec<Household>> {
        Ok(vec![])
    }

    pub async fn create_household(&self, household: &CreateHousehold) -> Result<Household> {
        let id = sqlx::query(
            "INSERT INTO households (talouden_nimi, laskutusosoite_sama, laskutusosoite_id)
             VALUES (?, ?, ?)"
        )
        .bind(&household.talouden_nimi)
        .bind(household.laskutusosoite_sama)
        .bind(household.laskutusosoite_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(Household {
            id,
            talouden_nimi: household.talouden_nimi.clone(),
            laskutusosoite_sama: household.laskutusosoite_sama,
            laskutusosoite_id: household.laskutusosoite_id,
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