use exitfailure::ExitFailure;
use std::env;
use statistics_cli::Response;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {

    let args: Vec<String> = env::args().collect();

    let ico = args.get(1).expect("ICO not found.");

    let res = Response::get(ico).await?;
    let company = res.results.first().expect("Company not found");
    let company_name = company.full_names.first().expect("Company name not found");
    println!("{}", company_name.value);

    Ok(())

}
