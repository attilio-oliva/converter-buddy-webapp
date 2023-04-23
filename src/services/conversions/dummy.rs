use converter_buddy::{format::Format, config::Config};

use super::{traits::ConversionError, ConversionService};

#[derive(Default)]
pub struct DummyConversion;

impl ConversionService for DummyConversion {
    fn process(
        &self,
        _input: &Vec<u8>,
        _output: &mut Vec<u8>,
        _source_format: Format,
        _config: Config
    ) -> Result<(), ConversionError> {
        Err(ConversionError::Unsupported)
    }
}
