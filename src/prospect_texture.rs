use wgpu::*;

use crate::{utils::prospect_fs::{read_file_panic, read_file_option}, abstraction::{high_level_abstraction::HighLevelGraphicsContext, prospect_window::ProspectWindow}};

pub struct ProspectTexture
{
    name : String,
    view : TextureView,
}

impl ProspectTexture
{
    pub fn from_file<S : AsRef<str>>(name : &str, path : S, window : &mut ProspectWindow) -> Result<Self, ()>
    {
        let path = path.as_ref().to_string();
        let contents = read_file_option(path);
        if contents.is_none()
        {
            return Err(());
        }
        let contents = contents.unwrap();
        let contents = contents.as_bytes();

        Ok(Self
        {
            name : name.to_string(),
            view : HighLevelGraphicsContext::create_texture_from_file(name, contents, window),
        })
    }

    pub fn from_string<S : AsRef<str>>(name : &str, contents : S, window : &mut ProspectWindow) -> Self
    {
        let contents = contents.as_ref();
        let contents = contents.as_bytes();

        Self
        {
            name : name.to_string(),
            view : HighLevelGraphicsContext::create_texture_from_file(name, contents, window),
        }
    }

    pub fn from_bytes(name : &str, contents : &[u8], window : &mut ProspectWindow) -> Self
    {
        Self
        {
            name : name.to_string(),
            view : HighLevelGraphicsContext::create_texture_from_file(name, contents, window),
        }
    }

    pub fn get_name(&self) -> String
    {   
        self.name.clone()
    }

    pub fn get_texture_view(&self) -> &TextureView
    {
        &self.view
    }
}