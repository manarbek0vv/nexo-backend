extern crate prost_types;

pub mod proto {
    pub mod comments {
        tonic::include_proto!("comments");
    }
}