use gloo_file::File;
use yewdux::prelude::Store;

#[derive(Default, Clone, PartialEq, Store)]
pub struct FileList {
    pub list: Vec<File>,
}
