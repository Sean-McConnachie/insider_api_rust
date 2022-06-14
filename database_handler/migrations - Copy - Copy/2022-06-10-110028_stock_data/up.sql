CREATE TABLE "scraper"."stock_data" (
    company_cik INTEGER PRIMARY KEY,
    ticker VARCHAR(8) UNIQUE NOT NULL,
    exchange VARCHAR(8) NOT NULL,
    short_name VARCHAR NOT NULL,
    full_name VARCHAR NOT NULL,
    isin VARCHAR(12) NOT NULL
);