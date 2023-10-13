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

CREATE TABLE instances(
  id          UUID        NOT NULL PRIMARY KEY,
  location    UUID        NOT NULL,
  laps        SERIAL      NOT NULL,
  started_at  TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
  finished_at TIMESTAMPTZ,

  FOREIGN KEY (location) REFERENCES location_mark(id) ON DELETE CASCADE
);

CREATE TABLE rings(
  id         UUID NOT NULL PRIMARY KEY ,
  instance   UUID NOT NULL,
  pos_in     GEOGRAPHY(POINT, 4326) NOT NULL,
  hue        SERIAL NOT NULL,
  addr       INET NOT NULL,
  index      SERIAL NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),

  FOREIGN KEY (instance) REFERENCES instances(id) ON DELETE CASCADE
);