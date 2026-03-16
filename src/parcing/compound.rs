use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct Compound {
    pub entry: String,
    pub formula: Option<String>,
    pub exact_mass: Option<f64>,
    pub mol_weight: Option<f64>,
    pub names: Vec<String>,
    pub substrates_for: Vec<String>,
    pub products_for: Vec<String>,
}

pub fn function_compound(html: String) -> Option<Compound> {
    let document = Html::parse_document(&html);

    let entry_sel = Selector::parse("div h1 a").ok()?;
    let formula_sel = Selector::parse("th:contains('Formula') + td").ok()?;
    let exact_mass_sel = Selector::parse("th:contains('Exact mass') + td").ok()?;
    let mol_weight_sel = Selector::parse("th:contains('Mol weight') + td").ok()?;
    let names_sel = Selector::parse("th:contains('Other names') + td span").ok()?;
    let substrates_sel = Selector::parse("th:contains('Substrate') + td a").ok()?;
    let products_sel = Selector::parse("th:contains('Product') + td a").ok()?;

    let entry = document
        .select(&entry_sel)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let formula = document
        .select(&formula_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let exact_mass = document
        .select(&exact_mass_sel)
        .next()
        .and_then(|el| {
            el.text()
                .collect::<String>()
                .trim()
                .replace(",", "")
                .parse::<f64>()
                .ok()
        });

    let mol_weight = document
        .select(&mol_weight_sel)
        .next()
        .and_then(|el| {
            el.text()
                .collect::<String>()
                .trim()
                .replace(",", "")
                .parse::<f64>()
                .ok()
        });

    let names = document
        .select(&names_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let substrates_for = document
        .select(&substrates_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let products_for = document
        .select(&products_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Some(Compound {
        entry,
        formula,
        exact_mass,
        mol_weight,
        names,
        substrates_for,
        products_for,
    })
}