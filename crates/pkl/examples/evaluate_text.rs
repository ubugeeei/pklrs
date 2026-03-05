/// One-shot evaluation with `evaluate_text` — no manual manager setup needed.
use pkl::evaluate_text;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value = evaluate_text(
        r#"
        name = "quick-eval"
        items = new Listing { 1; 2; 3 }
        "#,
    )?;

    println!("{value:#?}");

    // You can also deserialize the result
    let name = pkl::from_pkl_value::<String>(
        value
            .as_properties()
            .unwrap()
            .get("name")
            .unwrap(),
    )?;
    println!("name = {name}");

    Ok(())
}
