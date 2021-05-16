-- Your SQL goes here

CREATE TABLE "employees"
(
id SERIAL PRIMARY KEY,
firstname VARCHAR NOT NULL,
lastnam VARCHAR NOT NULL,
department VARCHAR NOT NULL,
salary  INT NOT NULL,
age  INT NOT NULL
)