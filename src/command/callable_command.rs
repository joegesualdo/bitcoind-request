use crate::client::Client;

pub trait CallableCommand {
    type Response;
    fn call(&self, client: &Client) -> Self::Response;
}
