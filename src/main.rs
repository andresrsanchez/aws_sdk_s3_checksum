use aws_sdk_s3::model::{ChecksumAlgorithm, ChecksumMode};
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Error, Region};

#[tokio::main]
async fn main() {
    let config = aws_config::from_env()
        .region(Region::new("eu-central-1"))
        .load()
        .await;
    let client = Client::new(&config);
    let _ = download(&client).await;
}

async fn upload(client: &Client) -> Result<(), Error> {
    let resp = client
        .put_object()
        .bucket("")
        .key("checksum")
        .checksum_algorithm(ChecksumAlgorithm::Sha256)
        .body(ByteStream::from(vec![1, 2, 3]))
        .send()
        .await;
    match resp {
        Ok(obj) => match obj.checksum_sha256() {
            Some(checksum) => println!("{}", checksum),
            None => panic!("cannot get checksum"),
        },
        Err(err) => {
            eprintln!("Error uploading {err}");
        }
    }
    Ok(())
}
async fn download(client: &Client) -> Result<(), Error> {
    let resp = client
        .get_object()
        .bucket("")
        .key("checksum")
        .checksum_mode(ChecksumMode::Enabled)
        .send()
        .await;
    match resp {
        Ok(obj) => match obj.checksum_sha256() {
            Some(checksum) => println!("{}", checksum),
            None => panic!("cannot get checksum"),
        },
        Err(err) => {
            eprintln!("Error uploading {err}");
        }
    }
    Ok(())
}
