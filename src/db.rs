use openssl::ssl::{SslConnector, SslMethod};
use postgres::Client;
use postgres_openssl::MakeTlsConnector;

const CONFIG_ENV: &str = "MESSAGE_STORE_URL";
const MISSING: &str = "config missing";

pub fn build() -> Client {
    connect(&read_configuration())
}

pub fn connect(config: &str) -> Client {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let client = Client::connect(&config, connector)
        .unwrap_or_else(|err| panic!("could not connect to database: {}", err));

    return client;
}

pub fn read_configuration() -> String {
    std::env::var(CONFIG_ENV).unwrap_or(String::from(MISSING))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE_STORE_URL: &str = "postgres://message_store:message_store@localhost:5432";

    #[test]
    fn builds() {
        let existing = std::env::var(CONFIG_ENV);
        std::env::set_var(CONFIG_ENV, MESSAGE_STORE_URL);

        let client = build();
        assert!(!client.is_closed());

        match existing {
            Ok(c) => std::env::set_var(CONFIG_ENV, c),
            _ => (),
        }
    }

    #[test]
    fn reads_configuration_from_env() {
        let config = "config";
        let existing = std::env::var(CONFIG_ENV);
        std::env::set_var(CONFIG_ENV, config);

        let read = read_configuration();

        assert_eq!(config, read);

        match existing {
            Ok(c) => std::env::set_var(CONFIG_ENV, c),
            _ => (),
        }
    }

    #[test]
    fn connects() {
        let client = connect(MESSAGE_STORE_URL);
        assert!(!client.is_closed());
    }

    #[test]
    #[should_panic(expected = "error connecting to server")]
    fn doesnt_connect_when_config_is_incorrect() {
        let client = connect("postgres://message_store:message_store@very-wrong:5432");
        assert!(!client.is_closed());
    }

    #[test]
    #[should_panic(expected = "invalid connection string")]
    fn panics_when_config_is_incorrect() {
        connect("nope");
        ();
    }
}
