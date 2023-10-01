use exitfailure::ExitFailure;
use std::env;
use std::process::exit;
use statistics_cli::Response;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {

    let args: Vec<String> = env::args().collect();

    let ico = match args.get(1) {
        Some(val) => val,
        None => exit(1)
    };

    let res = Response::get(ico).await?;
    let company_name = match res.results.first() {
        Some(val) => {
            match val.full_names.first() {
                Some(name) => name.value.to_string(),
                None => exit(1)
            }
        },
        None => exit(1)
    };
    println!("{}", company_name);

    Ok(())

}
