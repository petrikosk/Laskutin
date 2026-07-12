# Laskutin - Jäsenmaksulaskutusohjelma

Tauri-pohjainen jäsenmaksulaskutusohjelma SQLite-tietokannalla ja Vue.js-käyttöliittymällä.

## Teknologiat
- **Backend**: Rust + Tauri 2
- **Frontend**: Vue.js 3 + TypeScript + Custom CSS (ei Tailwind)
- **Tietokanta**: SQLite
- **Paketinhallinta**: pnpm
- **Build system**: Vite
- **PDF-generointi**: jsPDF (vektoripohjainen, src/utils/vectorPdfGenerator.ts)
- **Viivakoodi**: JsBarcode
- **Päivämääräpoiminta**: @vuepic/vue-datepicker

## Tietokantarakenne

### Yhdistyksen tiedot (organization)
- id (PK)
- nimi
- katuosoite
- postinumero
- postitoimipaikka
- puhelinnumero
- sahkoposti
- y_tunnus
- pankkitili
- bic

### Taloudet (households)
- id (PK)
- talouden_nimi (vapaaehtoinen)
- **vastaanottaja** (laskun saajan nimi)
- laskutusosoite_sama (boolean)
- laskutusosoite_id (FK, nullable)

### Osoitteet (addresses)
- id (PK)
- katuosoite
- postinumero
- postitoimipaikka
- talous_id (FK)

### Jäsenet (members)
- id (PK)
- etunimi
- sukunimi
- henkilotunnus (vapaaehtoinen)
- **syntymaaika** (pakollinen)
- puhelinnumero
- sahkoposti
- osoite_id (FK)
- liittymispaiva
- jasentyyppi (Varsinainen/Kannatus/Kunnia)
- aktiivinen (boolean)

### Jäsenmaksut (membership_fees)
- id (PK)
- vuosi
- jasentyyppi
- summa (DECIMAL tallentuu SQLite:ssa INTEGER:nä)

### Laskut (invoices)
- id (PK)
- talous_id (FK)
- luontipaiva
- erapaiva
- summa (DECIMAL tallentuu SQLite:ssa INTEGER:nä)
- viitenumero
- **laskunumero** (erillinen kenttä)
- maksettu (boolean)
- maksupaiva (nullable)

### Laskurivit (invoice_lines)
- id (PK)
- lasku_id (FK)
- jasen_id (FK)
- kuvaus
- summa (DECIMAL tallentuu SQLite:ssa INTEGER:nä)

## Toiminnallisuus
- ✅ Jäsentietojen hallinta (CRUD)
- ✅ Osoitteiden ja talouksien hallinta (CRUD)
- ✅ Jäsenmaksujen määrittely vuosittain
- ✅ Laskujen luonti (1 lasku per talous)
- ✅ PDF-laskutulostus suomalaisella layoutilla
- ✅ Pankkiviivakoodin generointi
- ✅ Natiivi tiedostojen tallennus
- ✅ Maksujen seuranta
- ✅ Automaattinen tyhjien talouksien siivous

## Komennot
```bash
# Kehitys
npm run tauri dev

# Build
npm run tauri build

# Frontend kehitys
npm run dev

# Backend build
cd src-tauri && cargo build

# Tietokanta migraatiot (automaattisesti)
# migrations/001_initial.sql
# migrations/002_add_fields.sql
```

## Projektin rakenne
- `src-tauri/` - Rust backend
  - `src/models.rs` - Tietokantamallit
  - `src/database_simple.rs` - Tietokantaoperaatiot
  - `src/commands.rs` - Tauri-komennot frontend:lle
  - `migrations/` - SQL-migraatiot
- `src/` - Vue.js frontend
  - `components/` - Vue-komponentit
  - `utils/pdfGenerator.ts` - PDF-generointi
- `public/` - Staattiset tiedostot

## Tärkeät toteutetut ominaisuudet

### PDF-tulostus
- Finnish invoice layout with proper formatting
- Banking barcode generation (CODE128)
- Native file save dialogs via backend commands
- A4 page size with proper scaling
- Print functionality

### Jäsenten hallinta
- Flexible address/household management:
  - **'oma'**: Create new household + address for single person
  - **'talous'**: Join existing household
  - **'uusi'**: Create new household that others can join
- Automatic cleanup when households become empty
- Finnish date picker with proper DD.MM.YYYY formatting
- Member type validation (Varsinainen/Nuorisojasen/Kannatus/Kunnia)
- Automatic age-based member type conversion from Nuorisojasen to Varsinainen

### Laskutus
- Automatic invoice generation per household
- Invoice lines per member with membership fees
- Reference number and invoice number generation
- Payment tracking
- Delete invoices functionality

## Backend-komennot
- `get_organization` / `update_organization`
- `get_members` / `create_member_with_address` / `update_member_with_address` / `delete_member`
- `get_households_with_addresses` / `create_household_with_address` / `update_household_with_address` / `delete_household`
- `get_invoices` / `create_invoice_for_year` / `mark_invoice_paid` / `delete_invoice`
- `get_membership_fees` / `create_membership_fee` / `update_membership_fee`
- `save_pdf_file` / `show_save_dialog` (native file operations)

## Tärkeät tekniset ratkaisut

### SQLite DECIMAL → INTEGER -ongelma
SQLite tallentaa DECIMAL-arvot joskus INTEGER:nä. Koodissa käytetään:
```rust
let summa = match row.try_get::<f64, _>("summa") {
    Ok(val) => val,
    Err(_) => row.get::<i64, _>("summa") as f64,
};
```

### Frontend-backend data mapping
Frontend: camelCase ↔ Backend: snake_case
- `vastaanottaja` ↔ `vastaanottaja`
- `talouden_nimi` ↔ `talouden_nimi`
- `household.vastaanottaja` käytetään PDF:ssä laskun saajan nimenä

### PDF-generointi
- jsPDF (vektoripohjainen, ei html2canvas)
- Backend commands for file operations (no frontend plugins)
- Proper Finnish banking barcode generation
- A4 size constraints ja single-page layout

## Kehitysympäristö
- WSL2 + Ubuntu
- LIBGL_ALWAYS_INDIRECT=1 (graphics rendering)
- Node.js 20.x
- Rust stable
- Custom CSS (Tailwind removed due to ES module conflicts)

## Tietokantapolut
- **Kehitys (debug)**: `dev-database.db` (paikallinen)
- **Tuotanto (release)**: `~/.local/share/laskutin/laskutin.db` (Linux/WSL)
- Automaattinen valinta build-moden perusteella
- `get_database_info` komento näyttää käytössä olevan tietokannan

## Seuraavat kehityskohteet
- Implement proper `create_membership_fee` and `update_membership_fee`
- Add proper `mark_invoice_paid` implementation
- Enhance error handling and validation
- Add backup/restore functionality
- Implement invoice templates