use ratatui::{
    style::Modifier,
    text::{Line, Span},
};

#[derive(Default)]
pub struct LineBuilder<'a> {
    pub spans: Vec<Span<'a>>,
}

impl<'a> LineBuilder<'a> {
    pub fn new(text: &'a str) -> Self {
        LineBuilder {
            spans: vec![Span::from(text)],
        }
    }

    pub fn build(self) -> Line<'a> {
        Line::from(self.spans)
    }

    pub fn bold_first_char(mut self) -> Self {
        if self.spans.is_empty() {
            return self;
        };
        if let Some(first_span) = self.spans.first_mut() {
            let content = first_span.to_string();
            let mut chars = content.char_indices();

            if chars.next().is_some() {
                let split = chars.next().map(|f| f.0).unwrap_or(content.len());
                let first_char = content[..split].to_owned();
                let rest = content[split..].to_owned();

                *first_span = Span::styled(first_char, Modifier::BOLD);
                self.spans.insert(1, Span::from(rest));
            }
        }

        self
    }

    pub fn surround_first_letter_with_brackets(mut self) -> Self {
        self.spans.insert(0, Span::from('['.to_string()));
        self.spans.insert(2, Span::from(']'.to_string()));

        self
    }
}
