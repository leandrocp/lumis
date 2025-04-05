use crate::languages::Language;

use super::Formatter;

pub trait HtmlFormatter: Formatter {
    fn lang(&self) -> Language;
    fn pre_class(&self) -> Option<&str>;

    fn write_pre_tag(&self) -> String {
        let class = if let Some(pre_class) = self.pre_class() {
            format!("athl {}", pre_class)
        } else {
            "athl".to_string()
        };

        format!("<pre class=\"{}\">", class)
    }

    fn write_code_tag(&self) -> String {
        format!(
            "<code class=\"language-{}\" translate=\"no\" tabindex=\"0\">",
            self.lang().id_name()
        )
    }
}