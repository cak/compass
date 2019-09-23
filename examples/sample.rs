use compass::CompassError;

fn main() -> Result<(), CompassError> {
    let target = String::from("target_url");
    let res = compass::fetch_pretty(&target)?;
    println!("{}", res);
    Ok(())
}
