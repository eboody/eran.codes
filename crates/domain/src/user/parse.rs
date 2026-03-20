pub(super) fn is_valid_email_address(value: &str) -> bool {
    parse_email_address(value).is_ok()
}

fn parse_email_address(
    value: &str,
) -> Result<email_address::EmailAddress, email_address::Error> {
    value.parse()
}
