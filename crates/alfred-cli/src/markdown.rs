use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

pub fn render_markdown(text: &str, base_style: Style) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut current_line_spans = Vec::new();
    let mut style_stack = vec![base_style];

    let parser = Parser::new(text);
    for event in parser {
        match event {
            Event::Text(t) => {
                let style = *style_stack.last().unwrap_or(&base_style);
                // Handle newlines within text if necessary, or let pulldown handle blocks
                // Simple approach: split by newline
                let parts: Vec<&str> = t.split('\n').collect();
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        lines.push(Line::from(current_line_spans.clone()));
                        current_line_spans.clear();
                    }
                    if !part.is_empty() {
                        current_line_spans.push(Span::styled(part.to_string(), style));
                    }
                }
            }
            Event::Code(c) => {
                let style = *style_stack.last().unwrap_or(&base_style);
                let code_style = style.fg(Color::Yellow); // Example code color
                current_line_spans.push(Span::styled(c.to_string(), code_style));
            }
            Event::Start(Tag::Emphasis) => {
                let mut style = *style_stack.last().unwrap_or(&base_style);
                style = style.add_modifier(Modifier::ITALIC);
                style_stack.push(style);
            }
            Event::End(TagEnd::Emphasis) => {
                style_stack.pop();
            }
            Event::Start(Tag::Strong) => {
                let mut style = *style_stack.last().unwrap_or(&base_style);
                style = style.add_modifier(Modifier::BOLD);
                style_stack.push(style);
            }
            Event::End(TagEnd::Strong) => {
                style_stack.pop();
            }
            Event::Start(Tag::Paragraph) => {
                // New block
            }
            Event::End(TagEnd::Paragraph) => {
                if !current_line_spans.is_empty() {
                    lines.push(Line::from(current_line_spans.clone()));
                    current_line_spans.clear();
                }
                // Add margin?
                lines.push(Line::from("")); 
            }
            Event::Start(Tag::CodeBlock(_)) => {
                 if !current_line_spans.is_empty() {
                    lines.push(Line::from(current_line_spans.clone()));
                    current_line_spans.clear();
                }
            }
             Event::End(TagEnd::CodeBlock) => {
                 if !current_line_spans.is_empty() {
                    lines.push(Line::from(current_line_spans.clone()));
                    current_line_spans.clear();
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                 lines.push(Line::from(current_line_spans.clone()));
                 current_line_spans.clear();
            }
            _ => {}
        }
    }

    if !current_line_spans.is_empty() {
        lines.push(Line::from(current_line_spans));
    }

    lines
}