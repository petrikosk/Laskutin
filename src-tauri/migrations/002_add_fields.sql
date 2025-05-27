-- Lisää vastaanottajan nimi taloudelle
ALTER TABLE households ADD COLUMN vastaanottaja TEXT;

-- Lisää laskunumero laskulle (erillinen kenttä viitenumerosta)
ALTER TABLE invoices ADD COLUMN laskunumero TEXT;

-- Päivitä esimerkkidata
UPDATE households SET vastaanottaja = 'Matti Korhonen' WHERE id = 1;
UPDATE households SET vastaanottaja = 'Testi Henkilö' WHERE id = 2;