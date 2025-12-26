use crate::{args::Args, stream_proxy_cache::StreamProxyCache};
use actix_web::{web::Data, HttpRequest, HttpResponse};

pub async fn catch_all(req: HttpRequest, args: Data<Args>) -> HttpResponse {
    let path = req.uri().path_and_query().map_or("/", |pq| pq.as_str());
    let target_url = format!(
        "{}://{}{}",
        req.connection_info().scheme(),
        args.domain,
        path
    );

    let Ok(parsed_url) = reqwest::Url::parse(&target_url) else {
        return HttpResponse::BadRequest().body("Invalid URL");
    };

    let url_path = StreamProxyCache::normalize_path(&parsed_url);
    let cache = StreamProxyCache::new(args.download_dir.clone());

    if let Some(data) = cache.read_from_cache(&url_path).await {
        return HttpResponse::Ok().body(data);
    }

    match cache.stream_and_cache(&target_url, &url_path).await {
        Ok(response) => response,
        Err(err) => HttpResponse::BadGateway().body(err),
    }
}
