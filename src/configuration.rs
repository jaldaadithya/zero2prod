use secrecy::{Secret, ExposeSecret};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings{
    pub port : u16,
    pub host: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String>{
        Secret::new(format!("mysql://{}:{}@{}:{}/{}",self.username,self.password.expose_secret(),self.host,self.port,self.database_name))
    }

    pub fn connection_string_without_db(&self) -> Secret<String> { 
        Secret::new(format!("mysql://{}:{}@{}:{}",self.username,self.password.expose_secret(),self.host,self.port))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    let base_path = std::env::current_dir().expect("Faield to determine the current directory");
    let configuration_directory = base_path.join("configuration");
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment:Environment = std::env::var("APP_ENVIRONMENT")
    .unwrap_or_else(|_| "local".into())
    .try_into()
    .expect("Failed to parse APP_ENVIRONMENT.");


    //looks for configuration in top level with an extension like yaml or json
    settings.merge(config::File::from(configuration_directory.join(environment.as_str())).required(true))?;
    settings.try_into()
}

pub enum Environment {
    Local,
    Production
}

impl Environment {
     pub fn as_str(&self) ->&'static str {
         match self {
             Environment::Local => "local",
             Environment::Production => "production"
         }
     }  
}

impl TryFrom<String> for Environment {
    
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "production" => Ok(Environment::Production),
            other => Err(format!("{} is not supported env variable , use either local or production. for APP_ENVIRONMENT",other))
        }
    }
}