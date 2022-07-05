use converter_buddy::{converter, format::Format};

use super::{traits::ConversionError, ConversionService};

#[derive(Default)]
pub struct LocalConversion;

impl ConversionService for LocalConversion {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        source_format: Format,
        target_format: Format,
    ) -> Result<(), ConversionError> {
        let converter = converter::from_format(source_format);
        converter
            .process(input, output, target_format)
            .map_err(|e| match e {
                converter::ConversionError::UnsupportedOperation => ConversionError::Unsupported,
                _ => ConversionError::Unexpected,
            })
    }
}
