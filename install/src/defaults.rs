pub const JSON_RPC_URL: &str = "http://api.devnet.exzo.network";

/// Returns last 30 releases from GitHub Release REST API, could be
/// increased up to 100 (?per_page=100). If we need a list with more
/// than 100 releases, we would need to implement pagination (?page_id=2)
pub const GITHUB_RELEASES_URL: &str = "https://api.github.com/repos/ExzoNetwork/Exzo-Network-Blockchain/releases";

lazy_static! {
    pub static ref CONFIG_PATH: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".config", "exzo", "install", "config.yml"]);
            path.to_str().unwrap().to_string()
        })
    };
    pub static ref USER_KEYPAIR: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".config", "exzo", "id.json"]);
            path.to_str().unwrap().to_string()
        })
    };
    pub static ref DATA_DIR: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".local", "share", "exzo", "install"]);
            path.to_str().unwrap().to_string()
        })
    };
}
