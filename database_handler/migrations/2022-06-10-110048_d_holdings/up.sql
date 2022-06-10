CREATE TABLE "dholdings" (
    id SERIAL PRIMARY KEY,
    accession_number BIGINT NOT NULL REFERENCES allfilings(accession_number),
    security_title VARCHAR NOT NULL,
    price REAL NOT NULL,
    exercise_date BIGINT DEFAULT NULL,
    expiration_date BIGINT DEFAULT NULL,
    underlying_security_title VARCHAR NOT NULL,
    underlying_security_price REAL NOT NULL,
    ownership_nature VARCHAR(1) NOT NULL,
    indirect_relation VARCHAR DEFAULT NULL
)