wit_bindgen_guest_rust::import!("../../wit/imports.wit");
wit_bindgen_guest_rust::export!("../../wit/exports.wit");

struct Exports;

impl exports::Exports for Exports {
    fn init() {
        let _ = log::set_boxed_logger(Box::new(wit_log::WitLog::new()));
        log::set_max_level(log::LevelFilter::Trace);
        log::trace!("Called the init() method \\o/");
    }

    fn help() -> String {
        "Simple echo".to_owned()
    }

    fn on_msg(
        content: String,
        author_id: String,
        _author_name: String,
        _room: String,
        _timestamp: u64,
        _event_id: String,
    ) -> Vec<exports::Message> {
        if !content.starts_with("!echo") {
            return vec![];
        }

        let content = content.replace("!echo", "");
        vec![exports::Message {
            content,
            formatted_content: exports::OptionalString::None,
            to: author_id,
            pong: exports::OptionalPong::None,
        }]
    }
}
