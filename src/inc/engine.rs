use std::usize;
//use egui_extras::*;
use crate::*;


pub struct Bluenc {

}
impl Bluenc{
    pub fn update(
        &mut self,
        mut renderer: &mut blue_engine::Renderer,
        window: &blue_engine::Window,
        objects: &mut blue_engine::ObjectStorage,
        camera: &mut blue_engine::CameraContainer,
        input: &blue_engine::InputHelper,
    ) {

    }
    pub fn startengine()->blue_engine::Engine{
        let powermode = "true";
        let mut cfgpowermode = blue_engine::PowerPreference::LowPower;
        if powermode == "true" {
            cfgpowermode = blue_engine::PowerPreference::HighPerformance;
        }
        let vsync = "false";
        #[cfg(not(windows))]
        let mut cfgvsync = blue_engine::wgpu::PresentMode::Immediate;
        #[cfg(not(windows))]
        if vsync != "false" && vsync != "" {
            cfgvsync = blue_engine::wgpu::PresentMode::Fifo;
        }
        let rendermode = "Vulkan";
        let cfgrender = "Vulkan";//self.vmap.getvar("blueengine.render");
        let rendermode = match cfgrender{
            "DX12" => {blue_engine::Backends::DX12}
            "Vulkan" => {blue_engine::Backends::VULKAN}
            "Metal" => {blue_engine::Backends::METAL}
            "GL" | _ => {blue_engine::Backends::GL}
        };
        let cfgtitle ="test123...";
        let renderheight = 600;
        let renderwidth = 1200;
        let rendertitle = "test";
        let mut engine = Engine::new_config(blue_engine::WindowDescriptor {
            width: renderwidth as u32, height: renderheight as u32, title: rendertitle,
            power_preference: cfgpowermode,
            present_mode: cfgvsync,
            features: blue_engine::header::imports::wgpu::Features::empty(),
            backends: rendermode,
            ..Default::default()
        })
            .expect("win");
        engine.window.set_title(cfgtitle.to_string());
        engine
    }
}
// impl Signal for Bluenc {
//     fn frame(
//         &mut self,
//         renderer: &mut blue_engine::Renderer,
//         window: &blue_engine::Window,
//         objects: &mut blue_engine::ObjectStorage,
//         camera: &mut blue_engine::CameraContainer,
//         input: &blue_engine::InputHelper,
//         encoder: &mut blue_engine::CommandEncoder,
//         view: &blue_engine::TextureView,
//     ) {
//     }
// }

