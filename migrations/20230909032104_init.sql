CREATE TABLE location_mark(
  id       UUID                   PRIMARY KEY,
  location GEOGRAPHY(POINT, 4326) NOT NULL
);

CREATE TABLE location_mark_localized_name(
  id      UUID         NOT NULL,
  country VARCHAR(4)   NOT NULL,
  name    VARCHAR(128) NOT NULL,

  PRIMARY KEY (id, country),
  FOREIGN KEY (id) REFERENCES location_mark(id) ON DELETE CASCADE
);