use sqlx::PgPool;
use uuid::Uuid;

use crate::models::playlist::{Playlist, PlaylistItem};
use crate::repositories::playlist as playlist_repo;
use crate::validation::validate_playlist_name;
use super::is_duplicate_key;

// ── Config ────────────────────────────────────────────────────────────────────

const INDEX_GAP_THRESHOLD: f64 = 1e-9;

// ── Types ─────────────────────────────────────────────────────────────────────

pub enum PlaylistError {
    NotFound,
    Forbidden,
    AlreadyAdded,
    ValidationError(String),
    Internal,
}

// ── Private ───────────────────────────────────────────────────────────────────

async fn resolve_playlist(pool: &PgPool, playlist_id: &str, user_id: &str) -> Result<Playlist, PlaylistError> {
    let playlist = playlist_repo::find_by_id(pool, playlist_id)
        .await
        .map_err(|_| PlaylistError::Internal)?
        .ok_or(PlaylistError::NotFound)?;

    if playlist.user_id != user_id {
        return Err(PlaylistError::Forbidden);
    }

    Ok(playlist)
}

async fn resolve_library_item(pool: &PgPool, user_library_item_id: &str, user_id: &str) -> Result<(), PlaylistError> {
    let owner = playlist_repo::find_library_item_user(pool, user_library_item_id)
        .await
        .map_err(|_| PlaylistError::Internal)?
        .ok_or(PlaylistError::NotFound)?;

    if owner != user_id {
        return Err(PlaylistError::Forbidden);
    }

    Ok(())
}

fn target_index(rest: &[f64], index: usize) -> f64 {
    match (rest.is_empty(), index) {
        (true, _) => 0.0,
        (_, 0) => {
            let new_index = rest[0] / 2.0;
            if new_index < INDEX_GAP_THRESHOLD {
                -1.0 // Signal normalization needed
            } else {
                new_index
            }
        }
        (_, i) if i >= rest.len() => rest.last().unwrap() + 1.0,
        (_, i) => (rest[i - 1] + rest[i]) / 2.0,
    }
}

fn gap_too_small(rest: &[f64], index: usize, new_index: f64) -> bool {
    let left  = index.checked_sub(1).and_then(|i| rest.get(i)).copied();
    let right = rest.get(index).copied();

    left.is_some_and(|l|  new_index - l  < INDEX_GAP_THRESHOLD)
        || right.is_some_and(|r| r - new_index < INDEX_GAP_THRESHOLD)
}

async fn normalize(pool: &PgPool, rest: &[&PlaylistItem], index: usize) -> Result<f64, PlaylistError> {
    let updates: Vec<(String, f64)> = rest
        .iter()
        .enumerate()
        .map(|(i, item)| (item.id.clone(), i as f64))
        .collect();

    playlist_repo::set_item_indices(pool, updates)
        .await
        .map_err(|_| PlaylistError::Internal)?;

    let normalized: Vec<f64> = (0..rest.len())
        .map(|i| i as f64)
        .collect();

    Ok(target_index(&normalized, index))
}

// ── Public ────────────────────────────────────────────────────────────────────

pub async fn list(pool: &PgPool, user_id: &str) -> Result<Vec<Playlist>, PlaylistError> {
    playlist_repo::list(pool, user_id)
        .await
        .map_err(|_| PlaylistError::Internal)
}

pub async fn create(pool: &PgPool, user_id: &str, name: &str) -> Result<Playlist, PlaylistError> {
    validate_playlist_name(name).map_err(PlaylistError::ValidationError)?;

    let id = Uuid::new_v4().to_string();

    playlist_repo::create(pool, &id, user_id, name)
        .await
        .map_err(|_| PlaylistError::Internal)
}

pub async fn delete(pool: &PgPool, user_id: &str, playlist_id: &str) -> Result<(), PlaylistError> {
    resolve_playlist(pool, playlist_id, user_id).await?;

    playlist_repo::delete(pool, playlist_id)
        .await
        .map_err(|_| PlaylistError::Internal)
        .map(|_| ())
}

pub async fn rename(pool: &PgPool, user_id: &str, playlist_id: &str, name: &str) -> Result<Playlist, PlaylistError> {
    validate_playlist_name(name).map_err(PlaylistError::ValidationError)?;
    resolve_playlist(pool, playlist_id, user_id).await?;

    playlist_repo::rename(pool, playlist_id, name)
        .await
        .map_err(|_| PlaylistError::Internal)?
        .ok_or(PlaylistError::NotFound)
}

pub async fn list_items(pool: &PgPool, user_id: &str, playlist_id: &str) -> Result<Vec<PlaylistItem>, PlaylistError> {
    resolve_playlist(pool, playlist_id, user_id).await?;

    playlist_repo::list_items(pool, playlist_id)
        .await
        .map_err(|_| PlaylistError::Internal)
}

pub async fn add_item(
    pool: &PgPool,
    user_id: &str,
    playlist_id: &str,
    user_library_item_id: &str,
) -> Result<PlaylistItem, PlaylistError> {
    resolve_playlist(pool, playlist_id, user_id).await?;
    resolve_library_item(pool, user_library_item_id, user_id).await?;

    let id = Uuid::new_v4().to_string();
    playlist_repo::add_item(pool, &id, playlist_id, user_library_item_id)
        .await
        .map_err(|e| if is_duplicate_key(&e) { PlaylistError::AlreadyAdded } else { PlaylistError::Internal })
}

pub async fn remove_item(
    pool: &PgPool,
    user_id: &str,
    playlist_id: &str,
    user_library_item_id: &str,
) -> Result<(), PlaylistError> {
    resolve_playlist(pool, playlist_id, user_id).await?;

    let deleted = playlist_repo::remove_item(pool, playlist_id, user_library_item_id)
        .await
        .map_err(|_| PlaylistError::Internal)?;
    if !deleted {
        return Err(PlaylistError::NotFound);
    }

    Ok(())
}

pub async fn move_item(
    pool: &PgPool,
    user_id: &str,
    playlist_id: &str,
    item_id: &str,
    index: usize,
) -> Result<PlaylistItem, PlaylistError> {
    resolve_playlist(pool, playlist_id, user_id).await?;

    let items = playlist_repo::list_items(pool, playlist_id)
        .await
        .map_err(|_| PlaylistError::Internal)?;
    if !items.iter().any(|i| i.id == item_id) {
        return Err(PlaylistError::NotFound);
    }

    let rest: Vec<&PlaylistItem> = items.iter().filter(|i| i.id != item_id).collect();
    let rest_indices: Vec<f64>   = rest.iter().map(|i| i.index).collect();

    let new_index = target_index(&rest_indices, index);
    let new_index = if new_index < 0.0 || gap_too_small(&rest_indices, index, new_index) {
        normalize(pool, &rest, index).await?
    } else {
        new_index
    };

    playlist_repo::update_item_index(pool, item_id, new_index)
        .await
        .map_err(|_| PlaylistError::Internal)?
        .ok_or(PlaylistError::NotFound)
}
