use std::env;

mod options;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let cli_args = options::Arguments::new(&args);
    let url = cli_args.unwrap().url;

    let res = reqwest::get(&url).await?;
    let body = res.text().await?;

    println!("{}", body);

    Ok(())
}
