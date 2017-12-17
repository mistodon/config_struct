use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub name: Cow<'static, str>
}

pub const CONFIG: Config = Config {
    name: Cow::Borrowed("Config name")
};
