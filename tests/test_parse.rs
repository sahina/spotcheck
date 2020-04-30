#[cfg(test)]
mod tests {
  use regex::Regex;
  use select::document::Document;
  use select::predicate::{Class, Name, Predicate};

  #[test]
  fn title() {
    // load test html
    let document = Document::from(include_str!("content.html"));
    // parse title
    let title = document.find(Name("h1")).next().unwrap().text();
    assert_eq!("My Title", title);
  }

  #[test]
  fn ranking() {
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
}
