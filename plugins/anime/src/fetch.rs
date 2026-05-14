use reqwest::blocking::Client;
use plugin_sdk::library_item::LibraryItem;
use plugin_sdk::plugin::{FetchRequest, FetchResult};

use crate::client::{client, JIKAN_BASE_URL};
use crate::schema::parse;
use crate::types::JikanSearchResponse;

// ── Private ───────────────────────────────────────────────────────────────────

fn build_url(req: &FetchRequest) -> String {
    let params = parse(&req.query);

    let mut url = format!(
        "{JIKAN_BASE_URL}/anime?limit={}&page={}&order_by={}&sort={}",
        req.page_size, req.page, params.order_by, params.sort_dir,
    );

    if let Some(q) =    &params.search { url.push_str(&format!("&q={q}")); }
    if let Some(t) =    &params.anime_type { url.push_str(&format!("&type={t}")); }
    if let Some(s) =    &params.status { url.push_str(&format!("&status={s}")); }
    if let Some(r) =    &params.rating { url.push_str(&format!("&rating={r}")); }
    if let Some(mn) =       params.score_min { url.push_str(&format!("&min_score={mn}")); }
    if let Some(mx) =       params.score_max { url.push_str(&format!("&max_score={mx}")); }
    if let Some(sd) = params.start_year { url.push_str(&format!("&start_date={sd}")); }
    if params.sfw {
        url.push_str("&sfw");
    }

    url
}

// ── Private ───────────────────────────────────────────────────────────────────

fn fetch_anime(client: &Client, req: &FetchRequest) -> Option<JikanSearchResponse> {
    let url = build_url(req);

    client
        .get(&url)
        .send()
        .and_then(|r| r.json::<JikanSearchResponse>())
        .ok()
}

fn page_to_result(page: JikanSearchResponse) -> FetchResult {
    let items = page.data.into_iter().map(|a| LibraryItem {
        id: a.mal_id.to_string(),
        title: a.title,
        thumbnail_url: a.images.jpg.image_url,
    }).collect::<Vec<_>>();

    let total = page.pagination.items.total;

    FetchResult { items, total }
}

// ── Public ────────────────────────────────────────────────────────────────────

pub fn fetch(req: FetchRequest) -> FetchResult {
    let client = client();

    match fetch_anime(&client, &req) {
        Some(page) => page_to_result(page),
        None => FetchResult { items: vec![], total: 0 },
    }
}
