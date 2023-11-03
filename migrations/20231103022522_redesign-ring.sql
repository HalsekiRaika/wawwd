-- Add migration script here
ALTER TABLE rings ADD COLUMN location UUID NOT NULL default '00000000-0000-0000-0000-000000000000';
ALTER TABLE rings ADD FOREIGN KEY (location) REFERENCES location_mark(id);