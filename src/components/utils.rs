/// Creates a unique ID for use in components like the [`CheckboxList`],
/// [`RadioList`], and [`ChipsList`].
pub fn create_unique_id(name: &String, value: &String) -> String {
    let value_no_spaces = value
        .clone()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    format!("{}-{}", name.clone(), value_no_spaces)
}
