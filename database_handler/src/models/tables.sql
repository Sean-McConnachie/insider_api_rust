/* Any stock that should be tracked in insider trading should be put in this table */
CREATE TABLE IF NOT EXISTS stock_data (
    company_cik INTEGER PRIMARY KEY,
    ticker VARCHAR(8) UNIQUE NOT NULL,
    exchange VARCHAR(8) NOT NULL,
    company_short_name VARCHAR(255) NOT NULL,
    company_full_name TEXT NOT NULL,
    isin VARCHAR(12) NOT NULL
);

/* Only a temporary storage table for making the json document requests */
CREATE TABLE IF NOT EXISTS insider.json_docs (
    company_cik INTEGER REFERENCES public.stock_data(company_cik),
    url VARCHAR(100) NOT NULL,
    old BOOLEAN NOT NULL,
    fulfilled BOOLEAN NOT NULL DEFAULT FALSE
);

/* This table contains all possible insider trades for any company found in public.stock_data */
CREATE TABLE IF NOT EXISTS insider.all_filings (
    accession_number BIGINT PRIMARY KEY,
    acceptance_datetime BIGINT NOT NULL,
    filing_date BIGINT NOT NULL,
    report_date BIGINT NOT NULL,
    size BIGINT NOT NULL,
    company_cik INTEGER REFERENCES public.stock_data(company_cik),
    link VARCHAR(45) DEFAULT NULL,
    index_link VARCHAR(128) DEFAULT NULL,
    form_type VARCHAR(1) NOT NULL,
    fulfilled BOOLEAN NOT NULL DEFAULT FALSE
);

/* A parent table - only used by insider.insider_roles */
CREATE TABLE IF NOT EXISTS insider.all_insiders (
    insider_cik INTEGER PRIMARY KEY,
    name TEXT
);

/* This table contains company-specific data for every insider */
CREATE TABLE IF NOT EXISTS insider.insider_roles (
    insider_cik INTEGER REFERENCES insider.all_insiders(insider_cik),
    company_cik INTEGER REFERENCES public.stock_data(company_cik),
    director BOOLEAN NOT NULL,
    officer BOOLEAN NOT NULL,
    ten_percent BOOLEAN NOT NULL,
    other BOOLEAN NOT NULL,
    officer_title TEXT,
    other_text TEXT,
    str1 TEXT,
    str2 TEXT,
    city TEXT,
    state TEXT,
    zip VARCHAR(10),
    state_description TEXT
);

/* Derivative holdings table for every insider file found in insider.all_filings */
CREATE TABLE IF NOT EXISTS insider.derivative_holdings (
    accession_number BIGINT REFERENCES insider.all_filings(accession_number),
    security_title TEXT,
    price REAL,
    exercise_date BIGINT,
    expiration_date BIGINT,
    underlying_security_title TEXT,
    underlying_security_price REAL,
    ownership_nature VARCHAR(1),
    indirect_relation TEXT
);

/* Non-derivatives holdings table for every insider file found in insider.all_filings */
CREATE TABLE IF NOT EXISTS insider.non_derivative_holdings (
    accession_number BIGINT REFERENCES insider.all_filings(accession_number),
    security_title TEXT,
    post_transaction_amount REAL,
    ownership_nature VARCHAR(1),
    indirect_relation TEXT
);

/* Derivative transactions table for every insider file found in insider.all_filings */
CREATE TABLE IF NOT EXISTS insider.derivative_transactions (
    accession_number BIGINT REFERENCES insider.all_filings(accession_number),
    security_title TEXT,
    price REAL,
    transaction_date BIGINT,
    deemed_execution_date BIGINT,
    transaction_code VARCHAR(1), -- Transaction coding
    transaction_equity_swap BOOLEAN,
    transaction_shares REAL, -- Transaction amounts
    transaction_share_price REAL,
    transaction_ad_code VARCHAR(1),
    exercise_date BIGINT,
    expiration_date BIGINT,
    underlying_security_title TEXT, -- Underlying securities
    underlying_security_price REAL,
    post_transaction_total_shares REAL,
    ownership_nature VARCHAR(1), -- Ownership nature
    indirect_relation TEXT
);

/* Non-derivatives transactions table for every insider file found in insider.all_filings */
CREATE TABLE IF NOT EXISTS insider.non_derivative_transactions (
    accession_number BIGINT REFERENCES insider.all_filings(accession_number),
    security_title TEXT,
    transaction_date BIGINT,
    transaction_code VARCHAR(1), -- Transaction coding
    transaction_equity_swap BOOLEAN,
    transaction_shares REAL, -- Transaction amounts
    transaction_share_price REAL,
    transaction_ad_code VARCHAR(1),
    post_transaction_total_shares REAL,
    ownership_nature VARCHAR(1), -- Ownership nature
    indirect_relation TEXT
);

/* Footnotes and remarks table for every insider file found in insider.all_filings - basically for useless data */
CREATE TABLE IF NOT EXISTS insider.filing_other (
    accession_number BIGINT REFERENCES insider.all_filings(accession_number),
    footnotes_json TEXT
)