wit_bindgen_guest_rust::import!("../../wit/imports.wit");
wit_bindgen_guest_rust::export!("../../wit/exports.wit");

use std::time::SystemTime;
use wit_log;

struct Exports;

fn pretty(timestamp: u64) -> String {
    if timestamp < 10 * 1000 {
        return format!("{} ms", timestamp);
    } else if timestamp < 60 * 1000 {
        return format!("{:.2}s", (timestamp / 1000));
    }
    let minutes = timestamp / 60_000;
    let seconds = (timestamp % 60_000) / 1000;
    if minutes < 60 {
        return format!("{:.2}m and {:.2}s", minutes, seconds);
    }
    let hours = minutes / 60;
    let minutes = minutes % 60;
    if hours < 24 {
        return format!("{:.2}h, {:.2}m and {:.2}s", hours, minutes, seconds);
    }
    let days = hours / 24;
    let hours = hours % 24;
    format!(
        "{:.2}d, {:.2}h, {:.2}m and {:.2}s",
        days, hours, minutes, seconds
    )
}

impl exports::Exports for Exports {
    fn init() {
        let _ = log::set_boxed_logger(Box::new(crate::wit_log::WitLog::new()));
        log::set_max_level(log::LevelFilter::Trace);
        log::trace!("Called the init() method \\o/");
    }

    fn help() -> String {
        "Simple ping".to_owned()
    }

    fn on_msg(
        content: String,
        author_id: String,
        author_name: String,
        room: String,
        timestamp: u64,
        event_id: String,
    ) -> Vec<exports::Message> {
        if !content.starts_with("!ping") {
            return vec![];
        }

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let diff = now - timestamp;
        log::info!("{}, {}, {}", now, timestamp, diff);
        let pretty_diff = pretty(diff);
        let content_without_command = content.trim_start_matches("!ping").trim();
        let message = if content_without_command.is_empty() {
            format!("{}: Pong! (ping took {} to arrive)", author_id, pretty_diff)
        } else {
            format!(
                "{}: Pong! (ping \"{}\" took {} to arrive)",
                author_id, content_without_command, pretty_diff
            )
        };
        let message_formatted = if content_without_command.is_empty() {
            format!(
                "<a href=\"https://matrix.to/#/{}\">{}</a>: Pong! (<a href=\"https://matrix.to/#/{}/{}\">ping</a> took {} to arrive)",
                author_id, author_name,room,event_id, pretty_diff
            )
        } else {
            format!(
                "<a href=\"https://matrix.to/#/{}\">{}</a>: Pong! (<a href=\"https://matrix.to/#/{}/{}\">ping</a> \"{}\" took {} to arrive)",
                author_id, author_name,room,event_id,content_without_command, pretty_diff
            )
        };

        let content = message;
        vec![exports::Message {
            content,
            formatted_content: exports::OptionalString::Some(message_formatted),
            to: author_id,
            pong: exports::OptionalPong::Some(diff),
        }]
    }
}
