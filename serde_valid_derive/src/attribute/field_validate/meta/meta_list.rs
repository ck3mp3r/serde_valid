use crate::attribute::common::message_format::MessageFormat;
use crate::attribute::field_validate::generic::{
    extract_generic_custom_validator, extract_generic_enumerate_validator_from_list,
};
use crate::attribute::MetaListFieldValidation;
use crate::attribute::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;

pub fn extract_field_validator_from_meta_list(
    field: &impl Field,
    validation_type: MetaListFieldValidation,
    validation: &syn::MetaList,
    message_format: MessageFormat,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    match validation_type {
        MetaListFieldValidation::Enumerate => extract_generic_enumerate_validator_from_list(
            field,
            validation,
            message_format,
            rename_map,
        ),
        MetaListFieldValidation::Custom => {
            extract_generic_custom_validator(field, validation, message_format, rename_map)
        }
    }
}
