CREATE TABLE rustaceans (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

INSERT INTO rustaceans (name)
VALUES
  ('Carol'),
  ('Jake'),
  ('Vivian'),
  ('Jon'),
  ('Steve');
