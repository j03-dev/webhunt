use webhunt::*;

#[allow(dead_code)]
const HTML: &'static str = r#"
        <!DOCTYPE html>
        <html>
        <body>
            <table id="users">
                <tr>
                    <td class="name">Alice</td>
                    <td class="age">25</td>
                    <td class="email">alice@example.com</td>
                </tr>
                <tr>
                    <td class="name">Bob</td>
                    <td class="age">30</td>
                    <td class="email">bob@example.com</td>
                </tr>
                <tr>
                    <td class="name">Charlie</td>
                    <td class="age">35</td>
                    <td class="email">charlie@example.com</td>
                </tr>
            </table>
            <form>
                <input type="text" name="username" value="test_user">
                <input type="hidden" name="csrf_token" value="abc123">
                <select name="country">
                    <option value="us" selected>United States</option>
                    <option value="uk">United Kingdom</option>
                </select>
            </form>
        </body>
        </html>
        "#;

#[test]
fn test_get_element_attribute_complex() {
    let html = Html::parse_document(HTML);

    let names: Vec<String> = get_element_attribute(&html, "td.name", "class").unwrap_or_default();
    assert_eq!(names, vec!["name", "name", "name"]);

    let emails: Vec<String> = get_element_attribute(&html, "td.email", "class").unwrap_or_default();
    assert_eq!(emails, vec!["email", "email", "email"]);

    let input_values: Vec<String> =
        get_element_attribute(&html, "input[name]", "name").unwrap_or_default();
    assert!(input_values.contains(&"username".to_string()));
    assert!(input_values.contains(&"csrf_token".to_string()));
}

#[derive(crate::Hunt)]
#[allow(dead_code)]
struct ComplexData {
    #[select(tag = "td.name")]
    names: Vec<String>,

    #[select(tag = "input[name='username']", attr = "value")]
    username: String,

    #[select(tag = "option[selected]", attr = "value")]
    selected_country: String,
}

#[test]
fn test_hunt_derive_complex() {
    let html = Html::parse_document(HTML);
    let data = ComplexData::from_html(&html);
    assert!(data.is_ok());
    let data = data.unwrap();
    assert_eq!(data.names, vec!["Alice", "Bob", "Charlie"]);
    assert_eq!(data.username, "test_user");
    assert_eq!(data.selected_country, "us");
}
