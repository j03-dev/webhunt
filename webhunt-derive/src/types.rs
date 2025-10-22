use deluxe::{ExtractAttributes, ParseMetaItem};

#[derive(ParseMetaItem)]
pub enum Kind {
    Inner,
    Attribute(String),
}

impl Default for Kind {
    fn default() -> Self {
        Self::Inner
    }
}

#[derive(ExtractAttributes, Default)]
#[deluxe(attributes(args))]
pub struct Args {
    pub tag: String,
    pub kind: Kind,
}
