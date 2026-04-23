use std::sync::Arc;
use tauri::State;

use crate::{to_command_result, CommandResult};

use super::traits::ItemImageService;

/// Fetch (and cache) the POE2 version of an item image given the
/// `web.poecdn.com/image/...` URL that lives on item records.
///
/// Returns a `data:image/webp;base64,...` URL the frontend can use as
/// `<img src>` directly. If the input URL isn't recognized, it's passed
/// through unchanged so callers get a sensible fallback.
#[tauri::command]
pub async fn get_item_image(
    service: State<'_, Arc<dyn ItemImageService>>,
    url: String,
) -> CommandResult<String> {
    to_command_result(service.get_image_data_url(&url).await)
}
