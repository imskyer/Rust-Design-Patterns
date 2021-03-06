use report::Report;
use report_builder::ReportBuilder;

pub struct HtmlReportBuilder {
    content: String,
    items: i32
}

impl HtmlReportBuilder {
    pub fn new() -> HtmlReportBuilder {
        HtmlReportBuilder {
            content: "".to_string(),
            items: 0
        }
    }
}

impl ReportBuilder<HtmlReportBuilder> for HtmlReportBuilder {
    fn with_header(mut self, header: &'static str) -> HtmlReportBuilder {
        self.content.push_str(&format!("<h1>{}</h1>\n", header));
        self.items += 1;
        self
    }

    fn with_paragraph(mut self, paragraph: &'static str) -> HtmlReportBuilder {
        self.content.push_str(&format!("<p>{}</p>\n", paragraph));
        self.items += 1;
        self
    }

    fn finish(self) -> Report {
        Report {
            content: self.content,
            items: self.items,
            format: "Html\n".to_string()
        }
    }
}
