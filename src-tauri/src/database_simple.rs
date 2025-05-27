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

        // Run second migration
        sqlx::query(include_str!("../migrations/002_add_fields.sql"))
            .execute(&pool)
            .await
            .ok(); // Ignore errors if columns already exist

        Ok(Database { pool })
    }

    pub async fn close(&self) {
        self.pool.close().await;
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
                vastaanottaja: row.try_get("vastaanottaja").ok(),
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

    pub async fn update_organization(&self, org: &CreateOrganization) -> Result<Organization> {
        // Check if organization exists
        let existing = sqlx::query!("SELECT id FROM organization LIMIT 1")
            .fetch_optional(&self.pool)
            .await?;

        if existing.is_some() {
            // Update existing organization
            sqlx::query!(
                "UPDATE organization SET 
                 nimi = ?, katuosoite = ?, postinumero = ?, postitoimipaikka = ?,
                 puhelinnumero = ?, sahkoposti = ?, y_tunnus = ?, pankkitili = ?, bic = ?,
                 updated_at = CURRENT_TIMESTAMP
                 WHERE id = (SELECT id FROM organization LIMIT 1)",
                org.nimi,
                org.katuosoite,
                org.postinumero,
                org.postitoimipaikka,
                org.puhelinnumero,
                org.sahkoposti,
                org.y_tunnus,
                org.pankkitili,
                org.bic
            )
            .execute(&self.pool)
            .await?;
        } else {
            // Insert new organization
            sqlx::query!(
                "INSERT INTO organization (nimi, katuosoite, postinumero, postitoimipaikka, puhelinnumero, sahkoposti, y_tunnus, pankkitili, bic)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                org.nimi,
                org.katuosoite,
                org.postinumero,
                org.postitoimipaikka,
                org.puhelinnumero,
                org.sahkoposti,
                org.y_tunnus,
                org.pankkitili,
                org.bic
            )
            .execute(&self.pool)
            .await?;
        }

        // Return updated organization
        let org = self.get_organization().await?;
        Ok(org.ok_or_else(|| sqlx::Error::RowNotFound)?)
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

    pub async fn delete_member(&self, id: i64) -> Result<()> {
        // Start a transaction to ensure data consistency
        let mut transaction = self.pool.begin().await?;
        
        // Check if member has invoice lines
        let invoice_lines_count = sqlx::query(
            "SELECT COUNT(*) as count FROM invoice_lines WHERE jasen_id = ?"
        )
        .bind(id)
        .fetch_one(&mut *transaction)
        .await?
        .get::<i64, _>("count");
        
        if invoice_lines_count > 0 {
            transaction.rollback().await?;
            return Err(anyhow::anyhow!(
                "Jäsentä ei voi poistaa, koska siihen liittyy laskurivejä. Poista ensin laskut."
            ));
        }
        
        // Get member's household info before deletion for cleanup
        let household_info = sqlx::query(
            "SELECT a.talous_id, COUNT(m.id) as member_count 
             FROM addresses a 
             LEFT JOIN members m ON m.osoite_id = a.id 
             WHERE a.id = (SELECT osoite_id FROM members WHERE id = ?)
             GROUP BY a.talous_id"
        )
        .bind(id)
        .fetch_optional(&mut *transaction)
        .await?;
        
        // Delete the member
        let affected_rows = sqlx::query("DELETE FROM members WHERE id = ?")
            .bind(id)
            .execute(&mut *transaction)
            .await?
            .rows_affected();
        
        if affected_rows == 0 {
            transaction.rollback().await?;
            return Err(anyhow::anyhow!("Jäsentä ei löytynyt ID:llä {}", id));
        }
        
        // Check if household became empty and clean up if needed
        if let Some(household_row) = household_info {
            let household_id: i64 = household_row.get("talous_id");
            let remaining_members = sqlx::query(
                "SELECT COUNT(*) as count FROM members m 
                 JOIN addresses a ON m.osoite_id = a.id 
                 WHERE a.talous_id = ?"
            )
            .bind(household_id)
            .fetch_one(&mut *transaction)
            .await?
            .get::<i64, _>("count");
            
            // If no members left in household, delete the household and its address
            if remaining_members == 0 {
                // Delete address first (which will cascade delete members if any remain)
                sqlx::query("DELETE FROM addresses WHERE talous_id = ?")
                    .bind(household_id)
                    .execute(&mut *transaction)
                    .await?;
                
                // Delete household
                sqlx::query("DELETE FROM households WHERE id = ?")
                    .bind(household_id)
                    .execute(&mut *transaction)
                    .await?;
            }
        }
        
        // Commit the transaction
        transaction.commit().await?;
        Ok(())
    }

    pub async fn get_households(&self) -> Result<Vec<Household>> {
        let rows = sqlx::query("SELECT * FROM households ORDER BY talouden_nimi")
            .fetch_all(&self.pool)
            .await?;

        let mut households = Vec::new();
        for row in rows {
            households.push(Household {
                id: row.get("id"),
                talouden_nimi: row.get("talouden_nimi"),
                vastaanottaja: row.try_get("vastaanottaja").ok(),
                laskutusosoite_sama: row.get("laskutusosoite_sama"),
                laskutusosoite_id: row.get("laskutusosoite_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(households)
    }

    pub async fn get_households_with_addresses(&self) -> Result<Vec<(Household, Address)>> {
        let rows = sqlx::query(
            "SELECT 
                h.id, h.talouden_nimi, h.laskutusosoite_sama, h.laskutusosoite_id,
                h.created_at, h.updated_at,
                a.katuosoite, a.postinumero, a.postitoimipaikka, a.talous_id,
                a.created_at as address_created_at, a.updated_at as address_updated_at
             FROM households h
             JOIN addresses a ON h.id = a.talous_id
             ORDER BY h.talouden_nimi"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for row in rows {
            let household = Household {
                id: row.get("id"),
                talouden_nimi: row.get("talouden_nimi"),
                vastaanottaja: row.try_get("vastaanottaja").ok(),
                laskutusosoite_sama: row.get("laskutusosoite_sama"),
                laskutusosoite_id: row.get("laskutusosoite_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            let address = Address {
                id: row.get("talous_id"), // This should be address id, but we'll use talous_id for now
                katuosoite: row.get("katuosoite"),
                postinumero: row.get("postinumero"),
                postitoimipaikka: row.get("postitoimipaikka"),
                talous_id: row.get("talous_id"),
                created_at: row.get("address_created_at"),
                updated_at: row.get("address_updated_at"),
            };

            result.push((household, address));
        }

        Ok(result)
    }

    pub async fn create_household(&self, household: &CreateHousehold) -> Result<Household> {
        let id = sqlx::query(
            "INSERT INTO households (talouden_nimi, vastaanottaja, laskutusosoite_sama, laskutusosoite_id)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&household.talouden_nimi)
        .bind(&household.vastaanottaja)
        .bind(household.laskutusosoite_sama)
        .bind(household.laskutusosoite_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(Household {
            id,
            talouden_nimi: household.talouden_nimi.clone(),
            vastaanottaja: household.vastaanottaja.clone(),
            laskutusosoite_sama: household.laskutusosoite_sama,
            laskutusosoite_id: household.laskutusosoite_id,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn get_membership_fees(&self) -> Result<Vec<MembershipFee>> {
        let rows = sqlx::query("SELECT * FROM membership_fees ORDER BY vuosi DESC, jasentyyppi")
            .fetch_all(&self.pool)
            .await?;

        let mut fees = Vec::new();
        for row in rows {
            let summa = match row.try_get::<f64, _>("summa") {
                Ok(val) => val,
                Err(_) => row.get::<i64, _>("summa") as f64,
            };
            
            fees.push(MembershipFee {
                id: row.get("id"),
                vuosi: row.get("vuosi"),
                jasentyyppi: row.get("jasentyyppi"),
                summa,
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }

        Ok(fees)
    }

    pub async fn create_membership_fee(&self, fee: &CreateMembershipFee) -> Result<MembershipFee> {
        let id = sqlx::query(
            "INSERT INTO membership_fees (vuosi, jasentyyppi, summa)
             VALUES (?, ?, ?)"
        )
        .bind(fee.vuosi)
        .bind(&fee.jasentyyppi.to_string())
        .bind(fee.summa)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(MembershipFee {
            id,
            vuosi: fee.vuosi,
            jasentyyppi: fee.jasentyyppi.to_string(),
            summa: fee.summa,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    pub async fn update_membership_fee(&self, id: i64, fee: &CreateMembershipFee) -> Result<MembershipFee> {
        sqlx::query(
            "UPDATE membership_fees SET summa = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(fee.summa)
        .bind(id)
        .execute(&self.pool)
        .await?;

        // Hae päivitetty tietue
        let row = sqlx::query("SELECT * FROM membership_fees WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        let summa = match row.try_get::<f64, _>("summa") {
            Ok(val) => val,
            Err(_) => row.get::<i64, _>("summa") as f64,
        };

        Ok(MembershipFee {
            id: row.get("id"),
            vuosi: row.get("vuosi"),
            jasentyyppi: row.get("jasentyyppi"),
            summa,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_invoices(&self) -> Result<Vec<InvoiceWithDetails>> {
        let rows = sqlx::query(
            "SELECT 
                i.id as invoice_id, i.talous_id, i.luontipaiva, i.erapaiva, i.summa, 
                i.viitenumero, i.laskunumero, i.maksettu, i.maksupaiva, 
                i.created_at as invoice_created_at, i.updated_at as invoice_updated_at,
                h.id as household_id, h.talouden_nimi, h.vastaanottaja, h.laskutusosoite_sama, h.laskutusosoite_id,
                h.created_at as household_created_at, h.updated_at as household_updated_at,
                a.id as address_id, a.katuosoite, a.postinumero, a.postitoimipaikka,
                a.created_at as address_created_at, a.updated_at as address_updated_at
            FROM invoices i 
            JOIN households h ON i.talous_id = h.id
            JOIN addresses a ON h.id = a.talous_id
            ORDER BY i.luontipaiva DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut invoices_with_details = Vec::new();
        
        for row in rows {
            let summa = match row.try_get::<f64, _>("summa") {
                Ok(val) => val,
                Err(_) => row.get::<i64, _>("summa") as f64,
            };

            let invoice = Invoice {
                id: row.get("invoice_id"),
                talous_id: row.get("talous_id"),
                luontipaiva: row.get("luontipaiva"),
                erapaiva: row.get("erapaiva"),
                summa,
                viitenumero: row.get("viitenumero"),
                laskunumero: row.try_get("laskunumero").ok().flatten(),
                maksettu: row.get("maksettu"),
                maksupaiva: row.get("maksupaiva"),
                created_at: row.get("invoice_created_at"),
                updated_at: row.get("invoice_updated_at"),
            };

            let household = Household {
                id: row.get("household_id"),
                talouden_nimi: row.get("talouden_nimi"),
                vastaanottaja: row.try_get("vastaanottaja").ok().flatten(),
                laskutusosoite_sama: row.get("laskutusosoite_sama"),
                laskutusosoite_id: row.get("laskutusosoite_id"),
                created_at: row.get("household_created_at"),
                updated_at: row.get("household_updated_at"),
            };

            let address = Address {
                id: row.get("address_id"),
                katuosoite: row.get("katuosoite"),
                postinumero: row.get("postinumero"),
                postitoimipaikka: row.get("postitoimipaikka"),
                talous_id: row.get("talous_id"),
                created_at: row.get("address_created_at"),
                updated_at: row.get("address_updated_at"),
            };

            // Hae laskurivit tälle laskulle
            let line_rows = sqlx::query(
                "SELECT 
                    il.id, il.lasku_id, il.jasen_id, il.kuvaus, il.summa, il.created_at,
                    m.id as member_id, m.etunimi, m.sukunimi, m.henkilotunnus, m.syntymaaika,
                    m.puhelinnumero, m.sahkoposti, m.osoite_id, m.liittymispaiva, m.jasentyyppi,
                    m.aktiivinen, m.created_at as member_created_at, m.updated_at as member_updated_at
                FROM invoice_lines il
                JOIN members m ON il.jasen_id = m.id
                WHERE il.lasku_id = ?
                ORDER BY m.sukunimi, m.etunimi"
            )
            .bind(invoice.id)
            .fetch_all(&self.pool)
            .await?;

            let mut lines = Vec::new();
            for line_row in line_rows {
                let line_summa = match line_row.try_get::<f64, _>("summa") {
                    Ok(val) => val,
                    Err(_) => line_row.get::<i64, _>("summa") as f64,
                };

                let line = InvoiceLine {
                    id: line_row.get("id"),
                    lasku_id: line_row.get("lasku_id"),
                    jasen_id: line_row.get("jasen_id"),
                    kuvaus: line_row.get("kuvaus"),
                    summa: line_summa,
                    created_at: line_row.get("created_at"),
                };

                let member = Member {
                    id: line_row.get("member_id"),
                    etunimi: line_row.get("etunimi"),
                    sukunimi: line_row.get("sukunimi"),
                    henkilotunnus: line_row.get("henkilotunnus"),
                    syntymaaika: line_row.get("syntymaaika"),
                    puhelinnumero: line_row.get("puhelinnumero"),
                    sahkoposti: line_row.get("sahkoposti"),
                    osoite_id: line_row.get("osoite_id"),
                    liittymispaiva: line_row.get("liittymispaiva"),
                    jasentyyppi: line_row.get("jasentyyppi"),
                    aktiivinen: line_row.get("aktiivinen"),
                    created_at: line_row.get("member_created_at"),
                    updated_at: line_row.get("member_updated_at"),
                };

                lines.push(InvoiceLineWithMember { line, member });
            }

            invoices_with_details.push(InvoiceWithDetails {
                invoice,
                household,
                address,
                lines,
                billing_address: None, // TODO: Jos laskutusosoite on eri
            });
        }

        Ok(invoices_with_details)
    }

    pub async fn validate_invoice_creation(&self, year: i32) -> Result<String> {
        // Tarkista että jäsenmaksut on määritelty kaikille jäsentyypeille
        let member_types = sqlx::query(
            "SELECT DISTINCT jasentyyppi FROM members WHERE aktiivinen = 1"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut missing_fees = Vec::new();
        for type_row in member_types {
            let member_type: String = type_row.get("jasentyyppi");
            let fee_exists = sqlx::query(
                "SELECT COUNT(*) as count FROM membership_fees WHERE jasentyyppi = ? AND vuosi = ?"
            )
            .bind(&member_type)
            .bind(year)
            .fetch_one(&self.pool)
            .await?
            .get::<i64, _>("count") > 0;

            if !fee_exists {
                missing_fees.push(member_type);
            }
        }

        if !missing_fees.is_empty() {
            return Err(anyhow::anyhow!(
                "Jäsenmaksut puuttuvat vuodelle {} jäsentyypeille: {}. Määrittele jäsenmaksut ennen laskujen luontia.",
                year,
                missing_fees.join(", ")
            ));
        }

        // Tarkista että on aktiivisia jäseniä
        let active_members = sqlx::query(
            "SELECT COUNT(*) as count FROM members WHERE aktiivinen = 1"
        )
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("count");

        if active_members == 0 {
            return Err(anyhow::anyhow!("Ei aktiivisia jäseniä laskutettavaksi."));
        }

        // Laske montako laskua luotaisiin (vain ne joilla ei ole jo laskua)
        let households_without_invoice = sqlx::query(
            "SELECT COUNT(DISTINCT h.id) as count
             FROM households h
             JOIN addresses a ON h.id = a.talous_id
             JOIN members m ON m.osoite_id = a.id
             JOIN membership_fees mf ON m.jasentyyppi = mf.jasentyyppi AND mf.vuosi = ?
             WHERE m.aktiivinen = 1
             AND NOT EXISTS (
                 SELECT 1 FROM invoices i 
                 WHERE i.talous_id = h.id 
                 AND strftime('%Y', i.luontipaiva) = CAST(? as TEXT)
             )"
        )
        .bind(year)
        .bind(year)
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("count");

        // Tarkista onko jo kaikille luotu laskut
        let existing_invoices = sqlx::query(
            "SELECT COUNT(*) as count FROM invoices 
             WHERE strftime('%Y', luontipaiva) = CAST(? as TEXT)"
        )
        .bind(year)
        .fetch_one(&self.pool)
        .await?
        .get::<i64, _>("count");

        if households_without_invoice == 0 {
            if existing_invoices > 0 {
                return Err(anyhow::anyhow!(
                    "Laskut on jo luotu vuodelle {}. {} laskua olemassa.",
                    year, existing_invoices
                ));
            } else {
                return Err(anyhow::anyhow!("Ei laskutettavia jäseniä vuodelle {}.", year));
            }
        }

        let message = if existing_invoices > 0 {
            format!(
                "Valmis luomaan {} uutta laskua vuodelle {}. {} laskua on jo olemassa. Luodaan laskut vain niille jäsenille, joilla ei ole vielä laskua.",
                households_without_invoice, year, existing_invoices
            )
        } else {
            format!(
                "Valmis luomaan {} laskua {} aktiiviselle jäsenelle vuodelle {}.",
                households_without_invoice, active_members, year
            )
        };

        Ok(message)
    }

    pub async fn create_invoice_for_year(&self, year: i32) -> Result<Vec<Invoice>> {

        // Hae vain ne taloudet joilla ei ole vielä laskua tälle vuodelle
        let households = sqlx::query(
            "SELECT DISTINCT h.id as household_id, h.talouden_nimi, h.vastaanottaja
             FROM households h
             JOIN addresses a ON h.id = a.talous_id
             JOIN members m ON m.osoite_id = a.id
             JOIN membership_fees mf ON m.jasentyyppi = mf.jasentyyppi AND mf.vuosi = ?
             WHERE m.aktiivinen = 1
             AND NOT EXISTS (
                 SELECT 1 FROM invoices i 
                 WHERE i.talous_id = h.id 
                 AND strftime('%Y', i.luontipaiva) = CAST(? as TEXT)
             )"
        )
        .bind(year)
        .bind(year)
        .fetch_all(&self.pool)
        .await?;

        let mut created_invoices = Vec::new();
        let current_date = chrono::Utc::now().date_naive();
        let due_date = current_date + chrono::Duration::days(30);

        for household_row in households {
            let household_id: i64 = household_row.get("household_id");
            
            // Hae jäsenmaksut tälle vuodelle
            let members = sqlx::query(
                "SELECT m.id, m.etunimi, m.sukunimi, m.jasentyyppi, mf.summa
                 FROM members m
                 JOIN addresses a ON m.osoite_id = a.id
                 JOIN households h ON a.talous_id = h.id
                 JOIN membership_fees mf ON m.jasentyyppi = mf.jasentyyppi AND mf.vuosi = ?
                 WHERE h.id = ? AND m.aktiivinen = 1"
            )
            .bind(year)
            .bind(household_id)
            .fetch_all(&self.pool)
            .await?;

            // Laske kokonaissumma
            let total_sum: f64 = members.iter()
                .map(|row| {
                    // Kokeile ensin f64, sitten i64 ja muunna
                    match row.try_get::<f64, _>("summa") {
                        Ok(val) => val,
                        Err(_) => row.get::<i64, _>("summa") as f64,
                    }
                })
                .sum();

            if total_sum <= 0.0 {
                continue; // Ei laskutettavaa
            }

            // Generoi viitenumero
            let reference_number = format!("{}{:05}", year, household_id);
            let invoice_number = format!("{}-{:03}", year, household_id);

            // Luo lasku
            let invoice_id = sqlx::query(
                "INSERT INTO invoices (talous_id, luontipaiva, erapaiva, summa, viitenumero, laskunumero, maksettu)
                 VALUES (?, ?, ?, ?, ?, ?, 0)"
            )
            .bind(household_id)
            .bind(current_date)
            .bind(due_date)
            .bind(total_sum)
            .bind(&reference_number)
            .bind(&invoice_number)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

            // Luo laskurivit jokaiselle jäsenelle
            for member_row in &members {
                let member_id: i64 = member_row.get("id");
                let member_fee: f64 = match member_row.try_get::<f64, _>("summa") {
                    Ok(val) => val,
                    Err(_) => member_row.get::<i64, _>("summa") as f64,
                };
                let etunimi: String = member_row.get("etunimi");
                let sukunimi: String = member_row.get("sukunimi");
                let description = format!("Jäsenmaksu {} - {} {}", year, etunimi, sukunimi);

                sqlx::query(
                    "INSERT INTO invoice_lines (lasku_id, jasen_id, kuvaus, summa)
                     VALUES (?, ?, ?, ?)"
                )
                .bind(invoice_id)
                .bind(member_id)
                .bind(&description)
                .bind(member_fee)
                .execute(&self.pool)
                .await?;
            }

            // Luo Invoice-objekti palautusta varten
            let invoice = Invoice {
                id: invoice_id,
                talous_id: household_id,
                luontipaiva: current_date,
                erapaiva: due_date,
                summa: total_sum,
                viitenumero: reference_number,
                laskunumero: Some(invoice_number),
                maksettu: false,
                maksupaiva: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            created_invoices.push(invoice);
        }

        Ok(created_invoices)
    }

    pub async fn mark_invoice_paid(&self, id: i64, payment_date: chrono::NaiveDate) -> Result<Invoice> {
        // Päivitä lasku maksetuksi
        sqlx::query(
            "UPDATE invoices SET maksettu = 1, maksupaiva = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(payment_date)
        .bind(id)
        .execute(&self.pool)
        .await?;

        // Hae päivitetty lasku
        let row = sqlx::query("SELECT * FROM invoices WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Invoice {
            id: row.get("id"),
            talous_id: row.get("talous_id"),
            luontipaiva: row.get("luontipaiva"),
            erapaiva: row.get("erapaiva"),
            summa: row.try_get::<f64, _>("summa").unwrap_or_else(|_| {
                row.get::<i64, _>("summa") as f64
            }),
            viitenumero: row.get("viitenumero"),
            laskunumero: row.get("laskunumero"),
            maksettu: row.get::<i64, _>("maksettu") != 0,
            maksupaiva: row.get("maksupaiva"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn delete_invoice(&self, id: i64) -> Result<()> {
        // Poista ensin laskurivit (foreign key constraint)
        sqlx::query("DELETE FROM invoice_lines WHERE lasku_id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        // Poista lasku
        sqlx::query("DELETE FROM invoices WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn delete_membership_fee(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM membership_fees WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_address(&self, address_id: i64, katuosoite: &str, postinumero: &str, postitoimipaikka: &str) -> Result<()> {
        sqlx::query(
            "UPDATE addresses SET katuosoite = ?, postinumero = ?, postitoimipaikka = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
        )
        .bind(katuosoite)
        .bind(postinumero)
        .bind(postitoimipaikka)
        .bind(address_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_member_address_id(&self, member_id: i64) -> Result<i64> {
        let row = sqlx::query("SELECT osoite_id FROM members WHERE id = ?")
            .bind(member_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get("osoite_id"))
    }

    pub async fn update_household(&self, id: i64, household: &CreateHousehold) -> Result<Household> {
        sqlx::query(
            "UPDATE households SET 
             talouden_nimi = ?, vastaanottaja = ?, laskutusosoite_sama = ?, laskutusosoite_id = ?, 
             updated_at = CURRENT_TIMESTAMP
             WHERE id = ?"
        )
        .bind(&household.talouden_nimi)
        .bind(&household.vastaanottaja)
        .bind(household.laskutusosoite_sama)
        .bind(household.laskutusosoite_id)
        .bind(id)
        .execute(&self.pool)
        .await?;

        // Fetch and return the updated household
        let row = sqlx::query("SELECT * FROM households WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(Household {
            id: row.get("id"),
            talouden_nimi: row.get("talouden_nimi"),
            vastaanottaja: row.try_get("vastaanottaja").ok(),
            laskutusosoite_sama: row.get("laskutusosoite_sama"),
            laskutusosoite_id: row.get("laskutusosoite_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn update_household_address(&self, household_id: i64, katuosoite: &str, postinumero: &str, postitoimipaikka: &str) -> Result<()> {
        sqlx::query(
            "UPDATE addresses SET 
             katuosoite = ?, postinumero = ?, postitoimipaikka = ?, 
             updated_at = CURRENT_TIMESTAMP 
             WHERE talous_id = ?"
        )
        .bind(katuosoite)
        .bind(postinumero)
        .bind(postitoimipaikka)
        .bind(household_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_household(&self, id: i64) -> Result<()> {
        // First, delete all members in this household
        sqlx::query("DELETE FROM members WHERE osoite_id IN (SELECT id FROM addresses WHERE talous_id = ?)")
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Delete the address
        sqlx::query("DELETE FROM addresses WHERE talous_id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        // Delete the household
        sqlx::query("DELETE FROM households WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_household_member_count(&self, household_id: i64) -> Result<i64> {
        let row = sqlx::query(
            "SELECT COUNT(*) as count FROM members m 
             JOIN addresses a ON m.osoite_id = a.id 
             WHERE a.talous_id = ?"
        )
        .bind(household_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get("count"))
    }

    // Tilastofunktiot Dashboard:ia varten
    pub async fn get_total_members(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM members WHERE aktiivinen = 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get("count"))
    }

    pub async fn get_open_invoices_count(&self) -> Result<i64> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM invoices WHERE maksettu = 0")
            .fetch_one(&self.pool)
            .await?;
        Ok(row.get("count"))
    }

    pub async fn get_total_receivables(&self) -> Result<f64> {
        let row = sqlx::query("SELECT COALESCE(SUM(summa), 0) as total FROM invoices WHERE maksettu = 0")
            .fetch_one(&self.pool)
            .await?;
        
        // Handle both f64 and i64 types from SQLite
        let total = row.try_get::<f64, _>("total").unwrap_or_else(|_| {
            row.get::<i64, _>("total") as f64
        });
        Ok(total)
    }

    pub async fn get_yearly_income(&self, year: i32) -> Result<f64> {
        let row = sqlx::query(
            "SELECT COALESCE(SUM(summa), 0) as total FROM invoices 
             WHERE maksettu = 1 AND strftime('%Y', maksupaiva) = ?"
        )
        .bind(year.to_string())
        .fetch_one(&self.pool)
        .await?;
        
        // Handle both f64 and i64 types from SQLite
        let total = row.try_get::<f64, _>("total").unwrap_or_else(|_| {
            row.get::<i64, _>("total") as f64
        });
        Ok(total)
    }
}