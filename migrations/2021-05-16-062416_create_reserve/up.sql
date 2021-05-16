-- Your SQL goes here

CREATE TABLE "reserve"
(
    id SERIAL PRIMARY KEY,
    full_name VARCHAR NOT NULL,
    phone_number VARCHAR NOT NULL,
    reserve_date VARCHAR NOT NULL

)