CREATE TABLE "filingsother" (
    accession_number BIGINT PRIMARY KEY REFERENCES allfilings(accession_number),
    footnotes_json TEXT NOT NULL
)