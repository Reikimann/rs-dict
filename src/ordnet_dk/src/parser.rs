use scraper::{
    Html,
    Selector,
    Element,
    selector::CssLocalName,
    CaseSensitivity,
};

use crate::utils::trim_whitespace;

// TODO: Remember to check if the inner_html is empty and what text to display instead.
// TODO: Write tests and documentation with examples (that can be tested).

pub fn get_match(doc: &Html) -> String {
    let selector_match = Selector::parse("div.definitionBoxTop > span.match").unwrap();

    let result_match: String = doc
        .select(&selector_match)
        .map(|el| el.inner_html())
        .collect::<Vec<_>>()
        .join("/");

    trim_whitespace(&result_match)
}

pub fn get_glossing(doc: &Html) -> String {
    let selector_glossing = Selector::parse("div.definitionBoxTop > span.tekstmedium.allow-glossing").unwrap();

    // BUG: Try searching "pis"; incorrect parsing.
    // Try one:
    // let result_glossing: String = doc
    //     .select(&selector_glossing)
    //     .map(|el| el.inner_html())
    //     .collect::<Vec<_>>()
    //     .join("");

    // Try two:
    let result_glossing: String = doc
        .select(&selector_glossing)
        .map(|el| {
            el.text().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("");

    trim_whitespace(&result_glossing)
}

pub fn get_inflection(doc: &Html) -> String {
    let selector_inflection = Selector::parse("div#id-boj > span.tekstmedium.allow-glossing").unwrap();

    let result_inflection: String = doc
        .select(&selector_inflection)
        .map(|el| {
            el.text().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("");

    trim_whitespace(&result_inflection)
}

pub fn get_pronounciation(doc: &Html) -> String {
    let selector_pronounciation = Selector::parse("div#id-udt > span.tekstmedium.allow-glossing").unwrap();
    let selector_span = Selector::parse("span").unwrap();

    let divider_class = CssLocalName("dividerDouble".into());
    let lydskrift_class = CssLocalName("lydskrift".into());

    let result_pronounciation = doc
        .select(&selector_pronounciation)
        .map(|el| el.inner_html())
        .map(|el| {
            let fragment = Html::parse_fragment(&el);

            let inner_texts: Vec<_> = fragment
                .select(&selector_span)
                .map(|el| {
                    let mut text = el.text().collect::<String>()
                        .replace("[", "").to_string()
                        .replace("]", "").to_string();

                    if !text.is_empty() {
                        text = text.replace("\u{a0}", "").to_string();
                        if el.has_class(&divider_class, CaseSensitivity::CaseSensitive) {
                            format!(" eller {}", text)
                        } else if el.has_class(&lydskrift_class, CaseSensitivity::CaseSensitive) {
                            format!(" [{}]", text)
                        } else {
                            format!(" {}", text)
                        }
                    } else {
                        String::new()
                    }
                })
                .collect();

            inner_texts.join("")
        })
        .collect::<Vec<_>>()
        .join("");

    trim_whitespace(&result_pronounciation)
}

pub fn get_etymology(doc: &Html) -> String {
    let selector_etymologi = Selector::parse("div#id-ety > span.tekstmedium.allow-glossing").unwrap();

    let result_etymology: String = doc
        .select(&selector_etymologi)
        .map(|el| {
            el.text().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("");

    trim_whitespace(&result_etymology)
}
