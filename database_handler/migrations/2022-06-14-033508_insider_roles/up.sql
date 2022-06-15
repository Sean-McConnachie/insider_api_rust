CREATE TABLE "insider_roles" (
    id SERIAL PRIMARY KEY,
    insider_cik INTEGER NOT NULL REFERENCES "all_insiders"(insider_cik),
    company_cik INTEGER NOT NULL REFERENCES "stock_data"(company_cik),
    director BOOLEAN NOT NULL,
    officer BOOLEAN NOT NULL,
    ten_percent BOOLEAN NOT NULL,
    other BOOLEAN NOT NULL,
    officer_title VARCHAR DEFAULT NULL,
    other_text VARCHAR DEFAULT NULL,
    str1 VARCHAR NOT NULL,
    str2 VARCHAR DEFAULT NULL,
    city VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    zip VARCHAR(10) NOT NULL,
    state_description VARCHAR DEFAULT NULL
);