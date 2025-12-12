use tonic::transport::Channel;
use crate::config::Config;


// ------------- Including proto -------------
use crate::proto::auth::auth_client::AuthClient;
use crate::proto::comments::comments_client::{self, CommentsClient};
use crate::proto::posts::posts_client::PostsClient;
use crate::proto::users::users_client::UsersClient;


#[derive(Clone)]
pub struct AppState {
    pub auth_client: AuthClient<Channel>,
    pub users_client: UsersClient<Channel>,
    pub posts_client: PostsClient<Channel>,
    pub comments_client: CommentsClient<Channel>,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let auth_channel = Channel::from_shared(config.auth_service_url.clone())?
                .connect().await?;
        let users_channel = Channel::from_shared(config.users_service_url.clone())?
            .connect().await?;
        let posts_channel = Channel::from_shared(config.posts_service_url.clone())?
            .connect().await?;
        let comments_channel = Channel::from_shared(config.comments_service_url.clone())?
            .connect().await?;

        let state = Self {
            auth_client: AuthClient::new(auth_channel),
            users_client: UsersClient::new(users_channel),
            posts_client: PostsClient::new(posts_channel),
            comments_client: CommentsClient::new(comments_channel),
        };

        Ok(state)
    }
}