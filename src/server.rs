use dotenv::dotenv;
use lib::config;
use lib::home::home::home_page_server::{HomePage, HomePageServer};
use lib::home::home::{HomePageRequest, HomePageResponse};
use lib::service::users_service;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct HomePageService {}

#[tonic::async_trait]
impl HomePage for HomePageService {
    async fn get_home_page_data(
        &self,
        request: Request<HomePageRequest>,
    ) -> Result<Response<HomePageResponse>, Status> {
        println!("Got a request: {:?}", request);
        let _req = request.into_inner();
        let _res = users_service::get_user_data(&_req.id).await.unwrap();
        println!("here is the response: {:?}", _res);

        Ok(Response::new(_res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let addr = config::SOCKET.parse()?;
    let home_service = HomePageService::default();

    Server::builder()
        .add_service(HomePageServer::new(home_service))
        .serve(addr)
        .await?;

    Ok(())
}
