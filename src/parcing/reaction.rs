// src/parcing/reaction.rs
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct Reaction {
    pub entry: String,
    pub name: Option<String>,
    pub definition: Option<String>,
    pub enzymes: Vec<String>,
    pub left_compounds: Vec<String>,
    pub right_compounds: Vec<String>,
}

pub fn function_reaction(html: String) -> Option<Reaction> {
    let document = Html::parse_document(&html);

    let th_selector = Selector::parse("th").ok()?;
    
    let mut entry = None;
    let mut name = None;
    let mut definition = None;
    let mut enzymes = Vec::new();
    let mut equation = None;

    for th in document.select(&th_selector) {
        let key = th.text().collect::<String>().trim().to_string();
        
        let mut next = th.next_sibling();
        while let Some(node) = next {
            if node.value().is_element() {
                if let Some(td) = scraper::ElementRef::wrap(node) {
                    if td.value().name() == "td" {
                        let value = td.text().collect::<String>().trim().to_string();
                        
                        match key.as_str() {
                            "Entry" => {
                                entry = value.split_whitespace().next().map(String::from);
                            }
                            "Name" => {
                                name = Some(value);
                            }
                            "Definition" => {
                                definition = Some(value);
                            }
                            "Equation" => {
                                equation = Some(value);
                            }
                            "Enzyme" => {
                                if let Ok(link_sel) = Selector::parse("a") {
                                    let td_html = td.inner_html();
                                    let td_doc = Html::parse_fragment(&td_html);
                                    
                                    for link in td_doc.select(&link_sel) {
                                        let enzyme = link.text().collect::<String>().trim().to_string();
                                        if !enzyme.is_empty() {
                                            enzymes.push(enzyme);
                                        }
                                    }
                                }
                                
                                if enzymes.is_empty() && !value.is_empty() {
                                    for part in value.split_whitespace() {
                                        if part.contains('.') {
                                            enzymes.push(part.to_string());
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        break;
                    }
                }
                break;
            }
            next = node.next_sibling();
        }
    }

    let (left_compounds, right_compounds) = if let Some(eq) = equation {
        parse_equation(&eq)
    } else {
        (Vec::new(), Vec::new())
    };

    Some(Reaction {
        entry: entry?,
        name,
        definition,
        enzymes,
        left_compounds,
        right_compounds,
    })
}

fn parse_equation(equation: &str) -> (Vec<String>, Vec<String>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let clean_eq = equation.replace("<a>", "").replace("</a>", "");

    if let Some(parts) = clean_eq.split("<=>").collect::<Vec<_>>().split_first() {
        let left_part = parts.0.trim();
        let right_part = parts.1.first().unwrap_or(&"").trim();

        left = left_part
            .split('+')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        right = right_part
            .split('+')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    (left, right)
}

#[deprecated(note = "Use function_reaction instead")]
pub fn parse_kegg_reaction(html: &str) -> Option<Reaction> {
    function_reaction(html.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parcing::Parser;

    #[tokio::test]
    async fn test_function_reaction() {
        let mut parser = Parser::new();
        let url = "https://www.kegg.jp/entry/R00259";
        let html = parser.fetch_kegg(url).await.unwrap();
        let reaction = function_reaction(html).unwrap();
        
        println!("{:#?}", reaction);
        assert_eq!(reaction.entry, "R00259");
        assert_eq!(reaction.enzymes, vec!["2.3.1.1"]);
        assert_eq!(reaction.left_compounds, vec!["C00024", "C00025"]);
        assert_eq!(reaction.right_compounds, vec!["C00010", "C00624"]);
    }
}