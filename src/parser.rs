use regex::Regex;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::collections::HashMap;

pub fn parse_title(doc: &Document) -> String {
    let title_raw = doc.find(Attr("id", "productTitle")).next().unwrap().text();
    let title_clean = title_raw.trim();

    title_clean.to_string()
}

pub fn parse_ranks(doc: &Document) -> HashMap<String, String> {
    let mut ranks: HashMap<String, String> = HashMap::new();

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
    }

    ranks
}

// pub fn parse_ranks(doc: &Document) -> HashMap<String, String> {
//     let mut ranks: HashMap<String, String> = HashMap::new();

//     for node in doc.find(Class("zg_hrsr_item")) {
//         // rank
//         let rank = node.find(Class("zg_hrsr_rank")).next().unwrap().text();

//         // title
//         let group = node
//             .find(Class("zg_hrsr_ladder").descendant(Name("a")))
//             .next()
//             .unwrap()
//             .text();

//         let clean_group = group
//             .replace("&amp;", "")
//             .replace("\n", "")
//             .replace(";", "");
//         let re = Regex::new(r"\s+").unwrap();
//         let after = re.replace_all(clean_group.as_str(), " ");
//         ranks.insert(rank, after.to_string());
//     }

//     ranks
// }
