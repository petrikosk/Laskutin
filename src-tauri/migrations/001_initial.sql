-- Yhdistyksen tiedot (organization)
CREATE TABLE IF NOT EXISTS organization (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nimi TEXT NOT NULL,
    katuosoite TEXT NOT NULL,
    postinumero TEXT NOT NULL,
    postitoimipaikka TEXT NOT NULL,
    puhelinnumero TEXT,
    sahkoposti TEXT,
    y_tunnus TEXT,
    pankkitili TEXT,
    bic TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Taloudet (households)
CREATE TABLE IF NOT EXISTS households (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    talouden_nimi TEXT,
    laskutusosoite_sama BOOLEAN NOT NULL DEFAULT 1,
    laskutusosoite_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (laskutusosoite_id) REFERENCES addresses(id)
);

-- Osoitteet (addresses)
CREATE TABLE IF NOT EXISTS addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    katuosoite TEXT NOT NULL,
    postinumero TEXT NOT NULL,
    postitoimipaikka TEXT NOT NULL,
    talous_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (talous_id) REFERENCES households(id) ON DELETE CASCADE
);

-- Jäsenet (members)
CREATE TABLE IF NOT EXISTS members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    etunimi TEXT NOT NULL,
    sukunimi TEXT NOT NULL,
    henkilotunnus TEXT,
    syntymaaika DATE,
    puhelinnumero TEXT,
    sahkoposti TEXT,
    osoite_id INTEGER NOT NULL,
    liittymispaiva DATE NOT NULL,
    jasentyyppi TEXT NOT NULL CHECK (jasentyyppi IN ('varsinainen', 'kannatus', 'kunnia')),
    aktiivinen BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (osoite_id) REFERENCES addresses(id)
);

-- Jäsenmaksut (membership_fees)
CREATE TABLE IF NOT EXISTS membership_fees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vuosi INTEGER NOT NULL,
    jasentyyppi TEXT NOT NULL CHECK (jasentyyppi IN ('varsinainen', 'kannatus', 'kunnia')),
    summa DECIMAL(10,2) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(vuosi, jasentyyppi)
);

-- Laskut (invoices)
CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    talous_id INTEGER NOT NULL,
    luontipaiva DATE NOT NULL DEFAULT (date('now')),
    erapaiva DATE NOT NULL,
    summa DECIMAL(10,2) NOT NULL,
    viitenumero TEXT NOT NULL UNIQUE,
    maksettu BOOLEAN NOT NULL DEFAULT 0,
    maksupaiva DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (talous_id) REFERENCES households(id)
);

-- Laskurivit (invoice_lines)
CREATE TABLE IF NOT EXISTS invoice_lines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    lasku_id INTEGER NOT NULL,
    jasen_id INTEGER NOT NULL,
    kuvaus TEXT NOT NULL,
    summa DECIMAL(10,2) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lasku_id) REFERENCES invoices(id) ON DELETE CASCADE,
    FOREIGN KEY (jasen_id) REFERENCES members(id)
);

-- Indeksit suorituskyvyn parantamiseksi
CREATE INDEX IF NOT EXISTS idx_members_osoite ON members(osoite_id);
CREATE INDEX IF NOT EXISTS idx_members_jasentyyppi ON members(jasentyyppi);
CREATE INDEX IF NOT EXISTS idx_members_aktiivinen ON members(aktiivinen);
CREATE INDEX IF NOT EXISTS idx_addresses_talous ON addresses(talous_id);
CREATE INDEX IF NOT EXISTS idx_invoices_talous ON invoices(talous_id);
CREATE INDEX IF NOT EXISTS idx_invoices_maksettu ON invoices(maksettu);
CREATE INDEX IF NOT EXISTS idx_invoice_lines_lasku ON invoice_lines(lasku_id);
CREATE INDEX IF NOT EXISTS idx_invoice_lines_jasen ON invoice_lines(jasen_id);
CREATE INDEX IF NOT EXISTS idx_membership_fees_vuosi_tyyppi ON membership_fees(vuosi, jasentyyppi);

-- Jäsenmaksut perusdata (pakolliset maksutyypit)
INSERT OR IGNORE INTO membership_fees (vuosi, jasentyyppi, summa) VALUES
(2024, 'varsinainen', 50.00),
(2024, 'kannatus', 25.00),
(2024, 'kunnia', 0.00),
(2025, 'varsinainen', 55.00),
(2025, 'kannatus', 30.00),
(2025, 'kunnia', 0.00);