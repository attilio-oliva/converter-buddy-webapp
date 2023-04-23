use converter_buddy::{
    converter::{self, Converter},
    format::Format, config::Config,
};

use super::{traits::ConversionError, ConversionService};

#[derive(Default)]
pub struct LocalConversion;

impl ConversionService for LocalConversion {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        source_format: Format,
        config: Config
    ) -> Result<(), ConversionError> {
        let converter =
            Converter::try_from(source_format).map_err(|_| ConversionError::Unsupported)?;
        let config = config
            .try_into()
            .map_err(|_| ConversionError::Unsupported)?;
        converter
            .process(input, output, config)
            .map_err(|e| match e {
                converter::ConversionError::UnsupportedOperation => ConversionError::Unsupported,
                _ => ConversionError::Unexpected,
            })
    }
}
