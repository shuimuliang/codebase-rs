use tonic::transport::Endpoint;
use tonic_health::proto::health_client::HealthClient;
use tonic_health::proto::HealthCheckRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dst = "http://[::1]:50051";
    let conn = Endpoint::new(dst)?.connect().await?;
    let mut client = HealthClient::new(conn);

    let response = client.watch(
        HealthCheckRequest {
            service: "helloworld.Greeter".into(),
        }).await.unwrap();

    let mut stream = response.into_inner();
    while let Ok(message) = stream.message().await {
        println!("new message: {:?}", message);
    }

    Ok(())
}
