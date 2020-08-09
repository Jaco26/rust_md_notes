use uuid::Uuid;
use comrak;

pub fn uuid() -> String {
  Uuid::new_v4().to_string()
}

#[derive(Clone)]
pub struct Markdown {
    options: comrak::ComrakOptions,
}

impl Markdown {
    pub fn new() -> Markdown {
        let mut options = comrak::ComrakOptions::default();
        options.extension.footnotes = true;
        options.extension.superscript = true;
        options.extension.tasklist = true;
        options.extension.header_ids = Some(String::from(""));
        options.extension.table = true;
        options.extension.tagfilter = true;
        options.extension.strikethrough = true;
        Markdown { options }
    }
    pub fn html(&self, md: &str) -> String {
        comrak::markdown_to_html(md, &self.options)
    }
}