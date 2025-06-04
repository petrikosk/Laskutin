-- Lisää nuorisojäsenen ikäraja yhdistyksen tietoihin
ALTER TABLE organization ADD COLUMN nuorisojasen_ikaraja INTEGER DEFAULT 18;