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

pub async fn open(url: &str) -> Result<MyHtml, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let document = response.text().await?;
    Ok(MyHtml::new(Html::parse_document(&document)))
}

pub struct MyHtml {
    inner: Html,
}

impl MyHtml {
    fn new(inner: Html) -> Self {
        Self { inner }
    }

    pub fn get_inner<'i, T: FromIterator<String>>(&self, tag: &'i str) -> Result<T, Error<'i>> {
        let selector = Selector::parse(tag)?;
        Ok(self
            .inner
            .select(&selector)
            .map(|elt| elt.inner_html())
            .collect())
    }

    pub fn get_inner_map<'i, R, T: FromIterator<R>>(
        &self,
        tag: &'i str,
        func: fn(String) -> R,
    ) -> Result<T, Error<'i>> {
        let selector = Selector::parse(tag)?;
        Ok(self
            .inner
            .select(&selector)
            .map(|elt| elt.inner_html())
            .map(func)
            .collect())
    }

    pub fn get_attr<'a, T: FromIterator<String>>(
        &self,
        tag: &'a str,
        attr: &'a str,
    ) -> Result<T, Error<'a>> {
        let selector = Selector::parse(tag)?;
        self.inner
            .select(&selector)
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
}

pub trait Hunt {
    fn from_html<'a>(html: &Html) -> Result<Self, Error<'a>>
    where
        Self: Sized;
}
