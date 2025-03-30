use html_escape::encode_safe;
use markdown::to_html;

/// Formats the article content for display
/// 
/// This function handles both HTML and plain text, applying proper formatting.
/// If the content is markdown, it will convert it to HTML.
pub fn format_article_content(content: &str) -> String {
    // If content is likely to be markdown, convert it to HTML
    if content.contains("##") || content.contains("*") || content.contains("```") {
        return to_html(content);
    }

    // If content already has HTML tags, just encode special characters in text content
    if content.contains("<") && content.contains(">") {
        return content.to_string();
    }

    // If it's plain text, convert newlines to <br> tags
    let escaped = encode_safe(content).to_string();
    escaped.replace("\n", "<br>")
}

/// Truncates a string to the specified length, ending with an ellipsis
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut truncated = s.chars().take(max_len).collect::<String>();
        truncated.push_str("...");
        truncated
    }
}

/// Converts a Russian month name to a number
pub fn month_name_to_number(month: &str) -> Option<u32> {
    match month.to_lowercase().as_str() {
        "январь" => Some(1),
        "февраль" => Some(2),
        "март" => Some(3),
        "апрель" => Some(4),
        "май" => Some(5),
        "июнь" => Some(6),
        "июль" => Some(7),
        "август" => Some(8),
        "сентябрь" => Some(9),
        "октябрь" => Some(10),
        "ноябрь" => Some(11),
        "декабрь" => Some(12),
        _ => None,
    }
}

/// Converts a month number to a Russian month name
pub fn month_number_to_name(month: u32) -> Option<String> {
    match month {
        1 => Some("Январь".to_string()),
        2 => Some("Февраль".to_string()),
        3 => Some("Март".to_string()),
        4 => Some("Апрель".to_string()),
        5 => Some("Май".to_string()),
        6 => Some("Июнь".to_string()),
        7 => Some("Июль".to_string()),
        8 => Some("Август".to_string()),
        9 => Some("Сентябрь".to_string()),
        10 => Some("Октябрь".to_string()),
        11 => Some("Ноябрь".to_string()),
        12 => Some("Декабрь".to_string()),
        _ => None,
    }
}