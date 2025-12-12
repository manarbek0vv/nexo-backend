use actix_web::{HttpResponse, Result, Scope, delete, get, patch, post, web::{self, ReqData}};

use crate::{dto::comments_dto::{AddCommentRequest, AddCommentResponse, DeleteCommentRequest, DeleteCommentResponse, GetCommentRequest, GetCommentResponse, GetCommentsRequest, GetCommentsResponse, UpdateCommentRequest, UpdateCommentResponse}, proto::comments, state::AppState};

pub fn comments_routes() -> Scope {
    web::scope("/comments")
        .service(get_comment)
        .service(get_comments)
        .service(add_comment)
        .service(update_comment)
        .service(delete_comment)
}

#[get("/{id}")]
async fn get_comment(
    state: web::Data<AppState>,
    id: web::Path<GetCommentRequest>,
) -> Result<HttpResponse> {

    let mut client = state.comments_client.clone();

    let request = id.into_inner();

    let request = comments::GetCommentRequest::from(request);

    let response = client
        .get_comment(tonic::Request::new(request))
        .await
        .map_err(|error| {
            match error.code() {
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner().comment.unwrap();

    let http_response = GetCommentResponse {
        comment: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[get("")]
async fn get_comments(
    state: web::Data<AppState>,
    body: web::Json<GetCommentsRequest>,
) -> Result<HttpResponse> {
    println!("GETTING COMMENTS");

    let mut client = state.comments_client.clone();

    let request = body.into_inner();

    let request = comments::GetCommentsRequest::from(request);

    let response = client
        .get_comments(tonic::Request::new(request))
        .await
        .map_err(|error| {
            match error.code() {
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner().comments;

    let http_response = GetCommentsResponse {
        comments: response.into_iter().map(|c| c.try_into()).collect::<Result<_, _>>()
        .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[post("")]
async fn add_comment(
    state: web::Data<AppState>,
    body: web::Json<AddCommentRequest>,
) -> Result<HttpResponse> {
    let mut client = state.comments_client.clone();

    let request = body.into_inner();

    let request = comments::AddCommentRequest::from(request);

    let response = client
        .add_comment(tonic::Request::new(request))
        .await
        .map_err(|error| {
            match error.code() {
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner().comment.unwrap();

    let http_response = AddCommentResponse {
        comment: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[patch("")]
async fn update_comment(
    state: web::Data<AppState>,
    body: web::Json<UpdateCommentRequest>,
) -> Result<HttpResponse> {
    let mut client = state.comments_client.clone();

    let request = body.into_inner();

    let request = comments::UpdateCommentRequest::from(request);

    let response = client
        .update_comment(tonic::Request::new(request))
        .await
        .map_err(|error| {
            match error.code() {
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner().comment.unwrap();

    let http_response = UpdateCommentResponse {
        comment: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}

#[delete("/{id}")]
async fn delete_comment(
    state: web::Data<AppState>,
    id: web::Path<DeleteCommentRequest>,
) -> Result<HttpResponse> {
    let mut client = state.comments_client.clone();

    let request = id.into_inner();

    let request = comments::DeleteCommentRequest::from(request);

    let response = client
        .delete_comment(tonic::Request::new(request))
        .await
        .map_err(|error| {
            match error.code() {
                _ => actix_web::error::ErrorInternalServerError(error.message().to_string()),
            }
        })?
        .into_inner().comment.unwrap();

    let http_response = DeleteCommentResponse {
        comment: response.try_into()
            .map_err(|_| actix_web::error::ErrorInternalServerError("Internal server error"))?
    };

    Ok(HttpResponse::Ok().json(http_response))
}