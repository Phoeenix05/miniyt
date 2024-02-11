use axum::{extract::Path, Json};

use crate::data::Channel;

pub async fn by_cuid(Path(cuid): Path<String>) -> Result<Json<Channel>, ()> {
    Err(())
}
