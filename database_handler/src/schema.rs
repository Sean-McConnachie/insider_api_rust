table! {
    all_filings (accession_number) {
        accession_number -> Int8,
        acceptance_datetime -> Int8,
        filing_date -> Int8,
        report_date -> Int8,
        size -> Int4,
        company_cik -> Int4,
        form_link -> Nullable<Varchar>,
        index_link -> Nullable<Varchar>,
        form_type -> Varchar,
        fulfilled -> Bool,
        insider_ciks -> Array<Int4>,
    }
}

table! {
    all_insiders (insider_cik) {
        insider_cik -> Int4,
        name -> Varchar,
    }
}

table! {
    filings_data (accession_number) {
        accession_number -> Int8,
        d_holdings -> Jsonb,
        d_transactions -> Jsonb,
        nd_holdings -> Jsonb,
        nd_transactions -> Jsonb,
        footnotes -> Jsonb,
    }
}

table! {
    insider_roles (id) {
        id -> Nullable<Int4>,
        insider_cik -> Int4,
        company_cik -> Int4,
        director -> Bool,
        officer -> Bool,
        ten_percent -> Bool,
        other -> Bool,
        officer_title -> Nullable<Varchar>,
        other_text -> Nullable<Text>,
        str1 -> Varchar,
        str2 -> Nullable<Text>,
        city -> Varchar,
        state -> Varchar,
        zip -> Varchar,
        state_description -> Nullable<Varchar>,
    }
}

table! {
    json_docs (id) {
        id -> Nullable<Int4>,
        company_cik -> Int4,
        url -> Varchar,
        old -> Bool,
        fulfilled -> Bool,
    }
}

table! {
    stock_data (company_cik) {
        company_cik -> Int4,
        ticker -> Varchar,
        exchange -> Varchar,
        short_name -> Varchar,
        full_name -> Varchar,
        isin -> Varchar,
    }
}

joinable!(all_filings -> stock_data (company_cik));
joinable!(filings_data -> all_filings (accession_number));
joinable!(insider_roles -> all_insiders (insider_cik));
joinable!(insider_roles -> stock_data (company_cik));
joinable!(json_docs -> stock_data (company_cik));

allow_tables_to_appear_in_same_query!(
    all_filings,
    all_insiders,
    filings_data,
    insider_roles,
    json_docs,
    stock_data,
);