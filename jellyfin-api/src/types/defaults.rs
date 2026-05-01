pub(crate) fn default_bool<const V: bool>() -> bool {
    V
}

pub(crate) fn system_info_encoder_location() -> Option<String>
{
    Option::Some("System".to_string())
}

pub(crate) fn system_info_system_architecture(
) -> Option<String> {
    Option::Some("X64".to_string())
}
