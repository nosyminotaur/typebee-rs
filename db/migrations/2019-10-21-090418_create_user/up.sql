-- Your SQL goes here
CREATE TABLE Users (
   id serial PRIMARY KEY,
   username VARCHAR (50) UNIQUE NOT NULL,
   passhash VARCHAR (128) NOT NULL,
   email VARCHAR (355) UNIQUE NOT NULL,
   created_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
   last_login TIMESTAMP
);