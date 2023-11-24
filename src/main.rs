use serde::Deserialize;
use config::Config;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", content = "config")]
enum ServerConfig {
    Remote(RemoteServerConfig),
    Local,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
struct RemoteServerConfig {
    port: u16,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
struct Root {
    server: ServerConfig,
}

fn main() {
    std::env::set_var("DEMO__SERVER__TYPE", "Remote");
    std::env::set_var("DEMO__SERVER__CONFIG__PORT", "1234");

    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("DEMO")
                .prefix_separator("__")
                .separator("__"),
        )
        .build()
        .expect("Failed to build config");
    let root: Root = config.try_deserialize().expect("Failed to parse config");
    println!("{:?}", root);

    // Sometimes works, sometimes fails with
    // Failed to parse config: invalid type: string "1234", expected u16
}
