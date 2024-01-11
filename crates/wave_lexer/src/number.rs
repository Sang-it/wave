use std::borrow::Cow;

pub fn parse_int(s: &str) -> Result<f64, &'static str> {
    let s = Cow::Borrowed(s);
    s.parse::<f64>().map_err(|_| "invalid float")
}
