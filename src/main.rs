// use std::env;

use select::document::Document;
use select::predicate::{Class, Name};

mod options;
mod parse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    // let cli_args = options::Args::new(&args);
    // let asin = cli_args.unwrap().value;
    // let url = format!("https://amzn.to/{}", asin);
    // let response = reqwest::get(&url).await?.text().await?;

    // span.id=productTitle
    // span.id=productSubtitle
    // li.id=SalesRank
    // ul.class=zg_hrsr
    //   li.class=zg_hrsr_item
    //     span.class=zg_hrsr_rank
    //     span.class=zg_hrsr_ladder
    // let document = Document::from(include_str!("content.html"));

    // get_rankings(&response);
    Ok(())
}

// fn get_rankings(response: &str) {
//     Document::from(&*response)
//         .find(Name("a"))
//         .filter_map(|n| n.attr("href"))
//         .for_each(|x| println!("{}", x));
// }
