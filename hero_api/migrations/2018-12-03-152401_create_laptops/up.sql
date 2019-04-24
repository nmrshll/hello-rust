-- Your SQL goes here
CREATE TABLE IF NOT EXISTS laptops (
  id SERIAL PRIMARY KEY,
  company VARCHAR NOT NULL,
  product VARCHAR NOT NULL,
  typename VARCHAR,
  inches VARCHAR,
  screenresolution VARCHAR,
  cpu VARCHAR,
  ram VARCHAR,
  memory VARCHAR,
  gpu VARCHAR,
  opsys VARCHAR,
  weight VARCHAR,
  price_euros REAL
)