pub struct AppConfig {
    // common
    pub dev_mode: bool,

    pub server_hostname: String,
    pub server_port: u16,

    // development
    pub frontend_url: String,
    pub frontend_prefix: String,

    // production
    pub assets_dir: String,
    pub assets_prefix: String,

    pub public_dir: String,

    pub templates_dir: String,
}

impl AppConfig {
    pub fn load() -> Self {
        use dotenvy::var;

        Self {
            // common
            dev_mode: cfg!(feature = "dev"),

            server_hostname: "0.0.0.0".to_string(),
            server_port: match var("PORT") {
                Ok(value) => value.parse::<u16>().unwrap_or(8080),
                Err(_) => 8080,
            },

            // development
            frontend_url: "http://localhost:8081".into(),
            frontend_prefix: "/templates".into(),

            // production
            assets_dir: var("ASSETS_DIR").expect("ASSETS_DIR must be set"),
            assets_prefix: "/assets".into(),

            public_dir: var("PUBLIC_DIR").expect("PUBLIC_DIR must be set"),

            templates_dir: var("TEMPLATES_DIR").expect("TEMPLATES_DIR must be set"),
        }
    }
}
