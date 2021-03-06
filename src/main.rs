use rich_strong_lib::{server::Server, head::LineType, config::Config, log::Log};

fn main() {
    Log::create_log_dir();
    
    let app_addr = Config::init();
    Config::init_client_side_setting();
    
    let mut app = Server::new(app_addr,LineType::Fox);
    
    app.init();
    app.start();
}
