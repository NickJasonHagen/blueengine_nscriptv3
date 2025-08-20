use crate::*;
//use blue_engine::image::DynamicImage;

pub struct BluencTextures{
    texturemap: HashMap<Box<str>,blue_engine::image::DynamicImage>,
}

impl BluencTextures{
    pub fn new() -> BluencTextures{
        BluencTextures{
            texturemap: HashMap::new(),
        }
    }
    pub fn load(&mut self ,engine:&mut blue_engine::Engine,filepath: &str){
        if let None = self.texturemap.get(filepath.into()){
            self.texturemap.insert(filepath.into(),blue_engine::image::open(&filepath).expect(&format!("Unexpectedly, the image {} image wasn't found.",&filepath)));
            square(
                &filepath,
                ObjectSettings::default(),
                &mut engine.renderer,
                &mut engine.objects,
            )
                .unwrap();

            let obj = engine.objects.get_mut(filepath).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&filepath));
            if let Some(textureloaded) = self.get(&filepath){
                obj.set_texture(filepath, blue_engine::TextureData::Image(textureloaded.clone()), blue_engine::TextureMode::Repeat, &mut engine.renderer)
                    .expect("Unexpectedly, the floor texture couldn't be processed.");
                obj.set_position((0.0,-99999.0,0.0));
            }
        }

    }
    pub fn get(&mut self,filepath: &str) -> Option<&mut blue_engine::image::DynamicImage>{
        return self.texturemap.get_mut(filepath);
    }
}
