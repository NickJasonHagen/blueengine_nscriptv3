use crate::*;
use blue_engine::{ShaderSettings,Renderer,ObjectStorage};
#[derive(Clone)]
pub struct BluencTextNode{
    pub text : String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub font: String,
    pub lenght:usize,
    pub chartextures:Vec<String>,
    pub color:Vec<f32>,

}
impl BluencTextNode{
    pub fn new(text:String,posx:f32,posy:f32,size:f32,font:String,lenght:usize) -> BluencTextNode{
        BluencTextNode{
            text: text,
            x:posx,
            y:posy,
            size:size,
            font:font,
            lenght:lenght,
            chartextures: Vec::new(),
            color: vec!(0.0,0.0,0.0,0.0),
        }
    }
}
pub struct BluencHud{
    pub objectsettingslayer1: ObjectSettings,
    pub textnodes: HashMap<Box<str>,BluencTextNode>,
}

impl BluencHud{
    pub fn new() ->BluencHud{
        BluencHud{
            objectsettingslayer1: ObjectSettings {camera_effect: None,shader_settings: ShaderSettings::default()},
            textnodes: HashMap::new()
        }
    }
    pub fn gettextnode(&mut self,name:&str) ->BluencTextNode{
        if let Some(this) = self.textnodes.get_mut(name){
            return this.clone();
        }
        print("error on textnodes, cant get the textnode, returning emptynode","r");
        return BluencTextNode::new("".to_string(), -2.0, -2.0, 10.0, "".to_string(), 0);
    }

    pub fn q_handler(&mut self,
        engine:&mut blue_engine::Engine,
        bluencobjects:&mut BluencObjects,
        storage:&mut NscriptStorage
    ){
        let qbuffer = &storage.customdata.static_vec_vec_vec_string[Q_TEXTNODES];
        if qbuffer.len() > 0{
            for i in qbuffer{
                //if i.len() > 0{ // if queed items in pool
                let mut counter = 0;

                //let prop = i[0].to_string() + ".text";
                let text = i[1].to_string();//self.vmap.getvar(&prop);
                //let prop = i.to_string() + ".x";
                let text_x = Nstring::f32(&i[2]);
                //let prop = i.to_string() + ".y";
                let text_y = Nstring::f32(&i[3]);
                //let prop = i.to_string() + ".size";
                let text_size = Nstring::f32(&i[4]);
                let scalewidth = text_size / 20.0;
                let scaleheight = text_size / 16.0;
                //let mut distancebetweenchars: f32;
                //distancebetweenchars = text_size / (1400.0 - (text_size * 21.0));

                //let prop = i.to_string() + ".font";
                let usedfont = &i[5];//self.vmap.getvar(&prop);
                let mut setposx = text_x.clone();
                let mut thistextnode = BluencTextNode::new(text.clone(),Nstring::f32(&i[2]),Nstring::f32(&i[3]),Nstring::f32(&i[4]),i[5].to_string(),0);
                for xchar in split(&text.trim(),""){
                    if xchar != "" {
                        let charnode = i[0].to_string() + "_textchar_" + &counter.to_string();
                        counter +=1;
                        charsquare(
                            &charnode.clone(),
                            self.objectsettingslayer1.clone(),
                            &mut engine.renderer,
                            &mut engine.objects,
                        )
                            .unwrap();
                        //print("adding characters to font","r");
                        let layer1 = engine.objects.get_mut(charnode.as_str()).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&charnode));

                        layer1.set_render_order(3);
                        layer1.camera_effect = None; //Some("main".into());
                        let textureprop = usedfont.to_string() + "/" + &BluencHud::pngcharname(&xchar) + ".png";
                        thistextnode.chartextures.push(textureprop.to_string());
                        //print(&textureprop,"r");
                        layer1.reference_texture(&textureprop);
                        layer1.resize(Vector3::new(scalewidth/100.0, scaleheight/100.0, 0.0));
                        layer1.set_position((setposx, text_y, 0.0));

                        layer1.set_color(thistextnode.color[0],thistextnode.color[1],thistextnode.color[2],thistextnode.color[3]);
                        setposx = BluencHud::pngfontcharspace(&xchar,&setposx,&text_size);

                    }
                }
                thistextnode.lenght = counter;
                //let tnode = BluencTextNode::new(text.clone(),Nstring::f32(&i[2]),Nstring::f32(&i[3]),Nstring::f32(&i[4]),i[5].to_string(),counter.clone());
                self.textnodes.insert(i[0].clone().into(),thistextnode);

                // remove the entree from the pool
                //self.vmap.setvar("blueengine.camera_q".to_owned(),&poolremove(&qbuffer,&i) );

                //self.vmap.setvar(i.to_string()+".lenght",&counter.to_string());
                //}

            }
            storage.customdata.static_vec_vec_vec_string[Q_TEXTNODES] = Vec::new();
        }
        let qbuffer = storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESUPDATES].clone();//self.vmap.getstringarray("blueengine.textnodeupdate_q");
        if qbuffer.len() != 0{

            for i in qbuffer{
                //if i.len() > -1{ // if queed items in pool
                let mut counter = 0;
                let mut thistextnode = self.gettextnode(i[0].as_str());
                let text = i[1].to_string();//thistextnode.text.to_string();
                //let text_size = thistextnode.size.clone();
                //let usedfont = thistextnode.font.clone();
                let newfont = i[5].to_string();
                let newx = Nstring::f32(&i[2]);
                let newy = Nstring::f32(&i[3]);
                let mut setposx = newx.clone();
                let splitchars = split(&text,"");

                let newlenght = splitchars.len();
                let newsize = Nstring::f32(&i[4]);
                let oldlenght = thistextnode.lenght.clone();//.len();
                let  scalewidth = newsize / 20.0;
                let  scaleheight = newsize   / 16.0;
                let mut rescalefromhere = false;
                let currenttexturevec = thistextnode.chartextures.clone();
                thistextnode.chartextures = Vec::new();
                let mut texture_i = 0;
                for xchar in &splitchars{
                    if xchar != &"" {
                        let charnode = i[0].to_string() + "_textchar_" + &counter.to_string();
                        counter +=1;
                        // check if the chrsize increased then add new nodes
                        if thistextnode.lenght < counter && counter < newlenght{
                            rescalefromhere = true;
                            let _ = charsquare(

                                &charnode.clone(),
                                self.objectsettingslayer1.clone(),
                                &mut engine.renderer,
                                &mut engine.objects,
                            );
                            //
                            // let layer1 = engine.objects.get_mut(charnode.as_str()).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&charnode));
                            // layer1.set_position(Vector3::new(setposx, newy, 1.0));
                        }

                        let layer1 = engine.objects.get_mut(charnode.as_str()).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&charnode));
                        layer1.set_render_order(3);
                        layer1.camera_effect = None ;

                        // check for texture changes
                        let texture = newfont.to_string() + "/" + &BluencHud::pngcharname(&xchar) + ".png";
                        let chrtexture;
                        if currenttexturevec.len() <= texture_i {chrtexture = "reset".to_string();}
                        else{chrtexture = currenttexturevec[texture_i].to_string();}
                        texture_i +=1;
                        if chrtexture != texture  || newfont != thistextnode.font{
                            rescalefromhere = true;
                            layer1.reference_texture(texture.clone());
                        }
                        thistextnode.chartextures.push(texture.to_string());
                        // check for changes on scale
                        if newlenght != thistextnode.lenght || rescalefromhere {
                            layer1.resize(Vector3::new(scalewidth/100.0, scaleheight/100.0, 0.0));
                            layer1.set_position(Vector3::new(setposx, newy, 0.0));
                            layer1.set_color(thistextnode.color[0],thistextnode.color[1],thistextnode.color[2],thistextnode.color[3]);
                            //layer1.flag_as_changed(true);
                        }
                        setposx = BluencHud::pngfontcharspace(&xchar,&setposx,&newsize);
                    }
                }

                // wipe the remaining
                let  mut counter2 = counter.clone();
                if counter < oldlenght {
                    for _xspace in counter..thistextnode.lenght{
                        let charnode = i[0].to_string() + "_textchar_" + &counter2.to_string();
                        //
                        // let layer1 = engine.objects.get_mut(charnode.as_str()).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&charnode));
                        // let texture = newfont.to_string() + "/space.png";
                        // layer1.reference_texture(texture.clone());
                        engine.objects.remove(charnode.as_str());
                        counter2 +=1;

                    }
                }
                thistextnode.lenght= counter;
                thistextnode.text = i[1].to_string();
                thistextnode.x = Nstring::f32(&i[2]);
                thistextnode.y = Nstring::f32(&i[3]);
                thistextnode.size = Nstring::f32(&i[4]);
                thistextnode.font = newfont;
                self.textnodes.insert(i[0].to_string().into(),thistextnode);

                //}
            }
            storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESUPDATES] = Vec::new();
        }

        for i in &storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESDELETE] {
            let mut counter = 0;
            let thistextnode = self.gettextnode(i[0].as_str());
            print(&i[0],"r");
            if thistextnode.lenght > 0 {
                for _xspace in counter..thistextnode.lenght{
                    let charnode = i[0].to_string() + "_textchar_" + &counter.to_string();
                    engine.objects.remove(charnode.as_str());
                    counter +=1;

                }
                self.textnodes.remove(i[0].as_str());
            }

        }
        storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESDELETE] = Vec::new();
        // 2d images squares
        if storage.customdata.static_vec_vec_vec_string[Q_IMAGE2D].len() >0{
            for i in &storage.customdata.static_vec_vec_vec_string[Q_IMAGE2D] {
                //if i[0] != ""{
                let _ = charsquare(

                    &i[0].clone(),
                    self.objectsettingslayer1.clone(),
                    &mut engine.renderer,
                    &mut engine.objects,
                );

                bluencobjects.newobject(&i[0]);
                let layer1 = engine.objects.get_mut(i[0].as_str()).expect(&format!("Unexpectedly, the '{}' 2Dobject wasn't found.",&i[0]));
                layer1.set_render_order(Nstring::usize(&i[6])).reference_texture(&i[1]).set_position(Vector3::new(Nstring::f32(&i[2]),Nstring::f32(&i[3]),0.0)).set_scale(Vector3::new(Nstring::f32(&i[4]),Nstring::f32(&i[5]),0.0));
                layer1.camera_effect = None ;

                //}
            }
            storage.customdata.static_vec_vec_vec_string[Q_IMAGE2D] = Vec::new();

        }

        if storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESCOLOR].len() >0{
            for i in &storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESCOLOR] {
                let mut counter = 0;
                let mut thistextnode = self.gettextnode(i[0].as_str());
                thistextnode.color = vec!(Nstring::f32(&i[1]),Nstring::f32(&i[2]),Nstring::f32(&i[3]),Nstring::f32(&i[4]));
                print(&i[0],"r");
                if thistextnode.lenght > 0 {
                    for _xspace in counter..thistextnode.lenght{
                        let charnode = i[0].to_string() + "_textchar_" + &counter.to_string();
                        let layer1 = engine.objects.get_mut(charnode.as_str()).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&i[0]));
                        layer1.set_color(thistextnode.color[3],thistextnode.color[0],thistextnode.color[1],thistextnode.color[2]);
                        layer1.flag_as_changed(true);// = true;
                        counter +=1;

                    }
                    //self.textnodes.remove(i[0].as_str());
                }

                self.textnodes.insert(i[0].as_str().into(), thistextnode);
            }
            storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESCOLOR] = Vec::new();

        }


    }
    pub fn pngcharname(char:&str) ->String{
        let chrtexturename = match char.to_lowercase().as_str(){
            "1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"0"| "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" |"r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" =>{
                return char.to_string();
            }
            "." => {"dot".to_string()}
            " " => {"space".to_string()}
            "," => {"comma".to_string()}
            "(" => {"parenthesisopen".to_string()}
            ")" => {"parenthesisclose".to_string()}
            "{" => {"curlybrackeyopen".to_string()}
            "}" => {"curlybrackeyclose".to_string()}
            "]" => {"squarebrackeyclose".to_string()}
            "[" => {"squarebrackeyopen".to_string()}
            ">" => {"anglebrackeyclose".to_string()}
            "<" => {"anglebrackeyopen".to_string()}
            "*" => {"asterix".to_string()}
            "&" => {"ampersand".to_string()}
            ":" => {"colon".to_string()}
            "!" => {"exclamationmark".to_string()}
            "$" => {"dollar".to_string()}
            "?" => {"questionmark".to_string()}
            "@" => {"at".to_string()}
            ";" => {"semicolon".to_string()}
            "|" => {"pipe".to_string()}
            "-" => {"dash".to_string()}
            "+" => {"plus".to_string()}
            "#" => {"hashtag".to_string()}
            "%" => {"percentage".to_string()}
            "^" => {"caret".to_string()}
            "_" => {"underscore".to_string()}
            "=" => {"equals".to_string()}
            "/" => {"slash".to_string()}
            "\\" => {"backslash".to_string()}


            _ => "space".to_string()
        };
        return chrtexturename;
    }
   pub fn pngfontcharspace(xchar: &str,setposx:&f32,textsize: &f32)->f32{
        let dist:f32;
        match xchar {// custom adjusting for characters
            "1" | "i" | " " | "." | "," | "-" | ";" | ":" | "|" | "_" | "!" => {
               dist = 0.48

            }
            "f" | "t" | "l" | "(" | ")" | "{" | "}" =>{
                dist = 0.55;
            }

            "y" | "z" | "x" | "d" | "u" | "r" | "s" | "o" | "c" | "a" | "@" =>{
                dist = 0.65;

            }
            "w" | "\\" | "/" | "#" | "m" => {
               dist = 0.66;
            }
            _ =>{
                dist = 0.61;
            }
        }

        return setposx + (textsize/((750.0-textsize))*dist); //- (textsize/1000.0));
    }

}

pub fn charsquare(
    name: impl AsRef<str>,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> Result<(), blue_engine::error::Error> {
    objects.insert(
        name.as_ref().into(),
        Object::new(
            name,
            vec![
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    uv: [1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    uv: [1.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    uv: [0.0, 1.0],
                    normal: [0.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    uv: [0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                },
            ],
            vec![2, 1, 0, 2, 0, 3],
            settings,
            renderer,
        )?,
    );

    Ok(())
}

pub fn nscriptfn_textnode(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let objectname = storage.getargstring(&args[0], block);
    let text = storage.getargstring(&args[1], block);
    let x = storage.getargstring(&args[2], block);
    let y = storage.getargstring(&args[3], block);
    let size = storage.getargstring(&args[4], block);
    let font = storage.getargstring(&args[5], block);
    storage.customdata.static_vec_vec_vec_string[Q_TEXTNODES].push(vec!(objectname,text,x,y,size,font));
    NscriptVar::new("v")
}
pub fn nscriptfn_textnodeupdate(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let objectname = storage.getargstring(&args[0], block);
    let text = storage.getargstring(&args[1], block);
    let x = storage.getargstring(&args[2], block);
    let y = storage.getargstring(&args[3], block);
    let size = storage.getargstring(&args[4], block);
    let font = storage.getargstring(&args[5], block);
    storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESUPDATES].push(vec!(objectname,text,x,y,size,font));
    NscriptVar::new("v")
}
pub fn nscriptfn_textnodedelete(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let objectname = storage.getargstring(&args[0], block);
    storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESDELETE].push(vec!(objectname));
    NscriptVar::new("v")
}
pub fn nscriptfn_image2d(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let objectname = storage.getargstring(&args[0], block);
    let texture = storage.getargstring(&args[1], block);
    let posx = storage.getargstring(&args[2], block);
    let posy = storage.getargstring(&args[3], block);
    let width = storage.getargstring(&args[4], block);
    let height = storage.getargstring(&args[5], block);
    let layer = storage.getargstring(&args[6], block);
    storage.customdata.static_vec_vec_vec_string[Q_IMAGE2D].push(vec!(objectname,texture,posx,posy,width,height,layer));
    NscriptVar::new("v")
}
pub fn nscriptfn_textnodesetcolor(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let objectname = storage.getargstring(&args[0], block);
    let red = storage.getargstring(&args[1], block);
    let green = storage.getargstring(&args[2], block);
    let blue = storage.getargstring(&args[3], block);
    let alpha = storage.getargstring(&args[4], block);
    storage.customdata.static_vec_vec_vec_string[Q_TEXTNODESCOLOR].push(vec!(objectname,red,green,blue,alpha));
    NscriptVar::new("v")
}
