CREATE TABLE account (
  id SERIAL PRIMARY KEY,
  name character varying(255) NOT NULL UNIQUE
);

INSERT INTO account (id, name) VALUES
(1, 'MStillwell'),
(2, 'new_user'),
(3, 'old_user');
