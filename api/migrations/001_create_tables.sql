CREATE TABLE IF NOT EXISTS senders (
  id serial PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS recipients (
  id serial PRIMARY KEY,
  username VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS mails (
  time TIMESTAMP NOT NULL,
  mess_id VARCHAR(255) NOT NULL,
  size serial nullable,
  dovecot boolean default false, 
  sender serial not NULL,
  recipient_list serial[],

  FOREIGN KEY (sender) REFERENCES senders(id),
);
