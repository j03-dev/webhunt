pub use scraper::{Html, Selector};
pub use webhunt_derive::Hunt;

pub type Error = Box<dyn std::error::Error>;

pub fn get_element_inner_html<T: FromIterator<String>>(html: &Html, tag: &str) -> Result<T, Error> {
    let selector = Selector::parse(Box::leak(tag.to_string().into_boxed_str()))?;
    Ok(html.select(&selector).map(|elt| elt.inner_html()).collect())
}

pub fn get_element_attribute<T: FromIterator<String>>(
    html: &Html,
    tag: &str,
    attr: &str,
) -> Result<T, Error> {
    let selector = Selector::parse(Box::leak(tag.to_string().into_boxed_str()))?;
    Ok(html
        .select(&selector)
        .map(|elt| {
            elt.value()
                .attr(attr)
                .expect("This attribute is not exist")
                .to_string()
        })
        .collect())
}

pub async fn open(url: &str) -> Result<Html, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let document = response.text().await?;
    Ok(Html::parse_document(&document))
}

pub trait Hunt {
    fn from_html(html: &Html) -> Result<Self, Error>
    where
        Self: Sized;
}
