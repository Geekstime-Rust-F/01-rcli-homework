use std::{fs::read_dir, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;

struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(dir: &str) -> Result<()> {
    let state = Arc::new(HttpServeState {
        path: PathBuf::from(dir),
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let service = ServeDir::new(dir);
    let app = Router::new()
        .nest_service("/tower", service)
        .route("/*path", get(file_handler))
        .with_state(state);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    if p.is_dir() {
        return match read_dir(&p) {
            Ok(paths) => {
                // TODO wrap the directory contents into a list of a link
                let mut filenames = Vec::new();
                for filename in paths {
                    let filename = filename.unwrap();
                    let filename = filename.file_name().into_string().unwrap();
                    filenames.push(filename.to_string());
                }
                (StatusCode::OK, format!("{:?}", filenames))
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to read directory: {:?}", e),
                );
            }
        };
    }
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("file not found: {:?}", p.display()),
        )
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to read file: {:?}", e),
            ),
        }
    }
}
