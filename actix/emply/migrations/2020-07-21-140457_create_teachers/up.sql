-- Your SQL goes here
CREATE TABLE "teachers"
(
    id SERIAL UNIQUE,
    email VARCHAR PRIMARY KEY,
    password VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    designation VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    salary INT NOT NULL,
    age INT NOT NULL
)