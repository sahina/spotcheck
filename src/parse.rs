use regex::Regex;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub struct Ranking {
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
}

pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
  reqwest::get(url).await?.text().await
}

pub fn to_doc(html: &str) -> Ranking {
  let document = Document::from(html);

  let title = parse_title(&document);

  Ranking {
    asin: "asin".to_string(),
    ranks: vec!["1".to_string()],
  }
}

pub fn parse_title(doc: &Document) -> String {
  return doc.find(Attr("id", "productTitle")).next().unwrap().text();
}

pub fn to_page_title(doc: &Document) -> String {
  return doc.find(Name("h1")).next().unwrap().text();
}

pub fn parse_rank(doc: &Document) -> Ranking {
  let ranking = Ranking::new();

  for (index, node) in doc.find(Class("zg_hrsr_item")).enumerate() {
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
  }

  return ranking;
}

#[cfg(test)]
mod test {
  use super::*;
  use regex::Regex;
  use select::document::Document;
  use select::predicate::{Class, Name, Predicate};

  #[test]
  fn title_from_html() {
    // load test html
    let document = Document::from(include_str!("content.html"));
    // parse title
    let title = document.find(Name("h1")).next().unwrap().text();
    assert_eq!("My Title", title);
  }

  #[test]
  fn ranking_from_html() {
    let expected_ranks = vec!["#3265", "#1152", "#5281"];
    let expected_titles = vec![
      "Science Fiction TV, Movie & Game Tie-In",
      "Trivia (Kindle Store)",
      "Trivia (Books)",
    ];

    // load test html
    let document = Document::from(include_str!("content.html"));

    // loop all ranking elements
    for (index, node) in document.find(Class("zg_hrsr_item")).enumerate() {
      // parse rank
      let rank = node.find(Class("zg_hrsr_rank")).next().unwrap().text();
      assert_eq!(expected_ranks[index], rank);

      // parse title
      // let name = Name("a").child("a").descendant("a");
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

      assert_eq!(expected_titles[index], after);
    }
  }

  #[test]
  fn page_title_from_doc() {
    let document = Document::from(include_str!("content.html"));
    let title = to_page_title(&document);

    assert_eq!("My Title", title);
  }

  #[test]
  fn ranking_from_doc() {
    let document = Document::from(include_str!("content.html"));
  }
}
