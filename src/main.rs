/*
 * Blue Engine by Elham Aryanpur
 *
 * Basic GUI example
 *
 * Licensed under Apache-2.0
*/

// Gui is a trait that you'll be using to add your UI
//#[cfg(feature = "egui")] print("no coroutines!","r");
use std::process;
use blue_engine::Vector3;


// includes van dit project
mod inc{
    pub mod nscriptfnbindings;
    pub mod bluenc_camera;
    pub mod bluenc_textures;
    pub mod bluenc_objects;
    pub mod bluenc_animation;
    pub mod bluenc_batchedmodels;
    pub mod bluenc_hud;
    pub mod bluenc_input;
}
pub use inc::bluenc_camera::*;
pub use inc::bluenc_hud::*;
pub use inc::bluenc_textures::*;
pub use inc::bluenc_objects::*;
pub use inc::nscriptfnbindings::*;
pub use inc::bluenc_animation::*;
pub use inc::bluenc_batchedmodels::*;
pub use inc::bluenc_input::*;


//pub use blue_engine_utilities::egui_plugin;
//#[cfg(feature = "egui")]
//pub use blue_engine_utilities::egui_plugin::egui as gui;
extern crate nscript_lib;
pub use nscript_lib::*;
// Basic imports
//#[cfg(feature = "egui")]
pub use blue_engine::{Engine,KeyCode,Object,Vertex, ObjectSettings,Signal, primitive_shapes::*};

pub const Q_POSITION: usize = 1;
pub const Q_SCALE: usize= 2;
pub const Q_ROTATION: usize= 3;
pub const Q_SETTEXTURE: usize= 0;
pub const Q_LOADTEXTURE: usize= 5;
pub const Q_CAMERA: usize= 6;
pub const Q_SQUARE: usize= 7;
pub const Q_TRIANGLE: usize= 8;
pub const Q_CUBE: usize= 9;
pub const Q_DELETE: usize= 10;
pub const Q_ANIMATION: usize= 11;
pub const BNC_SQUARE_Q: usize = 1;
pub const BNC_ALLANIMS: usize = 12;
pub const Q_CUSTOMMODELS: usize = 13;
pub const Q_COLOR: usize = 14;
pub const Q_TEXTNODES: usize = 15;
pub const Q_TEXTNODESUPDATES: usize = 16;
pub const Q_TEXTNODESDELETE: usize = 17;
pub const Q_IMAGE2D: usize = 18;
pub const Q_TEXTNODESCOLOR: usize = 19;
pub const Q_EVENTS: usize = 20;
pub const Q_ANIMATIONTIME: usize = 21;
pub struct BlueNc{
    //name:String,
    keyvec:Vec<blue_engine::KeyCode>,
    keynames:Vec<Box<str>>,
    codeblock:NscriptCodeBlock,
    textures:BluencTextures,
    objects:BluencObjects,
    animation:BluencAnimation,
    batchedmodels:BatchedModels,
    hud:BluencHud,
}
impl BlueNc{
    pub fn new()->BlueNc{
        let mut this = BlueNc{
            //name:"oi".to_string(),
            keyvec: Vec::new(),
            keynames: Vec::new(),
            codeblock: NscriptCodeBlock::new("BlueNc"),
            textures: BluencTextures::new(),
            objects: BluencObjects::new(),
            animation: BluencAnimation::new(),
            batchedmodels: BatchedModels::new(),
            hud: BluencHud::new(),
        };
        let keyvec = [
            blue_engine::KeyCode::Escape,
            blue_engine::KeyCode::Enter,
            blue_engine::KeyCode::Space,
            blue_engine::KeyCode::AltLeft,
            blue_engine::KeyCode::AltRight,
            blue_engine::KeyCode::ControlLeft,
            blue_engine::KeyCode::ControlRight,
            blue_engine::KeyCode::ShiftLeft,
            blue_engine::KeyCode::ShiftRight,
            blue_engine::KeyCode::Tab,
            blue_engine::KeyCode::CapsLock,
            blue_engine::KeyCode::F1,
            blue_engine::KeyCode::F2,
            blue_engine::KeyCode::F3,
            blue_engine::KeyCode::F4,
            blue_engine::KeyCode::F5,
            blue_engine::KeyCode::F6,
            blue_engine::KeyCode::F7,
            blue_engine::KeyCode::F8,
            blue_engine::KeyCode::F9,
            blue_engine::KeyCode::F10,
            blue_engine::KeyCode::F11,
            blue_engine::KeyCode::F12,
            blue_engine::KeyCode::Digit0,
            blue_engine::KeyCode::Digit1,
            blue_engine::KeyCode::Digit2,
            blue_engine::KeyCode::Digit3,
            blue_engine::KeyCode::Digit4,
            blue_engine::KeyCode::Digit5,
            blue_engine::KeyCode::Digit6,
            blue_engine::KeyCode::Digit7,
            blue_engine::KeyCode::Digit8,
            blue_engine::KeyCode::Digit9,
            blue_engine::KeyCode::ArrowUp,
            blue_engine::KeyCode::ArrowDown,
            blue_engine::KeyCode::ArrowLeft,
            blue_engine::KeyCode::ArrowRight,
            blue_engine::KeyCode::KeyA,
            blue_engine::KeyCode::KeyB,
            blue_engine::KeyCode::KeyC,
            blue_engine::KeyCode::KeyD,
            blue_engine::KeyCode::KeyE,
            blue_engine::KeyCode::KeyF,
            blue_engine::KeyCode::KeyG,
            blue_engine::KeyCode::KeyH,
            blue_engine::KeyCode::KeyI,
            blue_engine::KeyCode::KeyJ,
            blue_engine::KeyCode::KeyK,
            blue_engine::KeyCode::KeyL,
            blue_engine::KeyCode::KeyM,
            blue_engine::KeyCode::KeyN,
            blue_engine::KeyCode::KeyO,
            blue_engine::KeyCode::KeyP,
            blue_engine::KeyCode::KeyQ,
            blue_engine::KeyCode::KeyR,
            blue_engine::KeyCode::KeyS,
            blue_engine::KeyCode::KeyT,
            blue_engine::KeyCode::KeyU,
            blue_engine::KeyCode::KeyV,
            blue_engine::KeyCode::KeyW,
            blue_engine::KeyCode::KeyX,
            blue_engine::KeyCode::KeyY,
            blue_engine::KeyCode::KeyZ,
        ];
        for x in keyvec{
            this.keyvec.push(x);
        }
        let keyname = [
            // keymapping naming ( must contain the same size and order as the keymapping!!)
        "esc",
        "enter",
        "space",
        "altleft",
        "altright",
        "controlleft",
        "controlright",
        "shiftleft",
        "shiftright",
        "tab",
        "capslock",
        "f1",
        "f2",
        "f3",
        "f4",
        "f5",
        "f6",
        "f7",
        "f8",
        "f9",
        "f10",
        "f11",
        "f12",
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "up",
        "down",
        "left",
        "right",
        "a",
        "b",
        "c",
        "d",
        "e",
        "f",
        "g",
        "h",
        "i",
        "j",
        "k",
        "l",
        "m",
        "n",
        "o",
        "p",
        "q",
        "r",
        "s",
        "t",
        "u",
        "v",
        "w",
        "x",
        "y",
        "z",
    ];
        for x in keyname{
            this.keynames.push(x.into());
        }
        this

    }
    pub fn start_blueengine(block:&mut NscriptCodeBlock, storage: &mut NscriptStorage)->blue_engine::Engine{
        let vsync = storage.classgetprop("blueengine","vsync",block).stringdata.clone();
        let powermode = storage.classgetprop("blueengine","powermode",block).stringdata.clone();

        let mut renderwidth = Nstring::i32(&storage.classgetprop("blueengine","renderwidth",block).stringdata) ;
        let mut renderheight = Nstring::i32(&storage.classgetprop("blueengine","renderheight",block).stringdata);
        let rendertitle = "static title needs to be fixed";
        let cfgtitle = storage.classgetprop("blueengine","title",block).stringdata.clone();
        let mut cfgpowermode = blue_engine::PowerPreference::LowPower;
        if powermode == "true" || powermode == "!true" {
            cfgpowermode = blue_engine::PowerPreference::HighPerformance;
        }

        #[cfg(windows)]
        let mut cfgvsync = blue_engine::wgpu::PresentMode::AutoNoVsync;
        #[cfg(windows)]
        if vsync != "false" && vsync != "" {
            cfgvsync = blue_engine::wgpu::PresentMode::AutoVsync;
        }
        #[cfg(not(windows))]
        let mut cfgvsync = blue_engine::wgpu::PresentMode::Immediate;
        #[cfg(not(windows))]
        if vsync != "false" && vsync != "!false" {
            cfgvsync = blue_engine::wgpu::PresentMode::Fifo;
        }

        if renderwidth < 160 {
            renderwidth = 160
        }
        if renderheight < 120 {
            renderheight = 120
        }
        let cfgrender = storage.classgetprop("blueengine","render",block).stringdata.to_string();
        let rendermode = match cfgrender.as_str(){
            "DX12" => {blue_engine::Backends::DX12}
            "Vulkan" => {blue_engine::Backends::VULKAN}
            "Metal" => {blue_engine::Backends::METAL}
            "Primary" => {blue_engine::Backends::PRIMARY}
            "GL" | _ => {blue_engine::Backends::GL}
        };
        let mut engine = Engine::new_config(blue_engine::EngineSettings {
            width: renderwidth as u32, height: renderheight as u32, title: rendertitle,
            power_preference: cfgpowermode,
            present_mode: cfgvsync,
            features: blue_engine::wgpu::Features::empty(),
            backends: rendermode,
            ..Default::default()
        })
            .expect("win");
        engine.window.set_title(cfgtitle.to_string());
        engine
    }

    pub fn init(&mut self,storage:&mut NscriptStorage){
        for _x in 0..10{
            storage.customdata.static_vec_vec_string_vector3.push(Vec::new());
        }
        storage.customdata.static_vec_vec_string_vector3[Q_POSITION].push(("a".to_string(),1.00,10.0,0.005))
    }
    pub fn getkeyevents(&mut self,input: &blue_engine::InputHelper,storage:&mut NscriptStorage){
        let mut idx = 0;
        storage.setprop("key", "event", NscriptVar::newstring("key.event","false".to_string()), &mut self.codeblock);
        for xkey in &self.keynames{
            if input.key_held(self.keyvec[idx]) {
                storage.setprop("key", "event", NscriptVar::newstring("key.event","true".to_string()), &mut self.codeblock);
                storage.setprop("key", &xkey, NscriptVar::newstring("key","down".to_string()), &mut self.codeblock);
                // self.vmap.setvar(self.keynames[idx].to_owned(),"down");
                // self.vmap.setvar("key.event".to_owned(),"true");
                //cwrite(&keyname[idx],"y")
            }
            else{

                storage.setprop("key", &xkey, NscriptVar::newstring("key","up".to_string()), &mut self.codeblock);
                //self.vmap.setvar(self.keynames[idx].to_owned(),"up");
            }
            idx +=1;
        }
        if input.mouse_held(blue_engine::MouseButton::Left){
            storage.setprop("mousekey", "left", NscriptVar::newstring("key","down".to_string()), &mut self.codeblock);
        }
        else{
            storage.setprop("mousekey", "left", NscriptVar::newstring("key","up".to_string()), &mut self.codeblock);
        }
        if input.mouse_held(blue_engine::MouseButton::Right){
            storage.setprop("mousekey", "right", NscriptVar::newstring("key","down".to_string()), &mut self.codeblock);
        }
        else{
            storage.setprop("mousekey", "right", NscriptVar::newstring("key","up".to_string()), &mut self.codeblock);
        }

        let mousepos = input.cursor().unwrap_or((0.0,0.0));
        storage.setprop("mouse", "pos",  NscriptVar::newvec("mousepos",vec!(mousepos.0.to_string(),mousepos.1.to_string())), &mut self.codeblock);
        let mousepos = input.cursor_diff();
        storage.setprop("mouse", "diff",  NscriptVar::newvec("mousepos",vec!(mousepos.0.to_string(),mousepos.1.to_string())), &mut self.codeblock);
        let mousepos = input.scroll_diff();
        storage.setprop("mouse", "scrolldiff",  NscriptVar::newvec("mousepos",vec!(mousepos.0.to_string(),mousepos.1.to_string())), &mut self.codeblock);
    }
    // fn update(&mut self, renderer: &mutblue_engine::Renderer,
    //     window: &blue_engine::Window,
    //     objects: &mut blue_engine::ObjectStorage){
    //
    //}

}

fn main() {
    //#[cfg(feature = "egui")]
    //{
    // Initialize the engine with default settings
    let mut nscript = Nscript::new();
       nscript.insertstructowned("mouse", BlueNcMouse::new());
    nscript.insertstructowned("gamepad", BlueNcGamePad::new());
for _x in 0..30{
        nscript.storage.customdata.static_vec_vec_string.push(Vec::new());
        nscript.storage.customdata.static_vec_vec_vec_string.push(Vec::new());
        nscript.storage.customdata.static_vec_vec_string_vector3_32.push(Vec::new());
        nscript.storage.customdata.static_vec_string.push("".to_string());
        nscript.storage.customdata.static_vec_bool.push(false);
        //print(&x.to_string(),"br");
    }
    nscript.parsefile("settings.nc");
    //let mut engine = Engine::new().expect("win");
    let mut tmpblock = NscriptCodeBlock::new("__root");
    let mut engine = BlueNc::start_blueengine(&mut tmpblock,&mut nscript.storage);

    nscript.storage.setglobal("$windowsize", NscriptVar::newvec("$windowsize", vec!(engine.renderer.config.width.to_string(),engine.renderer.config.height.to_string())));
    nscript_blueengine_bindings(&mut nscript);
    //
    // //let mut textures = BluencTextures::new();
    // // Add a triangle to the screen
    // cube(
    //     "cube",
    //     ObjectSettings::default(),
    //     &mut engine.renderer,
    //     &mut engine.objects,
    // )
    //     .unwrap();
    // triangle(
    //     "triangle",
    //     ObjectSettings::default(),
    //     &mut engine.renderer,
    //     &mut engine.objects,
    // )
    //     .unwrap();
    // square(
    //     "square",
    //     ObjectSettings::default(),
    //     &mut engine.renderer,
    //     &mut engine.objects,
    // )
    //     .unwrap();
    let mut bnc = BlueNc::new();//
    bnc.init(&mut nscript.storage);
    //let mut bnc_objects = BluencObjects::new();
//let bnccamera = BluencCamera::new();
    let script = nscript.storage.getglobal("$cmdarg1").stringdata;
    nscript.parsefile(&script);
    //textures = BluencTextures::new();
// let filepath = "assets/blood.png";
//     bnc.textures.load(&mut engine, &filepath);

    //bnc_objects.cube(&mut engine, "triangle").settexture(&mut engine,"triangle","assets/blood.png",&mut bnc.textures);
//let  obj = bnc.objects.getobject("triangle");
 //   obj.setposition(&mut engine , 2.0,8.0,0.0).setrotation(&mut engine, 14.0, 60.0, 34.0);
    //  let floor = engine.objects.get_mut("triangle").expect("Unexpectedly, the 'floor' object wasn't found.");
    // floor.set_position(Vector3::new(2.0,8.0,0.0));
    //bnc_objects.settexture(&mut engine,"triangle","assets/blood.png",&mut bnc.textures);
    // let floor = engine.objects.get_mut("triangle").expect("Unexpectedly, the 'floor' object wasn't found.");
    // floor.set_scale(Vector3::new(3.0,1.0,1.0));
    // floor.set_position(Vector3::new(0.0,1.0,-12.0));
    // if let Some(textureloaded) = bnc.textures.get(&filepath){
    //
    //     floor.set_texture("triangle", blue_engine::TextureData::Image(textureloaded.clone()), blue_engine::TextureMode::Repeat, &mut engine.renderer)
    //         .expect("Unexpectedly, the floor texture couldn't be processed.");
    // }
    // Start the egui context
    //let gui_context = egui_plugin::EGUIPlugin::new();

    //nscript.insertstructowned("bluenc",bnc);
    // We add the gui as plugin, which runs once before everything else to fetch events, and once during render times for rendering and other stuff
    //engine.signals.add_signal("egui", Box::new(gui_context));
    //engine.signals.add_signal("bnc", Box::new(nscript));
    //let mut color = [1f32, 1f32, 1f32, 1f32];
//     let scale =Vector3::new(0.1, 0.1, 0.1);
//     // Update loop
//         // engine
// let  input = blue_engine::InputHelper::new();


// if let Some(test) = bnc_objects.storage.get_mut("triangle"){
//
//         test.setposition(&mut engine, 1.0, 5.0, 0.0).setrotation(&mut engine, 12.0, 30.0, 0.0).setscale(&mut engine, 1.0, 5.0, 1.0);
//     };

//bnc.objects.q_handler(&mut engine,&mut nscript.storage,&mut bnc.textures);
    engine
        .update_loop(move |engine| {





            if nscript.coroutines.len() > 0 {
                // coroutines all run once then the function returns here.
                BluencCamera::queehandler(&mut engine.camera,&mut nscript.storage);

                bnc.objects.q_handler(engine,&mut nscript.storage,&mut bnc.textures);
                bnc.batchedmodels.q_handler(engine, &mut bnc.objects,&mut bnc.textures, &mut nscript.storage);
                bnc.animation.q_handler(engine,&mut nscript.storage);
                bnc.hud.q_handler(engine,&mut bnc.objects, &mut nscript.storage);
                if nscript.storage.customdata.static_vec_vec_vec_string[Q_EVENTS].len() >0{
                    for xevent in &nscript.storage.customdata.static_vec_vec_vec_string[Q_EVENTS].clone(){
                        match xevent[0].as_str(){
                            "updatewindow" =>{
                                nscript.storage.setglobal("$windowsize", NscriptVar::newvec("$windowsize", vec!(engine.renderer.config.width.to_string(),engine.renderer.config.height.to_string())));
                            }
                            _ =>{

                            }

                        }
                    }
                }
                // obtain the plugin
                // let egui_plugin = engine
                //     .signals
                //     .get_signal::<egui_plugin::EGUIPlugin>("egui")
                //     .expect("Plugin not found")
                //     .expect("Plugin type mismatch");
                //
                // // ui function will provide the context
                // egui_plugin.ui(
                //     |ctx| {
                //         gui::Window::new("title").show(ctx, |ui| {
                //             ui.horizontal(|ui| {
                //                 ui.label("Pick a color");
                //                 ui.color_edit_button_rgba_unmultiplied(&mut color);
                //             });
                //         });
                //
                //         engine
                //             .objects
                //             .get_mut("triangle")
                //             .unwrap()
                //             .set_color(color[0], color[1], color[2], color[3]);
                //     },
                //     &engine.window,
                // );
                    // Create a mouse listener


bnc.getkeyevents(&mut engine.simple_input, &mut nscript.storage);

                nscript.executecoroutines();
        // if &nscript.storage.getglobal("$testtriangle").stringdata == "go"{
        //
        //             triangle("asd",ObjectSettings::default(),&mut engine.renderer, &mut engine.objects).unwrap();
        //             nscript.storage.setglobal("$testtriangle", NscriptVar::new("v"));
        //         }
        }else{
            print("Exit; no more coroutines running!","red");
               process::exit(1);
            //break;
        }
            })
            .expect("Error during update loop");

   // }


}
