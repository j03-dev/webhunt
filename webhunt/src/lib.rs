pub use scraper::{Html, Selector};

const ERR_MSG: &str = "Failed to parse this html";

pub fn get_element_inner_html<T: FromIterator<String>>(html: &Html, tag: &str) -> T {
    let selector = Selector::parse(tag).expect(ERR_MSG);
    html.select(&selector).map(|elt| elt.inner_html()).collect()
}

pub fn get_element_attribute<T: FromIterator<String>>(html: &Html, tag: &str, attr: &str) -> T {
    let selector = Selector::parse(tag).expect(ERR_MSG);
    html.select(&selector)
        .map(|elt| {
            elt.value()
                .attr(attr)
                .expect("This attribue is not exist")
                .to_string()
        })
        .collect()
}

pub async fn open(url: String) -> Result<Html, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let document = response.text().await.unwrap();
    Ok(Html::parse_document(&document))
}

trait Hunt {
    const URL: &srt;
}
