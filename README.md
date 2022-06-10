# Insider API built in Rust

---

## What is it?
Currently, the two main sites for getting insider information are:
- [OpenInsider](http://openinsider.com/)
- [Insider](https://finviz.com/insidertrading.ashx/)

There are also a few APIs, however, they are paid.

This application is designed to gather all insider trades for any company specified in the `public.stockdata` table
and provide a high-performance, free API to anyone using it in accordance with the license specified below.

---

## Written in Rust
The initial plan was to write the whole project in python. After some preliniminary testing, I found that python's
performance in parsing web pages was... *sub-optimal* to say the least. The first solution to this slow parsing was
writing the parser in rust (and calling it from python). This decreased parsing speed from `1.5s` to `0.05s` (**30x
faster**).

Writing my first 100-line program in `Rust` made me fall in love.

After realising how much faster Rust really is, I tested [`FastApi`](https://github.com/tiangolo/fastapi) against
[`Warp`](https://github.com/seanmonstar/warp). Requests per second were **7-8x faster** in most tests.

The switch to Rust was made and version 1 of the scraper ensued. This was a very crude implementation of **only** the
scraper/parser (no API). Obviously, since this was my first real project in Rust, I had to learn a lot about the language.
Therefore, I made the decision to leave bad implementations in the codebase and make a note of everything that could be
improved upon in `v2`.

While I base my decision to not update bad code on learning Rust, another major factor was just how **HORRIBLE** SEC's
API/resources/data structures were. I mean seriously - it was a nightmare trying to figure this out.

The three main crates used in this project are:
- [`Hyper`](https://github.com/hyperium/hyper)
- [`Actix-web`](https://github.com/actix/actix-web)
- [`Diesel`](https://github.com/diesel-rs/diesel)
- [`serde_json`](https://crates.io/crates/serde)

---

## API
### How to use the API
1) Navigate to the **[`website`](http://localhost:8080/)** and login via Google
2) Go to your account and create an app
3) Store the token somewhere in your application
4) Pass the token as a header parameter `{token}` in your request `REQUIRED!`

### Endpoints
#### **[`Base URL`](http://localhost:8080/api/)**: `http://localhost:8080/api`

*Indents with `{}` represent parameters.*

---

- `/companies/data`: Returns all stocks used in collection
  - `{company_ciks}`: `csv` of `company CIKS` which should be returned (alias = `company_cik`)
  - `{tickers}`: `csv` of `company tickers` which should be returned (alias = `ticker`)
  - `{exchanges}`: `csv` of `exchanges` which should be returned (alias = `exchange`)
  - `{isin}`: `csv` of `ISIN`s which should be returned (alias = `exchange`)
---
- `/insiders/names`: Returns all insider CIKS and names
  - `{insider_ciks}`: `csv` of `insider CIKS` which should be returned (alias = `insider_cik`)
- `/insiders/roles`: Returns all insiders and their roles in a company
  - `{insider_ciks}`: `csv` of `insider CIKS` which should be returned (alias = `insider_cik`)
  - `{address}`: `bool` indicating if the address should be returned (default = `false`)
---
In general, `nd_transactions` is data most people will be looking for.
- `/filings/metadata`: Returns all **metadata** for insider trades.
  - `{ciks}`: `csv` of `insider CIKS` which should be returned (alias = `cik`)
  - `{accession_numbers}`: `csv` of `accession numbers` which should be returned (alias = `accession_number`)
  - `{insider_ciks}`: `csv` of `insider CIKS` which should be returned (alias = `insider_cik`)
  - `{company_ciks}`: `csv` of `company CIKS` which should be returned (alias = `company_cik`)
  - `{form_types}`: `csv` of `form types` which should be returned (alias = `form_type`)
  - *`{start_date}`: `datetime` in epoch miliseconds 
  - *`{end_date}`: `datetime` in epoch miliseconds 
  - **`{period_range}`
  - **`{period_time}`
  - **`{period_go_back}`
- `/filings/data`: Returns all **data** for insider trades. This includes `Derivative holdings`, `Derivative transactions`, `Non-derivative holdings`, and `Non-derivative transactions`. Parameters are inherited from `/filings/metadata`.
  - `{uses}`: `csv` of `<option>` data to return (alias = `use`). Options values are: 
    - `d_holdings`
    - `d_transactions`
    - `nd_holdings`
    - `nd_transactions`

`*` Cannot be used in conjunction with any `**` parameters.

`**` Cannot be used in conjunction with any `*` parameters.
- `period_range`: How far back/forward to look. Syntax is `< int >< d | w | m | y >`. Eg. `5d`. Extreme values can also be used - ie. `53w`
- `period_time`: Specifies the **starting** point or **ending** point based on the parameter passed to `{period_go_back}`
- `period_go_back`: `bool` (default = `true`). If `true`, values dating **back** from `{period_time}` are returned. Otherwise, values dating **forward** from `{period_time}` are returned.