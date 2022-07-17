lazy_static! {
    pub static ref STEAM_WEB_API_KEY: String =
        std::env::var("STEAM_WEB_API_KEY").expect("STEAM_WEB_API_KEY");
}
