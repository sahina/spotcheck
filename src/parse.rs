use regex::Regex;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

struct Ranking {
  asin: String,
  ranks: Vec<String>,
}

impl Ranking {
  fn new() -> Ranking {
    Ranking {
      asin: String::new(),
      ranks: Vec::new(),
    }
  }

  fn new_with(asin: String) -> Ranking {
    Ranking {
      asin: asin,
      ranks: Vec::new(),
    }
  }
}

pub fn parse(html: &str) -> Ranking {
  let document = Document::from(html);

  let title = parse_title(&document);

  // loop all ranking elements
  for (index, node) in document.find(Class("zg_hrsr_item")).enumerate() {
    // parse rank number
    let rank = node.find(Class("zg_hrsr_rank")).next().unwrap().text();

    // parse rank section
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

    Ranking {
      asin: "asin",
      ranks: vec!["1", "2", "3"],
    }
  }
}

pub fn parse_title(document: &Document) -> String {
  return document
    .find(Attr("id", "productTitle"))
    .next()
    .unwrap()
    .text();
}

pub fn parse_rank(document: &Document) -> Vec<Ranking> {
  let mut rankings: Vec<Ranking> = Vec::new();

  for (index, node) in document.find(Class("zg_hrsr_item")).enumerate() {
    // parse rank number
    let rank = node.find(Class("zg_hrsr_rank")).next().unwrap().text();

    // parse rank section
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

    rankings.push(Ranking::new_with(String::from("asin")));

    return rankings;
  }
}
