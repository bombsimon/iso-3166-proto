use chrono::prelude::{DateTime, Utc};
use handlebars::Handlebars;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

const IBAN_URI: &str = "https://www.iban.com/country-codes";
const TEMPLATE_V1: &str = r#"// DO NOT EDIT! This file is generated with https://github.com/bombsimon/iso-3166-proto
// Proto last generated at {{time}}.
//
// This is a complete list of all country ISO codes as described in the ISO 3166 international
// standard.
//
// These codes are used throughout the IT industry by computer systems and software to ease the
// identification of country names.
//
// We have compiled them in the quick reference table below in order to help our clients do quick
// conversions from the numeric or 2 letter code to any country name.
//
// The list is generated from https://www.iban.com/country-codes
syntax = "proto3";

package iso3166.v1;

// ISO 3166-1 alpha-2 codes are two-letter country codes defined in ISO 3166-1, part of the ISO
// 3166 standard published by the International Organization for Standardization (ISO), to
// represent countries, dependent territories, and special areas of geographical interest.
enum Alpha2 {
{{#each countries}}
    {{this.alpha_2}} = {{@index}}; // {{this.name}}
{{/each}}
}

// ISO 3166-1 alpha-3 codes are three-letter country codes defined in ISO 3166-1, part of the ISO
// 3166 standard published by the International Organization for Standardization (ISO), to
// represent countries, dependent territories, and special areas of geographical interest. They
// allow a better visual association between the codes and the country names than the two-letter
// alpha-2 codes (the third set of codes is numeric and hence offers no visual association).
enum Alpha3 {
{{#each countries}}
    {{this.alpha_3}} = {{@index}}; // {{this.name}}
{{/each}}
}
"#;

#[derive(Serialize, Deserialize, Debug)]
struct CountryCode {
    name: String,
    alpha_2: String,
    alpha_3: String,
    numeric: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let uri = IBAN_URI.parse()?;
    let resp = client.get(uri).await?;
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let html = String::from_utf8(bytes.to_vec())?;

    let document = Html::parse_document(&html);
    let tbody = Selector::parse("tbody tr").unwrap();
    let td = Selector::parse("td").unwrap();

    let countries = document
        .select(&tbody)
        .map(|element| {
            let mut cols = element.select(&td);

            CountryCode {
                name: cols.next().unwrap().inner_html(),
                alpha_2: cols.next().unwrap().inner_html(),
                alpha_3: cols.next().unwrap().inner_html(),
                numeric: cols.next().unwrap().inner_html().parse::<u32>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let dt: DateTime<Utc> = SystemTime::now().into();

    let mut hb = Handlebars::new();
    hb.register_template_string("proto_v1", TEMPLATE_V1)
        .unwrap();

    let j = &serde_json::json!({ "countries": countries, "time": dt.format("%+").to_string() });
    println!("{}", hb.render("proto_v1", j).unwrap());

    Ok(())
}
