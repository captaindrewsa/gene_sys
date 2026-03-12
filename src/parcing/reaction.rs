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

    let entry_sel = Selector::parse("div h1 a").ok()?;
    let name_sel = Selector::parse("th:contains('Name') + td").ok()?;
    let definition_sel = Selector::parse("th:contains('Definition') + td").ok()?;
    let enzymes_sel = Selector::parse("th:contains('Enzyme') + td a").ok()?;
    let left_sel = Selector::parse("th:contains('Equation') + td .left a").ok()?;
    let right_sel = Selector::parse("th:contains('Equation') + td .right a").ok()?;

    let entry = document
        .select(&entry_sel)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let name = document
        .select(&name_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let definition = document
        .select(&definition_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let enzymes = document
        .select(&enzymes_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let left_compounds = document
        .select(&left_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let right_compounds = document
        .select(&right_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Some(Reaction {
        entry,
        name,
        definition,
        enzymes,
        left_compounds,
        right_compounds,
    })
}