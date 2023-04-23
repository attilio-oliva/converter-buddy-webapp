use std::sync::Arc;

use converter_buddy::{format::Format, config::Config};
use runtime_injector::{define_module, interface, Injector, IntoSingleton, Svc};

use crate::services::conversions::{
    ConversionError, ConversionService, DummyConversion, LocalConversion,
};

pub struct ConversionServiceManager {
    data_service: Svc<dyn ConversionService>,
}

impl ConversionServiceManager {
    // This is just a normal constructor. The only requirement is that each
    // parameter is a valid injectable dependency.
    pub fn new(data_service: Svc<dyn ConversionService>) -> Self {
        ConversionServiceManager { data_service }
    }

    pub fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        source_format: Format,
        config: Config
    ) -> Result<(), ConversionError> {
        self.data_service
            .process(input, output, source_format, config)
    }
}

interface!(dyn ConversionService = [DummyConversion, LocalConversion]);

pub fn get_dummy_service() -> Arc<dyn ConversionService> {
    let module = define_module! {
        services = [ConversionServiceManager::new.singleton()],
        interfaces = {
            dyn ConversionService = [DummyConversion::default.singleton()],
        },
    };

    let mut builder = Injector::builder();
    builder.add_module(module);
    let injector = builder.build();
    injector.get().unwrap()
}

pub fn get_local_service() -> Arc<dyn ConversionService> {
    let module = define_module! {
        services = [ConversionServiceManager::new.singleton()],
        interfaces = {
            dyn ConversionService = [LocalConversion::default.singleton()],
        },
    };

    let mut builder = Injector::builder();
    builder.add_module(module);
    let injector = builder.build();
    injector.get().unwrap()
}
