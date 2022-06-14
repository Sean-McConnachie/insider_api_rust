CREATE TABLE "scraper"."filings_data" (
    accession_number BIGINT PRIMARY KEY REFERENCES "scraper"."all_filings"(accession_number),
    d_holdings JSONB NOT NULL,
    d_transactions JSONB NOT NULL,
    nd_holdings JSONB NOT NULL,
    nd_transactions JSONB NOT NULL,
    footnotes JSONB NOT NULL
)