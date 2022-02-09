use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String{
        format!("mysql://{}:{}@{}:{}/{}",self.username,self.password,self.host,self.port,self.database_name)
    }

    pub fn connection_string_without_db(&self) -> String { 
        format!("mysql://{}:{}@{}:{}",self.username,self.password,self.host,self.port)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    //looks for configuration in top level with an extension like yaml or json
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}

