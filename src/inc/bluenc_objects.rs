use crate::*;
pub struct BluencObject{
    name: Arc<str>,
    x: f32,
    y: f32,
    z: f32,
    rx: f32,
    ry: f32,
    rz: f32,
    sx: f32,
    sy: f32,
    sz: f32,
    texture: Box<str>,
}
impl BluencObject{
    pub fn new(name:&str) -> BluencObject{
        BluencObject{
            name: name.into(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            rx: 0.0,
            ry: 0.0,
            rz: 0.0,
            sx: 0.5,
            sy: 0.5,
            sz: 0.5,
            texture: "".into(),
        }
    }

    pub fn setposition(&mut self,engine: &mut blue_engine::Engine,x:f32,y:f32,z:f32) ->&mut Self{
        self.x = x;
        self.y = y;
        self.z = z;
        let obj = engine.objects.get_mut(&self.name).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&self.name));
        obj.set_position(Vector3::new(self.x,self.y,self.z));
        self
    }
    pub fn setrotation(&mut self,engine: &mut blue_engine::Engine,x:f32,y:f32,z:f32)->&mut Self{
        self.rx = x;
        self.ry = y;
        self.rz = z;
        let obj = engine.objects.get_mut(&self.name).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&self.name));
        obj.set_rotation(Vector3::new(self.rx,self.ry,self.rz));
        self
    }
    pub fn setscale(&mut self,engine: &mut blue_engine::Engine,x:f32,y:f32,z:f32){
        self.sx = x;
        self.sy = y;
        self.sz = z;
        let obj = engine.objects.get_mut(&self.name).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&self.name));
        obj.set_scale(Vector3::new(self.sx,self.sy,self.sz));
    }
    pub fn setcolor(&mut self,engine: &mut blue_engine::Engine,a:f32,r:f32,g:f32,b:f32){
        let obj = engine.objects.get_mut(&self.name).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&self.name));
        obj.set_color(a,r,g,b);
    }
    pub fn settexture(&mut self, engine: &mut blue_engine::Engine , filepath:&str,_textures:&mut BluencTextures){
        self.texture = filepath.into();
        let obj = engine.objects.get_mut(&self.name).expect(&format!("Unexpectedly, the '{}' object wasn't found.",&self.name));
        obj.reference_texture(filepath);
        // if let Some(textureloaded) = textures.get(&filepath){
        //     obj.set_texture("triangle", blue_engine::TextureData::Image(textureloaded.clone()), blue_engine::TextureMode::Repeat, &mut engine.renderer)
        //         .expect("Unexpectedly, the floor texture couldn't be processed.");
        // }
    }
}
pub struct BluencObjects{
    pub storage: HashMap<Box<str>,BluencObject>,
    unknownobject: BluencObject,
}
impl BluencObjects{
    pub fn new() -> BluencObjects{
        BluencObjects{
            storage: HashMap::new(),
            unknownobject: BluencObject::new("unknown")
        }
    }
    pub fn newobject(&mut self,name:&str){
        self.storage.insert(name.into(),BluencObject::new(&name));
    }
    pub fn square(&mut self,engine:&mut blue_engine::Engine,name:&str) -> &mut Self{
        square(
            &name,
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        )
            .unwrap();
        self.storage.insert(name.into(),BluencObject::new(&name));
        self
    }
    pub fn cube(&mut self,engine:&mut blue_engine::Engine,name:&str) -> &mut Self{
        cube(
            &name,
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        )
            .unwrap();
        self.storage.insert(name.into(),BluencObject::new(&name));
        self
    }
    pub fn triangle(&mut self,engine:&mut blue_engine::Engine,name:&str) -> &mut Self{
        triangle(
            &name,
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        )
            .unwrap();
        self.storage.insert(name.into(),BluencObject::new(&name));
        self
    }

    pub fn getobject(&mut self,objectid:&str) ->&mut BluencObject{
        if let Some(foundobject) = self.storage.get_mut(objectid.into()){
            return foundobject
        }
        println!("cant find bluenc_object {}",&objectid);
        return &mut self.unknownobject
    }
    pub fn settexture(&mut self,engine:&mut blue_engine::Engine,name:&str,texturepath:&str,textures:&mut BluencTextures){
        if let Some(object) = self.storage.get_mut(name.into()){
            object.settexture(engine, texturepath, textures);
        };
    }
    pub fn q_handler(&mut self, engine:&mut blue_engine::Engine,storage:&mut NscriptStorage,textures:&mut BluencTextures){
        //load textures
        let vect = &storage.customdata.static_vec_vec_string[Q_LOADTEXTURE];
        if vect.len() >0{
            for loadtexture in vect{
                textures.load(engine,&loadtexture);
            }
            storage.customdata.static_vec_vec_string[Q_LOADTEXTURE] = Vec::new();
        }

        //squares
        let vect = &storage.customdata.static_vec_vec_string_vector3_32[Q_SQUARE];
        if vect.len() >0{
            for xobject in vect{
                let thisobject = self.square(engine,&xobject.0).getobject(&xobject.0);
                thisobject.setposition(engine, xobject.1, xobject.2, xobject.3);
            }
            storage.customdata.static_vec_vec_string_vector3_32[Q_SQUARE] = Vec::new();
        }

        //triangles
        let vect =&storage.customdata.static_vec_vec_string_vector3_32[Q_TRIANGLE];
        if vect.len()>0{
            for xobject in vect{
                let thisobject = self.triangle(engine,&xobject.0).getobject(&xobject.0);
                thisobject.setposition(engine, xobject.1, xobject.2, xobject.3);
            }
            storage.customdata.static_vec_vec_string_vector3_32[Q_TRIANGLE] = Vec::new();
        }

        //cubes
        let vect = &storage.customdata.static_vec_vec_string_vector3_32[Q_CUBE];
        if vect.len() >0{
            for xobject in vect{
                let thisobject = self.cube(engine,&xobject.0).getobject(&xobject.0);
                thisobject.setposition(engine, xobject.1, xobject.2, xobject.3);
            }
            storage.customdata.static_vec_vec_string_vector3_32[Q_CUBE] = Vec::new();
        }

        //positions
        let vect =&storage.customdata.static_vec_vec_string_vector3_32[Q_POSITION];
        if vect.len()>0{

            for xobject in vect{
                let thisobject = self.getobject(&xobject.0);
                thisobject.setposition(engine, xobject.1, xobject.2, xobject.3);
            }
            storage.customdata.static_vec_vec_string_vector3_32[Q_POSITION] = Vec::new();
        }

        //rotations
        let vect = &storage.customdata.static_vec_vec_string_vector3_32[Q_ROTATION];
        if vect.len()>0{
            for xobject in vect{
                let thisobject = self.getobject(&xobject.0);
                thisobject.setrotation(engine, xobject.1, xobject.2, xobject.3);
            }
            storage.customdata.static_vec_vec_string_vector3_32[Q_ROTATION] = Vec::new();
        }

        //scales
        let vect = &storage.customdata.static_vec_vec_string_vector3_32[Q_SCALE];
        if vect.len()>0{
            for xobject in vect{
                let thisobject = self.getobject(&xobject.0);
                thisobject.setscale(engine, xobject.1, xobject.2, xobject.3);
            }
            storage.customdata.static_vec_vec_string_vector3_32[Q_SCALE] = Vec::new();
        }

        //set textures
        let vect = &storage.customdata.static_vec_vec_vec_string[Q_SETTEXTURE];
        if vect.len()>0{
            for loadtexture in vect{
                let thisobject = self.getobject(&loadtexture[0]);
                thisobject.settexture(engine, &loadtexture[1], textures);
            }
            storage.customdata.static_vec_vec_vec_string[Q_SETTEXTURE] = Vec::new();
        }
        //set color
        let vect = &storage.customdata.static_vec_vec_vec_string[Q_COLOR];
        if vect.len()>0{
            for argb in vect{
                let thisobject = self.getobject(&argb[0]);
                thisobject.setcolor(engine, Nstring::f32(&argb[1]),  Nstring::f32(&argb[2]), Nstring::f32(&argb[3]),Nstring::f32(&argb[4]) );
            }
            storage.customdata.static_vec_vec_vec_string[Q_COLOR] = Vec::new();
        }


        //set textures
        let vect =&storage.customdata.static_vec_vec_string[Q_DELETE];
        if vect.len()>0{
            for thisobj in vect{
                engine.objects.remove(thisobj.as_str());//.expect(&format!("Unexpectedly, the '{}' object wasn't found.",&thisobj));
            }
            storage.customdata.static_vec_vec_string[Q_DELETE] = Vec::new();
        }

    }
}
