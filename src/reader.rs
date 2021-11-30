pub struct Reader {}

impl Reader {
    pub fn new<S: Into<String>, U: Into<String>>(_s: Option<S>, _t: Option<U>) -> Self {
        Reader{}
    }
}
