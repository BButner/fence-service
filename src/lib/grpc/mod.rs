use tonic::transport::Server;

pub mod display_manager;

pub async fn start_server() {
    let addr = "0.0.0.0:50052".parse().unwrap();

    Server::builder()
        .add_service(self::display_manager::get_service())
        .serve(addr)
        .await
        .unwrap();
}
