use crate::client::Client;

pub trait CallableCommand {
    type Response;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error>;
}
