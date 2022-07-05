use converter_buddy::format::Format;

use super::{traits::ConversionError, ConversionService};

#[derive(Default)]
pub struct DummyConversion;

impl ConversionService for DummyConversion {
    fn process(
        &self,
        _input: &Vec<u8>,
        _output: &mut Vec<u8>,
        _source_format: Format,
        _target_format: Format,
    ) -> Result<(), ConversionError> {
        Err(ConversionError::Unsupported)
    }
}
