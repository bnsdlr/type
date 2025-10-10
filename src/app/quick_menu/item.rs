use crate::{Theme, typing::Mode};

use ratatui::{prelude::*, widgets::ListItem};

pub enum QuickMenuItem {
    Category {
        label: Option<String>,
        options: Vec<QuickMenuItem>,
    },
    // IntInput { label: String, input: Vec<usize>, placeholder: String },
    Theme(Theme),
    Mode(Mode),
}

impl QuickMenuItem {
    pub fn category(label: Option<String>, options: Vec<QuickMenuItem>) -> Self {
        Self::Category { label, options }
    }

    pub fn label(self, label: Option<String>) -> Self {
        match self {
            Self::Category { options, .. } => Self::Category { label, options },
            s => s,
        }
    }

    // pub fn int_input(label: String, placeholder: String) -> Self {
    //     Self::IntInput { label, input: Vec::new(), placeholder }
    // }

    pub fn as_list_items(&self, list_width: u16, in_category: Option<&str>) -> Vec<ListItem> {
        self.as_list_items_filtered(in_category, list_width, &[])
    }

    fn matches(words: &[String], string: &str) -> bool {
        for word in words {
            if !string.contains(word) {
                return false;
            }
        }
        true
    }

    pub fn as_list_items_filtered(
        &self,
        in_category: Option<&str>,
        list_width: u16,
        words: &[String],
    ) -> Vec<ListItem> {
        match self {
            Self::Category { label, options } => {
                let category = Self::add_to_category(in_category, label.as_deref());
                let items: Vec<_> = options
                    .iter()
                    .flat_map(|opt| opt.as_list_items_filtered(category.as_deref(), list_width, words))
                    .collect();

                items
            }
            // Self::IntInput { label, .. } => {
            // },
            Self::Theme(theme) => {
                let category =
                    Self::add_to_category(in_category, Some(&theme.name)).unwrap_or_default();

                let matches = Self::matches(words, &category);
                let current_len = category.len();
                let mut line = Line::raw(category);

                line.push_span(Span::raw(format!("{:>1$}", " ", list_width as usize - current_len - 4)));
                line.push_span(Span::styled(" ", Style::new().bg(theme.main)));
                line.push_span(Span::raw(" "));
                line.push_span(Span::styled(" ", Style::new().bg(theme.bg)));
                line.push_span(Span::raw(" "));
                line.push_span(Span::styled(" ", Style::new().bg(theme.text)));

                if matches {
                    vec![ListItem::new(line)]
                } else {
                    Vec::new()
                }
            }
            Self::Mode(mode) => {
                Vec::new()
            }
        }
    }

    fn add_to_category(category: Option<&str>, text: Option<&str>) -> Option<String> {
        match (category, text) {
            (Some(category), Some(text)) => Some(format!("{category}󰅂 {text}")),
            (None, Some(text)) => Some(text.to_string()),
            (Some(category), None) => Some(category.to_string()),
            (None, None) => None,
        }
    }
}

impl<A> FromIterator<A> for QuickMenuItem
where
    A: Into<QuickMenuItem>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::Category {
            label: None,
            options: iter.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<Theme> for QuickMenuItem {
    fn from(value: Theme) -> Self {
        QuickMenuItem::Theme(value)
    }
}
