use minijinja::Environment;
use reqwest::Client;
use serde::Serialize;

use crate::config::AppConfig;

pub enum ViewRenderer {
    Dev {
        env: Environment<'static>,
        client: Client,
        origin: String,
        prefix: String,
    },

    Prod {
        env: Environment<'static>,
    },
}

impl ViewRenderer {
    pub fn new(config: &AppConfig) -> Self {
        if config.dev_mode {
            Self::Dev {
                env: Environment::new(),
                client: Client::default(),
                origin: config.frontend_url.clone(),
                prefix: config.frontend_prefix.clone(),
            }
        } else {
            let mut env = Environment::new();
            let loader = minijinja::path_loader(&config.templates_dir);

            env.set_loader(loader);

            Self::Prod { env }
        }
    }

    pub async fn render(&self, name: &str, context: impl Serialize) -> String {
        match self {
            ViewRenderer::Dev {
                env,
                client,
                origin,
                prefix,
            } => {
                let name = name.replace(".html", "");
                let url = format!("{}{}/{}", origin, prefix, name);
                let res = client
                    .get(url)
                    .send()
                    .await
                    .expect("failed to fetch template");

                let tmpl = res.text().await.expect("failed to read template");

                let html = env
                    .render_str(&tmpl, &context)
                    .expect("failed to render template");

                html
            }

            ViewRenderer::Prod { env } => {
                let tmpl = env.get_template(name).expect("failed to load template");
                let html = tmpl.render(context).expect("failed to render template");

                html
            }
        }
    }
}
