use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;
use std::env;
use std::error::Error;

mod options;

const BASE: &str = "http://amzn.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get asin from arg
    let args: Vec<String> = env::args().collect();
    let cli_args = options::Args::new(&args);
    let asin = cli_args.unwrap().value;

    // get html
    let url = format!("{}{}", BASE, asin);
    let html = reqwest::get(&url).await?.text().await?;

    // convert to doc to parse
    let doc = Document::from(&*html);

    // parse page title
    let title_raw = doc.find(Attr("id", "productTitle")).next().unwrap().text();
    let title_clean = title_raw.trim();

    println!("{}", title_clean);

    // loop all ranking elements
    let mut ranks = HashMap::new();

    for node in doc.find(Class("zg_hrsr_item")) {
        // rank
        let rank = node.find(Class("zg_hrsr_rank")).next().unwrap().text();

        // title
        let group = node
            .find(Class("zg_hrsr_ladder").descendant(Name("a")))
            .next()
            .unwrap()
            .text();

        let clean_group = group
            .replace("&amp;", "")
            .replace("\n", "")
            .replace(";", "");
        let re = Regex::new(r"\s+").unwrap();
        let after = re.replace_all(clean_group.as_str(), " ");
        ranks.insert(rank, after.to_string());
        println!("{:?}", ranks);
    }

    Ok(())
}
