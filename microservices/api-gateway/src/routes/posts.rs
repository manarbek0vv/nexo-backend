use actix_web::{HttpResponse, Result, Scope, delete, get, post, put, web};
use tonic::Code;

use crate::dto::posts_dto::{CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest, GetPostResponse, GetPostsResponse, UpdatePostRequest, UpdatePostResponse};
use crate::proto::posts;
use crate::{state::AppState};

pub fn posts_routes() -> Scope {
    web::scope("/posts")
        .service(get_post)
        .service(get_posts)
        .service(create_post)
        .service(update_post)
        .service(delete_post)
}

#[get("/{id}")]
async fn get_post(
    state: web::Data<AppState>,
    id: web::Path<GetPostRequest>,
) -> Result<HttpResponse> {
    let mut client = state.posts_client.clone();

    let request = id.into_inner();

    let response = client
        .get_post(tonic::Request::new(request.into()))
        .await
        .map_err(|error| {
            match error.code() {
                Code::NotFound => actix_web::error::ErrorNotFound(error.message().to_string()),
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner().post.unwrap();

    let http_response = GetPostResponse {
        post: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[get("")]
async fn get_posts(
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let mut client = state.posts_client.clone();

    let request = posts::GetPostsRequest { };

    let response = client
        .get_posts(tonic::Request::new(request))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message().to_string()))?
        .into_inner().posts;

    let http_response = GetPostsResponse {
        posts: response.into_iter().map(|c| c.try_into()).collect::<Result<_, _>>()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[post("")]
async fn create_post(
    state: web::Data<AppState>,
    body: web::Json<CreatePostRequest>
) -> Result<HttpResponse> {
    let mut client = state.posts_client.clone();

    let request = body.into_inner();

    let response = client
        .create_post(tonic::Request::new(request.into()))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message().to_string()))?
        .into_inner().post.unwrap();

    let http_response = CreatePostResponse {
        post: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[put("")]
async fn update_post(
    state: web::Data<AppState>,
    body: web::Json<UpdatePostRequest>
) -> Result<HttpResponse> {
    let mut client = state.posts_client.clone();

    let request = body.into_inner();

    let response = client
        .update_post(tonic::Request::new(request.into()))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message().to_string()))?
        .into_inner().post.unwrap();

    let http_response = UpdatePostResponse {
        post: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[delete("/{id}")]
async fn delete_post(
    state: web::Data<AppState>,
    id: web::Path<DeletePostRequest>
) -> Result<HttpResponse> {
    let mut client = state.posts_client.clone();

    let request = id.into_inner();

    let response = client
        .delete_post(tonic::Request::new(request.into()))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.message().to_string()))?
        .into_inner().post.unwrap();

    let http_response = DeletePostResponse {
        post: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
        };

    Ok(HttpResponse::Ok().json(http_response))
}