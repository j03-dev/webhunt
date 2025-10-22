use deluxe::ExtractAttributes;

#[derive(ExtractAttributes, Default)]
#[deluxe(attributes(select))]
pub struct Select {
    pub tag: String,
    pub attr: Option<String>,
}
