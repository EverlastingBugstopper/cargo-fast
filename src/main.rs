use clap::Clap;

#[derive(Clap, Debug)]
#[non_exhaustive]
#[clap(
    author,
    bin_name(env!("CARGO_PKG_NAME"))
)]
enum Cli {
    #[clap(name = "fast")]
    Fast,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Cli::parse();
    println!(
        r#"        _______
       //  ||\ \
 _____//___||_\ \___
)   _          _    \
 |_/ \________/ \___|
___\_/________\_/______"#
    );
    println!("why yes. yes it does.");
    Ok(())
}
