use crate::state::FoundationState;
use axum::body::StreamBody;
use axum::extract::{Path, State};
use axum::http::header::{CONTENT_LENGTH, LAST_MODIFIED};
use axum::http::{HeaderValue, StatusCode};
use axum::response::Response;
use std::time::{SystemTime, UNIX_EPOCH};
use time::format_description::well_known::Rfc2822;
use time::{OffsetDateTime, UtcOffset};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

type FileStreamResponse = Response<StreamBody<ReaderStream<File>>>;

fn get_gmt_string_from_system_time(system_time: &SystemTime) -> anyhow::Result<String> {
  Ok(
    OffsetDateTime::from_unix_timestamp(system_time.duration_since(UNIX_EPOCH)?.as_secs() as i64)?
      .to_offset(UtcOffset::UTC)
      .format(&Rfc2822)?,
  )
}

async fn prepare_download_response(
  file: File,
) -> anyhow::Result<FileStreamResponse> {
  let metadata = file.metadata().await?;

  let mut response = Response::new(StreamBody::new(ReaderStream::new(file)));
  let headers = response.headers_mut();

  let last_modified_stamp = get_gmt_string_from_system_time(&metadata.modified()?)?;

  headers.insert(CONTENT_LENGTH, HeaderValue::from(metadata.len()));
  headers.insert(
    LAST_MODIFIED,
    HeaderValue::from_str(last_modified_stamp.as_str())?,
  );

  Ok(response)
}

pub(crate) async fn download_static_content(
  State(state): State<FoundationState>,
  Path(file_path): Path<String>,
) -> Result<FileStreamResponse, StatusCode> {
  let file = state
    .static_content
    .get_file(file_path)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

  Ok(
    prepare_download_response(file)
      .await
      .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
  )
}
