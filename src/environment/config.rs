use crate::error::AppServerError;
use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub provider_url: String,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub port: String,
}

#[derive(Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub environment: EnvConfig,
}

static CONFIG: OnceCell<AppConfig> = OnceCell::new();

pub fn get_app_config() -> Result<&'static AppConfig, AppServerError> {
    CONFIG.get_or_try_init(|| {
        // Get the env variable
        let env = env::var("ENV")
            .map_err(|_| AppServerError::ConfigError("ENV is not set".to_string()))?;

        // Get the port variable
        let port = env::var("PORT")
            .map_err(|_| AppServerError::ConfigError("PORT is not set".to_string()))?;

        // Allowed environments
        let allowed_envs: HashSet<&str> = ["dev", "prd", "stg"].iter().cloned().collect();

        if !allowed_envs.contains(env.as_str()) {
            return Err(AppServerError::ConfigError(format!(
                "Invalid environment: {}",
                env
            )));
        }

        // Load the TOML configuration file corresponding to env
        let env_setting = Config::builder()
            .add_source(File::with_name(&format!("./src/environment/{}", env)))
            .build()
            .map_err(|_| {
                AppServerError::ConfigError(format!(
                    "Invalid file path: ./src/environment/{}.toml",
                    env
                ))
            })?;

        // Deserialize into EnvConfig
        let env_config: EnvConfig = env_setting
            .try_deserialize()
            .map_err(|_| AppServerError::ConfigError("Invalid EnvConfig structure".to_string()))?;

        // Build the app config
        let app_config = AppConfig {
            server: ServerConfig { port },
            environment: env_config,
        };

        Ok(app_config)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_get_app_config_valid_dev_env() {
        // Set up environment variables
        env::set_var("ENV", "dev");
        env::set_var("PORT", "8080");

        // Create a test configuration file for the 'dev' environment
        let config_content = r#"
            provider_url = "http://localhost:8080"
        "#;

        // Ensure the directory exists
        let config_dir = "./src/environment";
        fs::create_dir_all(config_dir).unwrap();

        // Write the configuration file
        let config_path = format!("{}/dev.toml", config_dir);
        fs::write(&config_path, config_content).unwrap();

        // Call the function
        let result = get_app_config();

        // Clean up
        fs::remove_file(&config_path).unwrap();
        env::remove_var("ENV");
        env::remove_var("PORT");

        // Assert the result
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.server.port, "8080");
        assert_eq!(
            config.environment.provider_url,
            "http://localhost:8080"
        );
    }
}
