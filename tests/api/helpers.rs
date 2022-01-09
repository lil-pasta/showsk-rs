use form_urlencoded::byte_serialize;
use once_cell::sync::Lazy;
use reqwest::Client;
use showsk_rs::configuration::{get_conf, DatabaseSetting};
use showsk_rs::startup::Application;
use showsk_rs::telemetry::{init_subscriber, subscriber_set_up};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub up_path: String,
    pub db_pool: PgPool,
}

pub enum UserField {
    Username,
    Password,
    PasswordVer,
    Email,
}

pub struct TestUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub password_ver: &'a str,
    pub email: &'a str,
    pub note: &'a str,
}

impl<'a> TestUser<'a> {
    pub fn valid_user() -> Self {
        self::TestUser {
            username: "goodboy420",
            password: "password89!",
            password_ver: "password89!",
            email: "gooboy@hotmail.com",
            note: "valid submission",
        }
    }

    pub fn missing_field(missing: UserField) -> Self {
        let (username, password, password_ver, email, note) = match missing {
            UserField::Username => (
                "",
                "password89!",
                "password89!",
                "goodboy420@hotmail.com",
                "missing username",
            ),
            UserField::Password => (
                "goodboy420",
                "",
                "password89!",
                "goodboy420@hotmail.com",
                "missing password",
            ),
            UserField::PasswordVer => (
                "goodboy420",
                "password89!",
                "",
                "goodboy420@hotmail.com",
                "missing password verification",
            ),
            UserField::Email => (
                "goodboy420",
                "password89!",
                "password89!",
                "",
                "missing email",
            ),
        };

        self::TestUser {
            username,
            password,
            password_ver,
            email,
            note,
        }
    }

    pub fn invalid_field(invalid: UserField) -> Self {
        let (username, password, password_ver, email, note) = match invalid {
            UserField::Username => (
                "fn malware(evil)->unlimitedpower{}",
                "password89!",
                "password89!",
                "goodboy420@hotmail.com",
                "invalid username",
            ),
            UserField::Password => (
                "goodboy420",
                "pass",
                "pass",
                "goodboy420@hotmail.com",
                "invalid password",
            ),
            UserField::PasswordVer => (
                "goodboy420",
                "password89!",
                "pass",
                "goodboy420@hotmail.com",
                "missmatch password verification",
            ),
            UserField::Email => (
                "goodboy420",
                "password89!",
                "password89!",
                "thisisnotapassword.com",
                "invalid email",
            ),
        };

        self::TestUser {
            username,
            password,
            password_ver,
            email,
            note,
        }
    }

    pub fn new_user_body(&self) -> String {
        let username: String = byte_serialize(self.username.as_bytes()).collect();
        let email: String = byte_serialize(self.email.as_bytes()).collect();
        let password: String = byte_serialize(self.password.as_bytes()).collect();
        let password_ver: String = byte_serialize(self.password_ver.as_bytes()).collect();
        format!(
            "username={}&email={}&password={}&password_ver={}",
            username, email, password, password_ver
        )
    }

    pub fn user_login_body(&self) -> String {
        let username: String = byte_serialize(self.username.as_bytes()).collect();
        let password: String = byte_serialize(self.password.as_bytes()).collect();
        format!("username={}&password={}", username, password)
    }

    pub async fn add_user(&self, app_address: &String) -> Result<reqwest::Response, anyhow::Error> {
        let body = self.new_user_body();
        let client = Client::new();
        let resp = client
            .post(&format!("{}/add_user", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to reach route add_user");
        Ok(resp)
    }

    pub async fn login_user(
        &self,
        app_address: &String,
    ) -> Result<reqwest::Response, anyhow::Error> {
        let body = self.user_login_body();
        let client = Client::new();
        let resp = client
            .post(&format!("{}/login", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("failed to reach route add_user");
        Ok(resp)
    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = subscriber_set_up("waterboy_test".into(), "info".into());
    init_subscriber(subscriber);
});

pub async fn spawn_app() -> TestApp {
    // tracing for test app
    Lazy::force(&TRACING);

    // spawn your test db with a random db name
    let conf = {
        let mut c = get_conf().expect("failed to bind path");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    let db_pool = spawn_test_db(&conf.database).await;
    let upp = conf.application.upload_path.clone();
    let application = Application::build(conf)
        .await
        .expect("failed to start test server");
    let port = application.get_port();
    let addr = format!("http://127.0.0.1:{}", &port);
    // start a server bound to that random port
    let _ = tokio::spawn(application.run_until_stop());

    TestApp {
        address: addr,
        up_path: upp,
        db_pool,
    }
}

pub async fn spawn_test_db(database: &DatabaseSetting) -> PgPool {
    let mut db_connection = PgConnection::connect_with(&database.connection_without_db())
        .await
        .expect("could not connect to postgres");

    db_connection
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            database.database_name
        ))
        .await
        .expect("failed to create test db");

    let pool = PgPool::connect_with(database.connection_with_db())
        .await
        .expect("could not connect to test db");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");
    pool
}
