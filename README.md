# compass
Web Application Recon Tool (BETA)

## Example

```Rust
fn main() -> Result<(), CompassError> {
    let target = String::from("target_url");
    let res = compass::fetch_pretty(&target)?;
    println!("{}", res);
    Ok(())
}
```

