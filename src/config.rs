use envconfig::Envconfig;
use log;
use std::str::FromStr;

#[derive(Debug)]
pub enum Environment {
    PROD,
    DEV,
}

impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PROD" => Ok(Self::PROD),
            "DEV" => Ok(Self::DEV),
            _ => {
                log::warn!("Environment from ENV different from [PROD|DEV]. Env value: {}. Defaulting to DEV.", s);
                Ok(Self::DEV)
            }
        }
    }
}

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "ENVIRONMENT", default = "DEV")]
    pub env: Environment,
}
