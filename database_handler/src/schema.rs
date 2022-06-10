table! {
    allfilings (accession_number) {
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
    }
}

table! {
    allinsiders (insider_cik) {
        insider_cik -> Int4,
        name -> Varchar,
    }
}

table! {
    dholdings (id) {
        id -> Int4,
        accession_number -> Int8,
        security_title -> Varchar,
        price -> Float4,
        exercise_date -> Nullable<Int8>,
        expiration_date -> Nullable<Int8>,
        underlying_security_title -> Varchar,
        underlying_security_price -> Float4,
        ownership_nature -> Varchar,
        indirect_relation -> Nullable<Varchar>,
    }
}

table! {
    dtransactions (id) {
        id -> Int4,
        accession_number -> Int8,
        security_title -> Varchar,
        price -> Float4,
        transaction_date -> Nullable<Int8>,
        deemed_execution_date -> Nullable<Int8>,
        transaction_code -> Varchar,
        transaction_equity_swap -> Bool,
        transaction_shares -> Float4,
        transaction_share_price -> Float4,
        transaction_ad_code -> Varchar,
        exercise_date -> Nullable<Int8>,
        expiration_date -> Nullable<Int8>,
        underlying_security_title -> Varchar,
        underlying_security_price -> Float4,
        post_transaction_total_shares -> Float4,
        ownership_nature -> Varchar,
        indirect_relation -> Nullable<Varchar>,
    }
}

table! {
    filingsother (accession_number) {
        accession_number -> Int8,
        footnotes_json -> Text,
    }
}

table! {
    insiderroles (id) {
        id -> Int4,
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
    jsondocs (id) {
        id -> Int4,
        company_cik -> Int4,
        url -> Varchar,
        old -> Bool,
        fulfilled -> Bool,
    }
}

table! {
    ndholdings (id) {
        id -> Int4,
        accession_number -> Int8,
        security_title -> Varchar,
        post_transaction_amount -> Float4,
        ownership_nature -> Varchar,
        indirect_relation -> Nullable<Varchar>,
    }
}

table! {
    ndtransactions (id) {
        id -> Int4,
        accession_number -> Int8,
        security_title -> Varchar,
        transaction_date -> Nullable<Int8>,
        transaction_code -> Varchar,
        transaction_equity_swap -> Bool,
        transaction_shares -> Float4,
        transaction_share_price -> Float4,
        transaction_ad_code -> Varchar,
        post_transaction_total_shares -> Float4,
        ownership_nature -> Varchar,
        indirect_relation -> Nullable<Varchar>,
    }
}

table! {
    stockdata (company_cik) {
        company_cik -> Int4,
        ticker -> Varchar,
        exchange -> Varchar,
        short_name -> Varchar,
        full_name -> Varchar,
        isin -> Varchar,
    }
}

joinable!(allfilings -> stockdata (company_cik));
joinable!(dholdings -> allfilings (accession_number));
joinable!(dtransactions -> allfilings (accession_number));
joinable!(filingsother -> allfilings (accession_number));
joinable!(insiderroles -> allinsiders (insider_cik));
joinable!(insiderroles -> stockdata (company_cik));
joinable!(jsondocs -> stockdata (company_cik));
joinable!(ndholdings -> allfilings (accession_number));
joinable!(ndtransactions -> allfilings (accession_number));

allow_tables_to_appear_in_same_query!(
    allfilings,
    allinsiders,
    dholdings,
    dtransactions,
    filingsother,
    insiderroles,
    jsondocs,
    ndholdings,
    ndtransactions,
    stockdata,
);
