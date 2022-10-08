use std::env;
use std::time::Duration;
use rusty_s3::{Bucket, Credentials, S3Action, UrlStyle};

fn main() {
    let access_key = "devcade2022AccessKeyinconstantly38254-unaccomplished";
    let secret_key = "devcade2022SecretKeylabor-intensive1699-wretchedness";

    // setting up a bucket
    let endpoint = "https://s3.csh.rit.edu".parse().expect("endpoint is a valid Url");
    let path_style = UrlStyle::VirtualHost;
    let name = "devcade-games";
    //let region = "eu-west-1";
    let bucket = Bucket::new(endpoint, path_style, name, "").expect("Url has a valid scheme and host");

    // setting up the credentials
    //let key = env::var(access_key).expect("AWS_ACCESS_KEY_ID is set and a valid String");
    //let secret = env::var(secret_key).expect("AWS_ACCESS_KEY_ID is set and a valid String");
    let credentials = Credentials::new(access_key, secret_key);

    // signing a request
    let presigned_url_duration = Duration::from_secs(60 * 60);
    let action = bucket.get_object(Some(&credentials), "bankshot.zip");
    println!("GET {}", action.sign(presigned_url_duration));
}
