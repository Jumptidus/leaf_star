mod common;

// app(socks) -> (socks)client(direct) -> echo
#[cfg(all(
    feature = "inbound-http",
    feature = "inbound-socks",
    feature = "outbound-direct",
))]
#[test]
fn test_http() {
    use leaf::{start, Config, RuntimeOption, StartOptions};

    let config1 = r#"
    {
        "inbounds": [
            {
                "protocol": "http",
                "address": "127.0.0.1",
                "port": 1086
            }
        ],
        "outbounds": [
            {
                "protocol": "direct"
            }
        ]
    }
    "#;

    let opts = StartOptions {
        config: Config::Str(config1.to_string()),
        #[cfg(feature = "auto-reload")]
        auto_reload: false,
        runtime_opt: RuntimeOption::SingleThread,
    };
    start(0, opts);

    //let configs = vec![config1.to_string()];
    // common::test_configs(configs, "127.0.0.1", 1086);
}
