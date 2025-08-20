pub enum Language {
    En,
    Ru,
}

pub fn t(lang: Language, key: &str) -> String {
    match (lang, key) {
        (Language::En, "hello") => "Hello!".to_string(),
        (Language::Ru, "hello") => "Привет!".to_string(),
        _ => key.to_string(),
    }
}
