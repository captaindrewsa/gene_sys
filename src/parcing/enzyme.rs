use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct Enzyme {
    pub entry: String,
    pub sysname: Option<String>,
    pub reaction_iubmb: Option<String>,
    pub names: Vec<String>,
    pub substrates: Vec<String>,
    pub products: Vec<String>,
    pub reactions: Vec<String>,
}

pub fn function_enzyme(html: String) -> Option<Enzyme> {
    let document = Html::parse_document(&html);

    let entry_sel = Selector::parse("div h1 a").ok()?;
    let sysname_sel = Selector::parse("th:contains('Sysname') + td").ok()?;
    let reaction_sel = Selector::parse("th:contains('Reaction') + td").ok()?;
    let names_sel = Selector::parse("th:contains('Other names') + td span").ok()?;
    let substrates_sel = Selector::parse("th:contains('Substrate') + td a").ok()?;
    let products_sel = Selector::parse("th:contains('Product') + td a").ok()?;
    let reactions_sel = Selector::parse("th:contains('Reaction') + td a").ok()?;

    let entry = document
        .select(&entry_sel)
        .next()?
        .text()
        .collect::<String>()
        .replace("EC ", "")
        .trim()
        .to_string();

    let sysname = document
        .select(&sysname_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let reaction_iubmb = document
        .select(&reaction_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let names = document
        .select(&names_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let substrates = document
        .select(&substrates_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let products = document
        .select(&products_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let reactions = document
        .select(&reactions_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Some(Enzyme {
        entry,
        sysname,
        reaction_iubmb,
        names,
        substrates,
        products,
        reactions,
    })
}