use super::*;

#[test]
fn optional_item_defaults_to_none() {
    assert_eq!(
        Item::optional("user_id", None),
        Item {
            label: Text::from("user_id"),
            value: Text::from("none"),
        }
    );
}
