use converter_buddy_webui::serve;
use converter_buddy_webui::services::conversions::injector::get_local_service;

fn main() {
    serve(get_local_service());
}
