# Laskutin - Jäsenmaksulaskutusohjelma

Tauri-pohjainen jäsenmaksulaskutusohjelma SQLite-tietokannalla ja Vue.js-käyttöliittymällä.

## Teknologiat
- **Backend**: Rust + Tauri
- **Frontend**: Vue.js + TypeScript + Tailwind CSS
- **Tietokanta**: SQLite
- **Paketinhallinta**: pnpm
- **Build system**: Vite

## Tietokantarakenne

### Yhdistyksen tiedot (organization)
- id (PK)
- nimi
- katuosoite
- postinumero
- postitoimipaikka
- puhelinnumero
- sähköposti
- y_tunnus
- pankkitili
- bic

### Jäsenet (members)
- id (PK)
- etunimi
- sukunimi
- henkilötunnus/syntymäaiva
- puhelinnumero
- sähköposti
- osoite_id (FK)
- liittymispäivä
- jäsentyyppi (varsinainen/kannatus/kunnia)
- aktiivinen (boolean)

### Osoitteet (addresses)
- id (PK)
- katuosoite
- postinumero
- postitoimipaikka
- talous_id (FK)

### Taloudet (households)
- id (PK)
- talouden_nimi (vapaaehtoinen)
- laskutusosoite_sama (boolean)
- laskutusosoite_id (FK, nullable)

### Jäsenmaksut (membership_fees)
- id (PK)
- vuosi
- jäsentyyppi
- summa

### Laskut (invoices)
- id (PK)
- talous_id (FK)
- luontipäivä
- eräpäivä
- summa
- viitenumero
- maksettu (boolean)
- maksupäivä (nullable)

### Laskurivit (invoice_lines)
- id (PK)
- lasku_id (FK)
- jäsen_id (FK)
- kuvaus
- summa

## Toiminnallisuus
- Jäsentietojen hallinta
- Osoitteiden ja talouksien hallinta
- Jäsenmaksujen määrittely vuosittain
- Laskujen luonti (1 lasku per talous)
- Laskutuloste-toiminto
- Maksujen seuranta

## Komennot
```bash
# Kehitys
pnpm tauri dev

# Build
pnpm tauri build

# Frontend kehitys
pnpm dev

# Testit
pnpm test

# Lint
pnpm lint

# Typecheck
pnpm type-check
```

## Projektin rakenne
- `src-tauri/` - Rust backend
- `src/` - Vue.js frontend
- `public/` - Staattiset tiedostot

## Tärkeät backend-vaatimukset (TODO)

### Jäsenten poisto
`delete_member` funktion pitää:
1. Poistaa jäsen tietokannasta
2. Tarkistaa onko taloudessa (addresses-taulun talous_id:n kautta) muita jäseniä
3. Jos talous jää tyhjäksi:
   - Poistaa osoite (addresses-taulusta)
   - Poistaa talous (households-taulusta)
   - Cascade delete huolehtii että kaikki liittyvät tiedot poistuvat

### Jäsenten lisäys/muokkaus
Frontend lähettää seuraavat tiedot:
- `osoitetyyppi`: 'oma' | 'talous' | 'uusi'
- `talous_id`: number | null (jos liittyy olemassa olevaan)
- `talouden_nimi`: string (jos luo uuden talouden)
- `katuosoite`, `postinumero`, `postitoimipaikka` (jos oma tai uusi osoite)

Backend logiikka:
- **'oma'**: Luo uusi talous + osoite yhdelle henkilölle
- **'talous'**: Liitä jäsen olemassa olevan talouden osoitteeseen
- **'uusi'**: Luo uusi talous + osoite, johon muut voivat liittyä

### Taloudet-lista
`get_households` pitää palauttaa:
```rust
{
  id: number,
  talouden_nimi: string,
  osoite: string  // Yhdistetty "katuosoite, postinumero postitoimipaikka"
}
```