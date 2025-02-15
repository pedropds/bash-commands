mod clip_history;
mod clip_model;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "list" => clip_history::list_history(),
            "clear" => clip_history::clear_history(),
            _ => eprintln!("Unknown command"),
        }
    } else {
        clip_history::listen_clipboard();
    }
}
