pub mod file {
    use std::{path::PathBuf, hash::Hasher, collections::hash_map::DefaultHasher};
    
    use axum::{response::IntoResponse, http::{StatusCode, HeaderMap, header, HeaderValue}, body::StreamBody};
    use tokio_util::io::ReaderStream;
    use std::hash::Hash;

    use crate::extensions::PathBufDetemineMimeExt;

    pub async fn open(path: PathBuf) -> Result<(HeaderMap, impl IntoResponse), StatusCode> {
        let file = tokio::fs::File::open(&path).await.map_err(|_| StatusCode::NOT_FOUND)?;

        let stream = ReaderStream::new(file);
        let body = StreamBody::new(stream);
        
        let mut headers = HeaderMap::new();

        if let Ok(metadata) = tokio::fs::metadata(&path).await {
            if let Ok(time) = metadata.modified() {
                let mut hasher = DefaultHasher::new();
                time.hash(&mut hasher);

                headers.append(header::ETAG, HeaderValue::from(hasher.finish()));
            };
        };

        let mime_type = path.get_mime_type();

        headers.append(header::CONTENT_TYPE, HeaderValue::from_str(mime_type.essence_str()).unwrap());

        Ok((headers, body))
    }
}