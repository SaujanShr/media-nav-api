use reqwest::blocking::Client;

// ── Config ────────────────────────────────────────────────────────────────────

pub const JIKAN_BASE_URL:         &str = "https://api.jikan.moe/v4";
pub const CONSUMET_BASE_URL: &str = "http://localhost:4000";

// ── Public ────────────────────────────────────────────────────────────────────

pub fn client() -> Client {
    Client::new()
}
