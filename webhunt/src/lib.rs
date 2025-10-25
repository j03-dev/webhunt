pub use scraper::{Html, Selector};
pub use webhunt_derive::Hunt;

pub type Error<'a> = scraper::error::SelectorErrorKind<'a>;

pub fn get_element_inner_html<'i, T: FromIterator<String>>(
    html: &Html,
    tag: &'i str,
) -> Result<T, Error<'i>> {
    let selector = Selector::parse(tag)?;
    Ok(html.select(&selector).map(|elt| elt.inner_html()).collect())
}

pub fn get_element_attribute<'a, T: FromIterator<String>>(
    html: &Html,
    tag: &'a str,
    attr: &'a str,
) -> Result<T, Error<'a>> {
    let selector = Selector::parse(tag)?;
    html.select(&selector)
        .map(|elt| {
            elt.value()
                .attr(attr)
                .ok_or_else(|| {
                    Error::InvalidAtRule(format!(
                        "Attribute '{attr}' not found on element selected by '{tag}'"
                    ))
                })
                .map(|value| value.to_string())
        })
        .collect()
}

pub async fn open(url: &str) -> Result<Html, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let document = response.text().await?;
    Ok(Html::parse_document(&document))
}

pub trait Hunt {
    fn from_html<'a>(html: &Html) -> Result<Self, Error<'a>>
    where
        Self: Sized;
}
