CREATE TABLE "scraper"."json_docs" (
    id SERIAL PRIMARY KEY,
    company_cik INTEGER NOT NULL REFERENCES "scraper"."stock_data"(company_cik),
    url VARCHAR NOT NULL,
    old BOOLEAN NOT NULL,
    fulfilled BOOLEAN NOT NULL DEFAULT FALSE
);