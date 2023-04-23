use std::{ptr, sync::Arc};

use converter_buddy::{format::Format, config::Config};

#[derive(Debug)]
pub enum ConversionError {
    Unsupported,
    Unexpected,
}

pub trait ConversionService: Send + Sync {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        source_format: Format,
        config: Config,
    ) -> Result<(), ConversionError>;
}

pub struct DynamicService(pub Arc<dyn ConversionService>);

impl PartialEq for DynamicService {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self as *const _, other as *const _)
    }
}

impl Clone for DynamicService {
    fn clone(&self) -> Self {
        DynamicService(self.0.clone())
    }
}
