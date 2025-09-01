use std::usize;

use crate::*;
pub struct BluencAnimation{
    pub animation_map: HashMap<String,Vec<String>>,
    pub animation_i: HashMap<String,usize>,
    pub animation_timers:HashMap<String,i64>,
    pub animation_currentframe:HashMap<String,String>,
    pub animationcounter: usize,
}

impl BluencAnimation{
    pub fn new() -> BluencAnimation{
        BluencAnimation{
            animation_map: HashMap::new(),
            animation_i: HashMap::new(),
            animation_timers: HashMap::new(),
            animation_currentframe: HashMap::new(),
            animationcounter: 0,// number of anims to run check in a frame

        }
    }
    pub fn q_handler(&mut self, _engine:&mut blue_engine::Engine,storage:&mut NscriptStorage){
        //load textures
        let anim = &storage.customdata.static_vec_vec_vec_string[Q_ANIMATION].clone();
        if anim.len()>0{
            for xitem in anim{
                let animvec = storage.objectgetprop(&xitem[0], &xitem[1]);
                self.set(&xitem[0],&animvec.stringvec);
            }

            storage.customdata.static_vec_vec_vec_string[Q_ANIMATION] = Vec::new();
        }

        let anim = &storage.customdata.static_vec_vec_vec_string[Q_ANIMATIONTIME];
        if anim.len()>0{
            for xitem in anim{
                self.setanimationtime(&xitem[0],Nstring::i64(&xitem[1]));
            }
            storage.customdata.static_vec_vec_vec_string[Q_ANIMATIONTIME] = Vec::new();
        }

        //animation handler
        let vect = &storage.customdata.static_vec_vec_string[BNC_ALLANIMS];
        let qmax =vect.len();
        if qmax>0{
            let mut topos = self.animationcounter + 25;
            if topos > qmax {
                topos = qmax.clone()+ 1;
            }
            for x in &vect[self.animationcounter..topos-1]{
                if x != ""{
                    let spriteframe = self.frame(&x);
                    if spriteframe != ""{
                        if storage.customdata.static_vec_vec_vec_string.len() >0{
                            storage.customdata.static_vec_vec_vec_string[Q_SETTEXTURE].push(vec!(x.to_string(),spriteframe));
                        }else{
                            storage.customdata.static_vec_vec_vec_string.push(Vec::new());
                        }
                    }

                    if topos >= qmax-1 {
                        topos = 1;
                    }
                self.animationcounter = topos-1;
                }
            }
        }


    }
    pub fn setanimationtime(&mut self,squareid: &str,animationtime:i64){
        self.animation_timers.insert(squareid.to_string(),animationtime);
    }

    pub fn set(&mut self,squareid: &str,animationarray: &Vec<String>){
        if animationarray.len() < 1 {
            //println!("error on the animation system, a given array doesnt meet the requirements, for ID:{}",squareid);
            return ;
        }
        let start_index = 1;
        let end_index = animationarray.len(); // Use the length of the array for the end index
        // Create a slice from start_index to the end of the array
        let trimmed_slice:Vec<String> = animationarray[start_index..end_index].to_owned();
        //rintln!("Settingarrayy   = {:?}",trimmed_slice);
        self.animation_map.insert(squareid.to_string(),trimmed_slice );
        self.animation_i.insert(squareid.to_string(),0);

        let timer = match animationarray[0].parse::<i64>(){
            Ok(f) => {f}
            Err(_e) => {20}
        };
        self.animation_timers.insert(squareid.to_string(),timer);
        self.animation_timers.insert(squareid.to_string() + "_diff",timer);

    }
    pub fn get(&mut self,squareid: &str) -> String{
        match self.animation_map.get(squareid){
            Some(res) => {
                println!("Found: {:?}",res);
            }
            None =>{
                //print!("Animation texture error for: {}",squareid);
            }
        }
        "".to_string()
    }
    pub fn frame(&mut self,squareid:&str) -> String{
        if squareid == ""{
            return "".to_owned();
        }
        let animarray: Vec<String> = match self.animation_map.get(squareid){
            Some(res) => {res.to_owned()}
            None =>{Vec::new()}
        };
        let mut anim_i: usize = match self.animation_i.get(squareid){
            Some(res) => {res.to_owned()}
            None =>{0}
        };
        let  frametime: i64 = match self.animation_timers.get(squareid){
            Some(res) => {res.to_owned()}
            None =>{0}
        };

        let timerdiffref = squareid.to_owned() + "_diff";
        let  timerdiff: i64 = match self.animation_timers.get(&timerdiffref){
            Some(res) => {res.to_owned()}
            None =>{0}
        };
        if Ntimer::diff(timerdiff) > frametime{
            if animarray.len() == 0 {
                return "".to_owned();
            }
            if anim_i >= animarray.len(){
                anim_i = 0
            }

            let thisframe = animarray[anim_i].to_string();
            anim_i = anim_i + 1;
            self.animation_i.insert(squareid.to_string(), anim_i);
            self.animation_timers.insert(timerdiffref, Ntimer::init());
            self.animation_currentframe.insert(squareid.to_string(),thisframe.clone());
             return thisframe;
        }
        return "".to_string();
        // let  runningframe = match self.animation_currentframe.get(squareid){
        //     Some(res) => {res.to_owned()}
        //     None =>{"".to_string()}
        // };
        //return runningframe;

    }
}

pub fn nscriptfn_loadsprite(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let dir = storage.getargstring(&args[0], block);
    //storage.setprop(&dir, "spritedir", NscriptVar::newstring("spritedir", dir.to_string()), block);
    for xfile in Nfile::dirtolist(&dir,false){
        if Nstring::instring(&xfile,".png") {
            // pushing textures to loader_q
            //print(&xfile,"bg");
            storage.customdata.static_vec_vec_string[Q_LOADTEXTURE].push(format!("{}{}",dir, xfile));
        }
    }
    let file = Nfile::read(&format!("{}/sprite.cfg", dir));
storage.classes.insert(dir.to_string(),NscriptClass::new(&dir));
    print(&dir,"orange");
    for xline in split(&file,NC_LINE_ENDING){
        let propname = split(xline," = ")[0].trim();
        if propname != ""{
            //print(&format!("[{}]", &propname),"pink");
            let array = Nstring::split(&Nstring::stringbetween(xline,"[","]"),",");
            let mut fullnamearray: Vec<String> = Vec::new();
            let mut xi = 0;
            for xitem in array{
                //print(&xitem,"grey");
                if xi == 0 {
                    fullnamearray.push(xitem);// first entree is the timer not a texture.
                    xi =1;
                }
                else{// continue all textures
                    fullnamearray.push(format!("{}/{}.png",&dir,xitem));
                }
            }
            storage.objectsetprop(&dir, propname, NscriptVar::newvec(propname.into(),fullnamearray));
        }

    }

    NscriptVar::newstring(".", dir)
}

pub fn nscriptfn_spritesetanimation(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let obj = storage.getargstring(&args[0], block);
    let anim = storage.getargstring(&args[1], block);

    //print(&format!("set sprite [{}] with anim [{}]",&obj,&anim),"br");
    storage.customdata.static_vec_vec_vec_string[Q_ANIMATION].push(vec!(obj,anim));
    //storage.customdata.static_vec_vec_string.push(vec!(obj,anim));
    NscriptVar::new(".")
}

pub fn nscriptfn_allspritesadd(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let obj = storage.getargstring(&args[0], block);
    let array = &storage.customdata.static_vec_vec_string[BNC_ALLANIMS];
    let mut set = true;
    for x in array{
        if x == &obj{
            set = false;
            break;
        }
    }
    if set{
        //print(&format!("adding[{}]",&obj),"br");
        storage.customdata.static_vec_vec_string[BNC_ALLANIMS].push(obj);
    }
    NscriptVar::new(".")
}
pub fn nscriptfn_allspritesremove(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let obj = storage.getargstring(&args[0], block);
     storage.customdata.static_vec_vec_string[BNC_ALLANIMS].retain(|x| x != &obj);

    NscriptVar::new(".")
}
pub fn nscriptfn_allsprites(_args:&Vec<&str>,_block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    NscriptVar::newvec(".",storage.customdata.static_vec_vec_string[BNC_ALLANIMS].clone())
}
pub fn nscriptfn_spritesetanimationtime(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let spriteid = storage.getargstring(args[0], block);
    let animationtime = storage.getargstring(args[1], block);
    storage.customdata.static_vec_vec_vec_string[Q_ANIMATIONTIME].push(vec!(spriteid,split(&animationtime,".")[0].to_string()));

    NscriptVar::new(".")
}
