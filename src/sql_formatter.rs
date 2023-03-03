use sqlformat::{format, FormatOptions, Indent, QueryParams};

pub struct SqlFormatter;

impl SqlFormatter {
    pub fn format(sql_code: &str) -> String {
        let options = FormatOptions {
            indent: Indent::Spaces(4),
            ..FormatOptions::default()
        };
        format(sql_code, &QueryParams::None, options)
    }
}
