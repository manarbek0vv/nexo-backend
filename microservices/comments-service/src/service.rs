use tonic::{Request, Response, Status};

use crate::{proto::proto::comments::{AddCommentRequest, AddCommentResponse, DeleteCommentRequest, DeleteCommentResponse, GetCommentRequest, GetCommentResponse, GetCommentsRequest, GetCommentsResponse, UpdateCommentRequest, UpdateCommentResponse, comments_server::Comments}, repository::CommentsRepository};

#[derive(Debug)]
pub struct CommentsService {
    repository: CommentsRepository
}

impl CommentsService {
    pub fn new(repository: CommentsRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl Comments for CommentsService {
    async fn get_comment(
        &self,
        request: Request<GetCommentRequest>
    ) -> Result<Response<GetCommentResponse>, Status> {
        let request = request.into_inner();

        let comment = self.repository.get_comment(&request)
            .await.map_err(|_| Status::internal("DB: Error on getting comment"))?;

        let response = GetCommentResponse {
            comment: Some(comment.into())
        };

        Ok(Response::new(response))
    }
    async fn get_comments(
        &self,
        request: Request<GetCommentsRequest>
    ) -> Result<Response<GetCommentsResponse>, Status> {
        let request = request.into_inner();

        let comment = self.repository.get_comments(&request)
            .await.map_err(|_| Status::internal("DB: Error on getting comment"))?;

        let response = GetCommentsResponse {
            comments: comment.into_iter().map(|c| c.into()).collect()
        };

        Ok(Response::new(response))
    }

    async fn add_comment(
        &self,
        request: Request<AddCommentRequest>
    ) -> Result<Response<AddCommentResponse>, Status> {
        let request = request.into_inner();

        let comment = self.repository.add_comment(&request)
            .await.map_err(|_| Status::internal("DB: Error on adding comment"))?;

        let response = AddCommentResponse {
            comment: Some(comment.into())
        };

        Ok(Response::new(response))
    }

    async fn update_comment(
        &self,
        request: Request<UpdateCommentRequest>
    ) -> Result<Response<UpdateCommentResponse>, Status> {
        let request = request.into_inner();

        let comment = self.repository.update_comment(&request)
            .await.map_err(|_| Status::internal("DB: Error on updating comment"))?;

        let response = UpdateCommentResponse {
            comment: Some(comment.into())
        };

        Ok(Response::new(response))
    }
    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>
    ) -> Result<Response<DeleteCommentResponse>, Status> {
        let request = request.into_inner();

        let comment = self.repository.delete_comment(&request)
            .await.map_err(|_| Status::internal("DB: Error on deleting comment"))?;

        let response = DeleteCommentResponse {
            comment: Some(comment.into())
        };

        Ok(Response::new(response))
    }
}