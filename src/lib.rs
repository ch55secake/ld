use std::fs;
use derive_builder::Builder;

#[derive(Builder, Debug)]
pub(crate) struct DirectoryItem { 
    
    /// Name of the item within the directory 
    name: String,
    
    // Type of the item within the directory 
    f_type: String,
}


pub(crate) fn read_find() {
    
}