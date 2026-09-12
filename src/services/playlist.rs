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

fn target_index(total_other: usize, index: usize, neighbors: &[f64]) -> f64 {
    if total_other == 0 {
        return 0.0;
    }
    if index == 0 {
        let new_index = neighbors[0] / 2.0;
        if new_index < INDEX_GAP_THRESHOLD {
            -1.0 // Signal normalization needed
        } else {
            new_index
        }
    } else if index >= total_other {
        neighbors[neighbors.len() - 1] + 1.0
    } else {
        (neighbors[0] + neighbors[1]) / 2.0
    }
}

fn gap_too_small(total_other: usize, index: usize, neighbors: &[f64], new_index: f64) -> bool {
    if index == 0 {
        neighbors.first().is_some_and(|r| r - new_index < INDEX_GAP_THRESHOLD)
    } else if index >= total_other {
        false
    } else {
        (new_index - neighbors[0] < INDEX_GAP_THRESHOLD) || (neighbors[1] - new_index < INDEX_GAP_THRESHOLD)
    }
}


async fn normalize(pool: &PgPool, playlist_id: &str, exclude_item_id: &str, index: usize) -> Result<f64, PlaylistError> {
    let rest: Vec<PlaylistItem> = playlist_repo::list_items(pool, playlist_id)
        .await
        .map_err(|_| PlaylistError::Internal)?
        .into_iter()
        .filter(|item| item.id != exclude_item_id)
        .collect();

    let updates: Vec<(String, f64)> = rest
        .iter()
        .enumerate()
        .map(|(i, item)| (item.id.clone(), i as f64))
        .collect();

    playlist_repo::set_item_indices(pool, updates)
        .await
        .map_err(|_| PlaylistError::Internal)?;

    let total = rest.len();
    let neighbors: Vec<f64> = match index {
        0 if total > 0 => vec![0.0],
        i if total > 0 && i >= total => vec![(total - 1) as f64],
        i if total > 0 => vec![(i - 1) as f64, i as f64],
        _ => Vec::new(),
    };

    Ok(target_index(total, index, &neighbors))
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

    let exists = playlist_repo::item_exists(pool, playlist_id, item_id)
        .await
        .map_err(|_| PlaylistError::Internal)?;
    if !exists {
        return Err(PlaylistError::NotFound);
    }

    let total_other = playlist_repo::count_items(pool, playlist_id)
        .await
        .map_err(|_| PlaylistError::Internal)? as usize - 1;

    let neighbors = if total_other == 0 {
        Vec::new()
    } else {
        let offset = if index >= total_other {
            total_other - 1
        } else {
            index.saturating_sub(1)
        } as i64;

        playlist_repo::neighbor_indices(pool, playlist_id, item_id, offset)
            .await
            .map_err(|_| PlaylistError::Internal)?
    };

    let new_index = target_index(total_other, index, &neighbors);
    let new_index = if new_index < 0.0 || gap_too_small(total_other, index, &neighbors, new_index) {
        normalize(pool, playlist_id, item_id, index).await?
    } else {
        new_index
    };

    playlist_repo::update_item_index(pool, item_id, new_index)
        .await
        .map_err(|_| PlaylistError::Internal)?
        .ok_or(PlaylistError::NotFound)
}
