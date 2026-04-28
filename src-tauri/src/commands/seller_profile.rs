use system_service::{SellerProfileDTO, SellerProfileService, UpdateSellerProfileDTO};
use tauri::State;

use crate::AppState;

use super::{SResult, Success};

#[tauri::command]
#[specta::specta]
pub async fn get_seller_profile(state: State<'_, AppState>) -> SResult<SellerProfileDTO> {
    let db_conn = state.system_db();
    match SellerProfileService::get_or_create_profile(db_conn).await {
        Ok(profile) => Ok(Success {
            error: None,
            message: None,
            data: Some(profile),
        }),
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn update_seller_profile(
    profile: UpdateSellerProfileDTO,
    state: State<'_, AppState>,
) -> SResult<SellerProfileDTO> {
    let db_conn = state.system_db();
    match SellerProfileService::update_profile(db_conn, profile).await {
        Ok(updated) => Ok(Success {
            error: None,
            message: None,
            data: Some(updated),
        }),
        Err(err) => Err(err.into()),
    }
}
