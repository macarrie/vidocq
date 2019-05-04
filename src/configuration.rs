pub struct CliOptions<'a> {
    pub media_type: Option<&'a str>,
}

impl<'a> Default for CliOptions<'a> {
    fn default() -> Self {
        CliOptions { media_type: None }
    }
}
