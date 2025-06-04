-- Update CHECK constraints to include nuorisojasen

-- SQLite doesn't support ALTER COLUMN directly, so we need to recreate the tables
-- with updated constraints

-- Create new members table with updated constraint
CREATE TABLE members_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    etunimi TEXT NOT NULL,
    sukunimi TEXT NOT NULL,
    henkilotunnus TEXT,
    syntymaaika DATE,
    puhelinnumero TEXT,
    sahkoposti TEXT,
    osoite_id INTEGER NOT NULL,
    liittymispaiva DATE NOT NULL,
    jasentyyppi TEXT NOT NULL CHECK (jasentyyppi IN ('varsinainen', 'nuorisojasen', 'kannatus', 'kunnia')),
    aktiivinen BOOLEAN NOT NULL DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (osoite_id) REFERENCES addresses(id)
);

-- Copy data from old table
INSERT INTO members_new SELECT * FROM members;

-- Drop old table and rename new one
DROP TABLE members;
ALTER TABLE members_new RENAME TO members;

-- Recreate indexes for members
CREATE INDEX IF NOT EXISTS idx_members_osoite ON members(osoite_id);
CREATE INDEX IF NOT EXISTS idx_members_jasentyyppi ON members(jasentyyppi);
CREATE INDEX IF NOT EXISTS idx_members_aktiivinen ON members(aktiivinen);

-- Create new membership_fees table with updated constraint
CREATE TABLE membership_fees_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vuosi INTEGER NOT NULL,
    jasentyyppi TEXT NOT NULL CHECK (jasentyyppi IN ('varsinainen', 'nuorisojasen', 'kannatus', 'kunnia')),
    summa DECIMAL(10,2) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(vuosi, jasentyyppi)
);

-- Copy data from old table
INSERT INTO membership_fees_new SELECT * FROM membership_fees;

-- Drop old table and rename new one
DROP TABLE membership_fees;
ALTER TABLE membership_fees_new RENAME TO membership_fees;

-- Recreate index for membership_fees
CREATE INDEX IF NOT EXISTS idx_membership_fees_vuosi_tyyppi ON membership_fees(vuosi, jasentyyppi);

-- Add default nuorisojasen membership fees
INSERT OR IGNORE INTO membership_fees (vuosi, jasentyyppi, summa) VALUES
(2024, 'nuorisojasen', 25.00),
(2025, 'nuorisojasen', 30.00);