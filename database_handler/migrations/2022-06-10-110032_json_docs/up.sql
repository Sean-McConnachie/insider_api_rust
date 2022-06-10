CREATE TABLE "jsondocs" (
    id SERIAL PRIMARY KEY,
    company_cik INTEGER NOT NULL REFERENCES stockdata(company_cik),
    url VARCHAR NOT NULL,
    old BOOLEAN NOT NULL,
    fulfilled BOOLEAN NOT NULL DEFAULT FALSE
);