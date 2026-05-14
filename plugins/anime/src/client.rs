use reqwest::blocking::Client;

// ── Config ────────────────────────────────────────────────────────────────────

pub const BASE_URL:         &str = "https://api.jikan.moe/v4";

// ── Public ────────────────────────────────────────────────────────────────────

pub fn client() -> Client {
    Client::new()
}
