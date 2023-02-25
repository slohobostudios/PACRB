use sfml::graphics::Texture;
use std::process::exit;
use tracing::error;

pub mod aseperite_parse;
pub mod resource_manager;

// Loading screen stuff. One time use. It's ok to be verbose
pub fn load_sfml_logo() -> sfml::SfBox<Texture> {
    let file_name = &format!("{}{}", resource_manager::ASSETS_PATH, "sfml-logo-big.png")[..];
    match Texture::from_file(file_name) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to load sfml logo! file name: {}", file_name);
            error!("{}", e);
            exit(10);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_load_sfml_logo() {
        let _texture = load_sfml_logo();
        assert!(true);
    }
}
