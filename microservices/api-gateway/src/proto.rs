pub mod auth {
    tonic::include_proto!("auth");
}
pub mod users {
    tonic::include_proto!("users");
}
pub mod posts {
    tonic::include_proto!("posts");
}
pub mod comments {
    tonic::include_proto!("comments");
}