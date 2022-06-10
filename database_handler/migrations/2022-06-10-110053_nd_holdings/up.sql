CREATE TABLE "ndholdings" (
    id SERIAL PRIMARY KEY,
    accession_number BIGINT NOT NULL REFERENCES allfilings(accession_number),
    security_title VARCHAR NOT NULL,
    post_transaction_amount REAL NOT NULL,
    ownership_nature VARCHAR(1) NOT NULL ,
    indirect_relation VARCHAR DEFAULT NULL
);