use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: i64,
    pub nimi: String,
    pub katuosoite: String,
    pub postinumero: String,
    pub postitoimipaikka: String,
    pub puhelinnumero: Option<String>,
    pub sahkoposti: Option<String>,
    pub y_tunnus: Option<String>,
    pub pankkitili: Option<String>,
    pub bic: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganization {
    pub nimi: String,
    pub katuosoite: String,
    pub postinumero: String,
    pub postitoimipaikka: String,
    pub puhelinnumero: Option<String>,
    pub sahkoposti: Option<String>,
    pub y_tunnus: Option<String>,
    pub pankkitili: Option<String>,
    pub bic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Household {
    pub id: i64,
    pub talouden_nimi: Option<String>,
    pub laskutusosoite_sama: bool,
    pub laskutusosoite_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHousehold {
    pub talouden_nimi: Option<String>,
    pub laskutusosoite_sama: bool,
    pub laskutusosoite_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Address {
    pub id: i64,
    pub katuosoite: String,
    pub postinumero: String,
    pub postitoimipaikka: String,
    pub talous_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAddress {
    pub katuosoite: String,
    pub postinumero: String,
    pub postitoimipaikka: String,
    pub talous_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MemberType {
    Varsinainen,
    Kannatus,
    Kunnia,
}

impl ToString for MemberType {
    fn to_string(&self) -> String {
        match self {
            MemberType::Varsinainen => "varsinainen".to_string(),
            MemberType::Kannatus => "kannatus".to_string(),
            MemberType::Kunnia => "kunnia".to_string(),
        }
    }
}

impl std::str::FromStr for MemberType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "varsinainen" => Ok(MemberType::Varsinainen),
            "kannatus" => Ok(MemberType::Kannatus),
            "kunnia" => Ok(MemberType::Kunnia),
            _ => Err(format!("Invalid member type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Member {
    pub id: i64,
    pub etunimi: String,
    pub sukunimi: String,
    pub henkilotunnus: Option<String>,
    pub syntymaaika: Option<NaiveDate>,
    pub puhelinnumero: Option<String>,
    pub sahkoposti: Option<String>,
    pub osoite_id: i64,
    pub liittymispaiva: NaiveDate,
    pub jasentyyppi: String,
    pub aktiivinen: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Member {
    pub fn get_member_type(&self) -> Result<MemberType, String> {
        self.jasentyyppi.parse()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMember {
    pub etunimi: String,
    pub sukunimi: String,
    pub henkilotunnus: Option<String>,
    pub syntymaaika: Option<NaiveDate>,
    pub puhelinnumero: Option<String>,
    pub sahkoposti: Option<String>,
    pub osoite_id: i64,
    pub liittymispaiva: NaiveDate,
    pub jasentyyppi: MemberType,
    pub aktiivinen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MembershipFee {
    pub id: i64,
    pub vuosi: i32,
    pub jasentyyppi: String,
    pub summa: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MembershipFee {
    pub fn get_member_type(&self) -> Result<MemberType, String> {
        self.jasentyyppi.parse()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMembershipFee {
    pub vuosi: i32,
    pub jasentyyppi: MemberType,
    pub summa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: i64,
    pub talous_id: i64,
    pub luontipaiva: NaiveDate,
    pub erapaiva: NaiveDate,
    pub summa: f64,
    pub viitenumero: String,
    pub maksettu: bool,
    pub maksupaiva: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoice {
    pub talous_id: i64,
    pub luontipaiva: NaiveDate,
    pub erapaiva: NaiveDate,
    pub summa: f64,
    pub viitenumero: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InvoiceLine {
    pub id: i64,
    pub lasku_id: i64,
    pub jasen_id: i64,
    pub kuvaus: String,
    pub summa: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceLine {
    pub lasku_id: i64,
    pub jasen_id: i64,
    pub kuvaus: String,
    pub summa: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceWithDetails {
    pub invoice: Invoice,
    pub household: Household,
    pub address: Address,
    pub lines: Vec<InvoiceLineWithMember>,
    pub billing_address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineWithMember {
    pub line: InvoiceLine,
    pub member: Member,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberWithAddress {
    pub member: Member,
    pub address: Address,
    pub household: Household,
}