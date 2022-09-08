# Insider API built in Rust

---

## What is it?
Currently, the two main sites for getting insider information are:
- [OpenInsider](http://openinsider.com/)
- [Insider](https://finviz.com/insidertrading.ashx/)

There are also a few APIs, however, they are paid.

This application is designed to gather all-time insider trades for any company specified in the `public.stockdata` table
and provide a high-performance API.

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

## Where the data comes from
Figuring out how to collect all the data through SEC was a ***pain*** to say the least. Their data is stored in a horrible manner.

#### A) Collecting all-time filing links
  1) For each company CIK in `stock_data` table:
     1) Query `json` which contains the majority of filings with basic data and links to `.xml` documents
     2) Store basic data in database
     3) Store links to additional `json`s in database 
  2) For each additional `json` link in database
     1) Format is different to step one, but data is the same, however, from an earlier time
     2) Store basic data in database

**Note: Up until now, we do not have any useful information. Just basic dates, accession numbers and links to `.xml` files.**

#### B) Collecting recent filing links (can be run every day)
  1) Collect basic filing data for each company (max 20 filings) from RSS feed using company CIKs in `stock_data` table
  2) Store basic data in database

**Note: The links gathered from here are not actually links to `.xml` files. Instead, they are links to an `html` document, which contains the links to the `.xml` files that we want.**

#### C) Getting the `.xml` files from step B
  1) Open `html` links from step B
  2) Parse the page
  3) Find the wanted `.xml` link/file
  4) Update associated filing in DB

#### D) Finally collecting the useful filing data (i.e. money talk)

  1) Open the `form_link` for each filing in `all_filings` table
  2) Parse this monstrosity of a document
  3) Store data in `filings_data` table

**Note: The data structure in the `xml` document is never the same. It changes based on what fields are present which
makes parsing very difficult (especially since this is always-safe rust).**

--

This program also rate limits itself as SEC has a limit of around 10 requests per second (this program can do 70k
requests/s when tested locally).

### Endpoints
#### **[`Base URL`](http://localhost:8080/api/)**: `http://localhost:8080/api`

*Indents with `{}` represent parameters.*

---

- `/companies/data`: Returns all stocks used in collection
  - `{company_ciks}`: `csv` of `company CIKS` which should be returned (alias = `company_cik`)
  - `{tickers}`: `csv` of `company tickers` which should be returned (alias = `ticker`)
  - `{exchanges}`: `csv` of `exchanges` which should be returned (alias = `exchange`)
  - `{isins}`: `csv` of `ISIN`s which should be returned (alias = `isin`)
---
- `/insiders/names`: Returns all insider CIKS and names
  - `{insider_ciks}`: `csv` of `insider CIKS` which should be returned (alias = `insider_cik`)
- `/insiders/roles`: Returns all insiders and their roles in a company
  - `{insider_ciks}`: `csv` of `insider CIKS` which should be returned (alias = `insider_cik`)
  - `{company_ciks}`: `csv` of `company CIKS` which should be returned (alias = `company_cik`)
  - `{director}`: `bool` indicating if the role is a director
  - `{officer}`: `bool` indicating if the role is a director
  - `{ten_percent}`: `bool` indicating if the role is a director
  - `{other}`: `bool` indicating if the role is a director
---
In general, `nd_transactions` is data most people will be looking for.
- `/filings/metadata`: Returns all **metadata** for insider trades.
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

© 2022 Sean McConnachie. All rights reserved
