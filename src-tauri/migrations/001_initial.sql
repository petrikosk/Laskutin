-- Yhdistyksen tiedot (organization)
CREATE TABLE organization (
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
CREATE TABLE households (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    talouden_nimi TEXT,
    laskutusosoite_sama BOOLEAN NOT NULL DEFAULT 1,
    laskutusosoite_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (laskutusosoite_id) REFERENCES addresses(id)
);

-- Osoitteet (addresses)
CREATE TABLE addresses (
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
CREATE TABLE members (
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
CREATE TABLE membership_fees (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vuosi INTEGER NOT NULL,
    jasentyyppi TEXT NOT NULL CHECK (jasentyyppi IN ('varsinainen', 'kannatus', 'kunnia')),
    summa DECIMAL(10,2) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(vuosi, jasentyyppi)
);

-- Laskut (invoices)
CREATE TABLE invoices (
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
CREATE TABLE invoice_lines (
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
CREATE INDEX idx_members_osoite ON members(osoite_id);
CREATE INDEX idx_members_jasentyyppi ON members(jasentyyppi);
CREATE INDEX idx_members_aktiivinen ON members(aktiivinen);
CREATE INDEX idx_addresses_talous ON addresses(talous_id);
CREATE INDEX idx_invoices_talous ON invoices(talous_id);
CREATE INDEX idx_invoices_maksettu ON invoices(maksettu);
CREATE INDEX idx_invoice_lines_lasku ON invoice_lines(lasku_id);
CREATE INDEX idx_invoice_lines_jasen ON invoice_lines(jasen_id);
CREATE INDEX idx_membership_fees_vuosi_tyyppi ON membership_fees(vuosi, jasentyyppi);

-- Esimerkkidata
INSERT INTO organization (nimi, katuosoite, postinumero, postitoimipaikka, puhelinnumero, sahkoposti, y_tunnus, pankkitili, bic) VALUES
('Esimerkkiyhdistys ry', 'Yhdistyskatu 1', '00100', 'Helsinki', '09-12345678', 'yhdistys@example.fi', '1234567-8', 'FI12 3456 7890 1234 56', 'OKOYFIHH');

INSERT INTO households (id, talouden_nimi, laskutusosoite_sama) VALUES 
(1, 'Korhosen perhe', 1),
(2, NULL, 1);

INSERT INTO addresses (id, katuosoite, postinumero, postitoimipaikka, talous_id) VALUES
(1, 'Kotikatu 1', '00100', 'Helsinki', 1),
(2, 'Testikatu 2', '00200', 'Espoo', 2);

INSERT INTO members (etunimi, sukunimi, henkilotunnus, puhelinnumero, sahkoposti, osoite_id, liittymispaiva, jasentyyppi) VALUES
('Matti', 'Korhonen', '010180-123A', '040-1234567', 'matti@example.com', 1, '2023-01-15', 'varsinainen'),
('Maija', 'Korhonen', '020190-456B', '040-7654321', 'maija@example.com', 1, '2023-01-15', 'varsinainen'),
('Testi', 'Henkilö', '030185-789C', '050-1112223', 'testi@example.com', 2, '2023-03-10', 'kannatus');

INSERT INTO membership_fees (vuosi, jasentyyppi, summa) VALUES
(2024, 'varsinainen', 50.00),
(2024, 'kannatus', 25.00),
(2024, 'kunnia', 0.00),
(2025, 'varsinainen', 55.00),
(2025, 'kannatus', 30.00),
(2025, 'kunnia', 0.00);