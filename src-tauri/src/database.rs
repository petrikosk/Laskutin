use crate::models::*;
use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
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

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Database { pool })
    }

    // Organization methods
    pub async fn get_organization(&self) -> Result<Option<Organization>> {
        let org = sqlx::query_as!(Organization, "SELECT * FROM organization LIMIT 1")
            .fetch_optional(&self.pool)
            .await?;

        Ok(org)
    }

    pub async fn update_organization(&self, org: &CreateOrganization) -> Result<Organization> {
        let existing = self.get_organization().await?;

        if let Some(existing_org) = existing {
            let updated = sqlx::query_as!(
                Organization,
                r#"
                UPDATE organization 
                SET nimi = ?, katuosoite = ?, postinumero = ?, postitoimipaikka = ?,
                    puhelinnumero = ?, sahkoposti = ?, y_tunnus = ?, pankkitili = ?, bic = ?,
                    updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                RETURNING *
                "#,
                org.nimi,
                org.katuosoite,
                org.postinumero,
                org.postitoimipaikka,
                org.puhelinnumero,
                org.sahkoposti,
                org.y_tunnus,
                org.pankkitili,
                org.bic,
                existing_org.id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(updated)
        } else {
            let created = sqlx::query_as!(
                Organization,
                r#"
                INSERT INTO organization (nimi, katuosoite, postinumero, postitoimipaikka, 
                                        puhelinnumero, sahkoposti, y_tunnus, pankkitili, bic)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                RETURNING *
                "#,
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
            .fetch_one(&self.pool)
            .await?;

            Ok(created)
        }
    }

    // Member methods
    pub async fn get_members(&self) -> Result<Vec<MemberWithAddress>> {
        let rows = sqlx::query!(
            r#"
            SELECT m.*, a.*, h.*,
                   m.id as member_id, a.id as address_id, h.id as household_id
            FROM members m
            JOIN addresses a ON m.osoite_id = a.id
            JOIN households h ON a.talous_id = h.id
            ORDER BY m.sukunimi, m.etunimi
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for row in rows {
            let member = Member {
                id: row.member_id,
                etunimi: row.etunimi,
                sukunimi: row.sukunimi,
                henkilotunnus: row.henkilotunnus,
                syntymaaika: row.syntymaaika,
                puhelinnumero: row.puhelinnumero,
                sahkoposti: row.sahkoposti,
                osoite_id: row.osoite_id,
                liittymispaiva: row.liittymispaiva,
                jasentyyppi: row.jasentyyppi,
                aktiivinen: row.aktiivinen,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            let address = Address {
                id: row.address_id,
                katuosoite: row.katuosoite,
                postinumero: row.postinumero,
                postitoimipaikka: row.postitoimipaikka,
                talous_id: row.talous_id,
                created_at: row.created_at_1,
                updated_at: row.updated_at_1,
            };

            let household = Household {
                id: row.household_id,
                talouden_nimi: row.talouden_nimi,
                laskutusosoite_sama: row.laskutusosoite_sama,
                laskutusosoite_id: row.laskutusosoite_id,
                created_at: row.created_at_2,
                updated_at: row.updated_at_2,
            };

            result.push(MemberWithAddress {
                member,
                address,
                household,
            });
        }

        Ok(result)
    }

    pub async fn create_member(&self, member: &CreateMember) -> Result<Member> {
        let created = sqlx::query_as!(
            Member,
            r#"
            INSERT INTO members (etunimi, sukunimi, henkilotunnus, syntymaaika, 
                               puhelinnumero, sahkoposti, osoite_id, liittymispaiva, 
                               jasentyyppi, aktiivinen)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
            member.etunimi,
            member.sukunimi,
            member.henkilotunnus,
            member.syntymaaika,
            member.puhelinnumero,
            member.sahkoposti,
            member.osoite_id,
            member.liittymispaiva,
            member.jasentyyppi.to_string(),
            member.aktiivinen
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    pub async fn update_member(&self, id: i64, member: &CreateMember) -> Result<Member> {
        let updated = sqlx::query_as!(
            Member,
            r#"
            UPDATE members 
            SET etunimi = ?, sukunimi = ?, henkilotunnus = ?, syntymaaika = ?,
                puhelinnumero = ?, sahkoposti = ?, osoite_id = ?, liittymispaiva = ?,
                jasentyyppi = ?, aktiivinen = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#,
            member.etunimi,
            member.sukunimi,
            member.henkilotunnus,
            member.syntymaaika,
            member.puhelinnumero,
            member.sahkoposti,
            member.osoite_id,
            member.liittymispaiva,
            member.jasentyyppi.to_string(),
            member.aktiivinen,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    pub async fn delete_member(&self, id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM members WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Household methods
    pub async fn get_households(&self) -> Result<Vec<Household>> {
        let households =
            sqlx::query_as!(Household, "SELECT * FROM households ORDER BY talouden_nimi")
                .fetch_all(&self.pool)
                .await?;

        Ok(households)
    }

    pub async fn create_household(&self, household: &CreateHousehold) -> Result<Household> {
        let created = sqlx::query_as!(
            Household,
            r#"
            INSERT INTO households (talouden_nimi, laskutusosoite_sama, laskutusosoite_id)
            VALUES (?, ?, ?)
            RETURNING *
            "#,
            household.talouden_nimi,
            household.laskutusosoite_sama,
            household.laskutusosoite_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    // Membership fee methods
    pub async fn get_membership_fees(&self) -> Result<Vec<MembershipFee>> {
        let fees = sqlx::query_as!(
            MembershipFee,
            "SELECT * FROM membership_fees ORDER BY vuosi DESC, jasentyyppi"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(fees)
    }

    pub async fn create_membership_fee(&self, fee: &CreateMembershipFee) -> Result<MembershipFee> {
        let created = sqlx::query_as!(
            MembershipFee,
            r#"
            INSERT INTO membership_fees (vuosi, jasentyyppi, summa)
            VALUES (?, ?, ?)
            RETURNING *
            "#,
            fee.vuosi,
            fee.jasentyyppi.to_string(),
            fee.summa
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(created)
    }

    pub async fn update_membership_fee(
        &self,
        id: i64,
        fee: &CreateMembershipFee,
    ) -> Result<MembershipFee> {
        let updated = sqlx::query_as!(
            MembershipFee,
            r#"
            UPDATE membership_fees 
            SET vuosi = ?, jasentyyppi = ?, summa = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#,
            fee.vuosi,
            fee.jasentyyppi.to_string(),
            fee.summa,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    // Invoice methods
    pub async fn get_invoices(&self) -> Result<Vec<InvoiceWithDetails>> {
        let invoices = sqlx::query_as!(Invoice, "SELECT * FROM invoices ORDER BY luontipaiva DESC")
            .fetch_all(&self.pool)
            .await?;

        let mut result = Vec::new();
        for invoice in invoices {
            let details = self.get_invoice_details(&invoice).await?;
            result.push(details);
        }

        Ok(result)
    }

    async fn get_invoice_details(&self, invoice: &Invoice) -> Result<InvoiceWithDetails> {
        // Get household and address
        let household_row =
            sqlx::query!("SELECT * FROM households WHERE id = ?", invoice.talous_id)
                .fetch_one(&self.pool)
                .await?;

        let household = Household {
            id: household_row.id,
            talouden_nimi: household_row.talouden_nimi,
            laskutusosoite_sama: household_row.laskutusosoite_sama,
            laskutusosoite_id: household_row.laskutusosoite_id,
            created_at: household_row.created_at,
            updated_at: household_row.updated_at,
        };

        // Get main address
        let address_row = sqlx::query!(
            "SELECT * FROM addresses WHERE talous_id = ?",
            invoice.talous_id
        )
        .fetch_one(&self.pool)
        .await?;

        let address = Address {
            id: address_row.id,
            katuosoite: address_row.katuosoite,
            postinumero: address_row.postinumero,
            postitoimipaikka: address_row.postitoimipaikka,
            talous_id: address_row.talous_id,
            created_at: address_row.created_at,
            updated_at: address_row.updated_at,
        };

        // Get billing address if different
        let billing_address = if let Some(billing_id) = household.laskutusosoite_id {
            if !household.laskutusosoite_sama {
                let billing_row = sqlx::query!("SELECT * FROM addresses WHERE id = ?", billing_id)
                    .fetch_optional(&self.pool)
                    .await?;

                if let Some(row) = billing_row {
                    Some(Address {
                        id: row.id,
                        katuosoite: row.katuosoite,
                        postinumero: row.postinumero,
                        postitoimipaikka: row.postitoimipaikka,
                        talous_id: row.talous_id,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Get invoice lines with member details
        let line_rows = sqlx::query!(
            r#"
            SELECT il.*, m.*,
                   il.id as line_id, m.id as member_id
            FROM invoice_lines il
            JOIN members m ON il.jasen_id = m.id
            WHERE il.lasku_id = ?
            "#,
            invoice.id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut lines = Vec::new();
        for row in line_rows {
            let line = InvoiceLine {
                id: row.line_id,
                lasku_id: row.lasku_id,
                jasen_id: row.jasen_id,
                kuvaus: row.kuvaus,
                summa: row.summa,
                created_at: row.created_at,
            };

            let member = Member {
                id: row.member_id,
                etunimi: row.etunimi,
                sukunimi: row.sukunimi,
                henkilotunnus: row.henkilotunnus,
                syntymaaika: row.syntymaaika,
                puhelinnumero: row.puhelinnumero,
                sahkoposti: row.sahkoposti,
                osoite_id: row.osoite_id,
                liittymispaiva: row.liittymispaiva,
                jasentyyppi: row.jasentyyppi,
                aktiivinen: row.aktiivinen,
                created_at: row.created_at_1,
                updated_at: row.updated_at,
            };

            lines.push(InvoiceLineWithMember { line, member });
        }

        Ok(InvoiceWithDetails {
            invoice: invoice.clone(),
            household,
            address,
            lines,
            billing_address,
        })
    }

    pub async fn create_invoice_for_year(&self, year: i32) -> Result<Vec<Invoice>> {
        // Get all active members grouped by household
        let members = sqlx::query!(
            r#"
            SELECT m.*, a.talous_id, mf.summa as fee_amount
            FROM members m
            JOIN addresses a ON m.osoite_id = a.id
            JOIN membership_fees mf ON mf.vuosi = ? AND mf.jasentyyppi = m.jasentyyppi
            WHERE m.aktiivinen = 1
            ORDER BY a.talous_id, m.id
            "#,
            year
        )
        .fetch_all(&self.pool)
        .await?;

        // Group by household
        let mut households: std::collections::HashMap<i64, Vec<_>> =
            std::collections::HashMap::new();
        for member in members {
            households
                .entry(member.talous_id)
                .or_insert_with(Vec::new)
                .push(member);
        }

        let mut created_invoices = Vec::new();

        for (talous_id, household_members) in households {
            let total_amount: f64 = household_members.iter().map(|m| m.fee_amount).sum();

            let reference_number = self.generate_reference_number().await?;
            let due_date = chrono::Local::now().naive_local().date() + chrono::Duration::days(30);

            // Create invoice
            let invoice = sqlx::query_as!(
                Invoice,
                r#"
                INSERT INTO invoices (talous_id, luontipaiva, erapaiva, summa, viitenumero)
                VALUES (?, date('now'), ?, ?, ?)
                RETURNING *
                "#,
                talous_id,
                due_date,
                total_amount,
                reference_number
            )
            .fetch_one(&self.pool)
            .await?;

            // Create invoice lines
            for member in household_members {
                sqlx::query!(
                    r#"
                    INSERT INTO invoice_lines (lasku_id, jasen_id, kuvaus, summa)
                    VALUES (?, ?, ?, ?)
                    "#,
                    invoice.id,
                    member.id,
                    format!("Jäsenmaksu {} - {}", year, member.jasentyyppi),
                    member.fee_amount
                )
                .execute(&self.pool)
                .await?;
            }

            created_invoices.push(invoice);
        }

        Ok(created_invoices)
    }

    pub async fn mark_invoice_paid(
        &self,
        id: i64,
        payment_date: chrono::NaiveDate,
    ) -> Result<Invoice> {
        let updated = sqlx::query_as!(
            Invoice,
            r#"
            UPDATE invoices 
            SET maksettu = 1, maksupaiva = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *
            "#,
            payment_date,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    async fn generate_reference_number(&self) -> Result<String> {
        let count = sqlx::query!("SELECT COUNT(*) as count FROM invoices")
            .fetch_one(&self.pool)
            .await?;

        let base_number = format!(
            "{}{:04}",
            chrono::Local::now().format("%Y"),
            count.count + 1
        );

        // Calculate check digit using Finnish reference number algorithm
        let check_digit = self.calculate_check_digit(&base_number);

        Ok(format!("{}{}", base_number, check_digit))
    }

    fn calculate_check_digit(&self, number: &str) -> u8 {
        let weights = [7, 3, 1];
        let sum: u32 = number
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let digit = c.to_digit(10).unwrap_or(0);
                digit * weights[i % 3]
            })
            .sum();

        let remainder = sum % 10;
        if remainder == 0 {
            0
        } else {
            (10 - remainder) as u8
        }
    }
}
