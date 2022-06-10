CREATE TABLE "dtransactions" (
    id SERIAL PRIMARY KEY,
    accession_number BIGINT NOT NULL REFERENCES allfilings(accession_number),
    security_title VARCHAR NOT NULL,
    price REAL NOT NULL,
    transaction_date BIGINT DEFAULT NULL,
    deemed_execution_date BIGINT DEFAULT NULL,
    transaction_code VARCHAR(1) NOT NULL, -- Transaction coding
    transaction_equity_swap BOOLEAN NOT NULL,
    transaction_shares REAL NOT NULL, -- Transaction amounts
    transaction_share_price REAL NOT NULL,
    transaction_ad_code VARCHAR(1) NOT NULL,
    exercise_date BIGINT DEFAULT NULL,
    expiration_date BIGINT DEFAULT NULL,
    underlying_security_title VARCHAR NOT NULL, -- Underlying securities
    underlying_security_price REAL NOT NULL,
    post_transaction_total_shares REAL NOT NULL,
    ownership_nature VARCHAR(1) NOT NULL, -- Ownership nature
    indirect_relation VARCHAR DEFAULT NULL
)