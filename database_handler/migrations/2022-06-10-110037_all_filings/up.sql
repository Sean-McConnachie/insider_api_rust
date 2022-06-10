CREATE TABLE "allfilings" (
    accession_number BIGINT PRIMARY KEY,
    acceptance_datetime BIGINT NOT NULL,
    filing_date BIGINT NOT NULL,
    report_date BIGINT NOT NULL,
    size INTEGER NOT NULL,
    company_cik INTEGER NOT NULL REFERENCES stockdata(company_cik),
    form_link VARCHAR DEFAULT NULL,
    index_link VARCHAR DEFAULT NULL,
    form_type VARCHAR(1) NOT NULL,
    fulfilled BOOLEAN NOT NULL DEFAULT FALSE
);