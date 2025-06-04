-- Add unique constraints to prevent duplicate members

-- Unique constraint on email if provided
CREATE UNIQUE INDEX IF NOT EXISTS idx_members_email 
ON members(sahkoposti) WHERE sahkoposti IS NOT NULL AND sahkoposti != '';

-- Composite unique constraint to prevent duplicate names in same household
CREATE UNIQUE INDEX IF NOT EXISTS idx_members_name_household
ON members(etunimi, sukunimi, osoite_id);

-- Unique constraint on henkilotunnus (personal ID) if provided
CREATE UNIQUE INDEX IF NOT EXISTS idx_members_henkilotunnus 
ON members(henkilotunnus) WHERE henkilotunnus IS NOT NULL AND henkilotunnus != '';