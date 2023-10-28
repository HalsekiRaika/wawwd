CREATE TABLE images (
  id         UUID        NOT NULL PRIMARY KEY,
  image      BYTEA       NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (id) REFERENCES rings(id) ON DELETE CASCADE
);