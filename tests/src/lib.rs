use webhunt::*;



        // let titles: Vec<String> =
        //     get_element_inner_html(&html_page, "h3>div>div.Title").any_err()?;

        // let anime_urls: Vec<String> =
        //     get_element_attribute(&html_page, "li.TPostMv>article>a", "href").any_err()?;


#[derive(Hunt, Debug)]
#[allow(dead_code)]
struct ComplexData {
    #[field(tag = "td.name", kind = Inner)]
    names: Vec<String>,
    // #[field(tag = "input[name='username']", kind = Attribute("value"))]
    // username: String,

    // #[field(tag = "option[selected]", kind = Attribute("value"))]
    // selected_country: String,
}

#[tokio::test]
async fn test_hunt_derive_complex() {
    let html = open("").await.unwrap();
    let data = ComplexData::from_html(&html);
    println!("{data:#?}");
    assert_eq!(data.names, vec!["Alice", "Bob", "Charlie"]);
    // assert_eq!(data.username, "test_user");
    // assert_eq!(data.selected_country, "us");
}
