use crate::*;
// wrapped function to include all rustfn at once in main()
pub fn nscript_blueengine_bindings(nscript: &mut Nscript){
    nscript.insertfn("setcamerapos",nscriptfn_setcamerapos,"setcamerapos(objectid,x:float,y:float,z:float)");
    nscript.insertfn("objectsetposition",nscriptfn_objectsetposition,"objectsetposition(objectid,x:float,y:float,z:float)");
    nscript.insertfn("objectsetcolor",nscriptfn_objectsetcolor,"objectsetcolor(objectid,int,int,int,int) // sets the objects color with a argb");
    nscript.insertfn("objectsetrotation",nscriptfn_objectsetrotation,"objectsetrotation(objectid,x:float,y:float,z:float)");
    nscript.insertfn("objectsetscale",nscriptfn_objectsetscale,"objectsetscale(objectid,x:float,y:float,z:float)");
    nscript.insertfn("cube",nscriptfn_cube,"cube(objectid,x:float,y:float,z:float) // returns objectid , spawns a cube in 3d");
    nscript.insertfn("triangle",nscriptfn_triangle,"triangle(objectid,x:float,y:float,z:float) // returns objectid , spawns a triangle in 3d");
    nscript.insertfn("square",nscriptfn_square,"square(objectid,x:float,y:float,z:float) // returns objectid , spawns a square in 3d");
    nscript.insertfn("loadtexture",nscriptfn_loadtexture,"loadtexture(filepath) // returns filepath");
    nscript.insertfn("settexture",nscriptfn_settexture,"settexture(objectid,filepath) // returns filepath");
    nscript.insertfn("objectremove",nscriptfn_objectremove,"objectremove(objectid) // returns objectid");
    nscript.insertfn("loadsprite",nscriptfn_loadsprite,"loadsprite(sprite_dir) // returns spriteid");
    nscript.insertfn("addsprite",nscriptfn_allspritesadd,"addsprite(sprite_object) // adds to the all sprite index for changes");
    nscript.insertfn("removesprite",nscriptfn_allspritesremove,"removesprite(sprite_object) // removes a sprite from the index");
    nscript.insertfn("spritesetanimation",nscriptfn_spritesetanimation,"spritesetanimation(sprite_object,animationname) // sets a new row of sprites to a object");
    nscript.insertfn("allsprites",nscriptfn_allsprites,"allsprites() // returns a vector with all sprites");
    nscript.insertfn("batchedmodel_modelspawn",nscriptfn_batchedmodel_modelspawn,"batchedmodel_modelspawn(newobjecid,modelfile) // spawns a batchedmodel from a file");
    nscript.insertfn("batchedmodel_spawntobuffer",nscriptfn_batchedmodel_spawntobuffer,"batchedmodel_spawntobuffer(texture,x,y,z,rx,ry,rz,sx,sy,sz,shape_as_string) // spawns a model to a buffer, shape:cube,triangle,square ,spawn with spawnfrombuffer");
    nscript.insertfn("batchedmodel_spawnfrombuffer",nscriptfn_batchedmodel_spawnfrombuffer,"batchedmodel_spawnfrombuffer(newobjectid,buffer) //returns objectid,  spawns a model from a buffer");
    nscript.insertfn("batchedmodel_buildfromfile",nscriptfn_batchedmodel_buildfromfile,"batchedmodel_buildfromfile(file) //returns objectid,  preloads a file, uses the filename as objectreference : use with spawnfrombuffer()");
    nscript.insertfn("batchedmodel_setposition",nscriptfn_batchedmodel_setposition,"batchedmodel_setposition(object,x,y,z) // adjusts the objects matrix");
    nscript.insertfn("batchedmodel_setrotation",nscriptfn_batchedmodel_setrotation,"batchedmodel_setrotation(object,x,y,z) // adjusts the objects matrix");
    nscript.insertfn("batchedmodel_setscale",nscriptfn_batchedmodel_setscale,"batchedmodel_setscale(object,x,y,z) // adjusts the objects matrix");
    nscript.insertfn("batchedmodel_delete",nscriptfn_batchedmodel_delete,"batchedmodel_delete(object) //  deletes the objects matrix");
    nscript.insertfn("batchedmodel_modelbuffertofile",nscriptfn_batchedmodel_modelbuffertofile,"batchedmodel_modelbuffertofile(buffer,file) //  buffer to file");
    nscript.insertfn("textnode",nscriptfn_textnode,"textnode(objname,text,x,y,size,font) //  renders text on the screen on a camera disabled layer");
    nscript.insertfn("textnodeupdate",nscriptfn_textnodeupdate,"textnodeupdate(objname,text,x,y,size,font) //  updates rendered text on the screen on a camera disabled layer, see textnode()");
    nscript.insertfn("textnodedelete",nscriptfn_textnodedelete,"textnodedelete(objname) //  deletes rendered text on the screen on a camera disabled layer, see textnode()");
    nscript.insertfn("image2d",nscriptfn_image2d,"image2d(objname,textureref,posx,posy,scalex,scaley) //  creates a square object on the hud layer.");
    nscript.insertfn("textnodesetcolor",nscriptfn_textnodesetcolor,"textnodesetcolor(objname,f32:red,f32:green,f32:blue,f32:alpha) //  change the color of a textnode");
    nscript.insertfn("updatewindow",nscriptfn_updatewindow,"updatewindow() //  sets $windowsize as a vec [width,height]");
    nscript.insertfn("pixelstocoords",nscriptfn_pixelstocoords,"pixelstocoords(pos[x,y],screen[width,height]) // takes 2 vectors2's calculates the pixels to coords -1.0 to 1.0 and returns a vec[x,y] with the results");
}
pub fn nscriptfn_updatewindow(_args:&Vec<&str>,_block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    storage.customdata.static_vec_vec_vec_string[Q_EVENTS].push(vec!("updatewindow".to_string()));
    NscriptVar::new("windowupdated")
}
pub fn nscriptfn_pixelstocoords(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let pos = storage.getargstringvec(args[0], block);
    let screensize = storage.getargstringvec(args[1], block);
    let mut x  = Nstring::f32(&pos[0]);
    let mut y = Nstring::f32(&pos[1]);

    let width = Nstring::f32(&screensize[0]);
    let height = Nstring::f32(&screensize[1]);
    if x > width {
        x = width.clone();
    }
    if x < 0.0 {
        x = 0.0;
    }
    if y > height {
        y = height.clone();
    }
    if y < 0.0 {
        y = 0.0;
    }
    let xproc = width/200.0;
    let yproc = height/200.0;

    let xp = -1.0 + ((x/xproc) * 0.01);
    let yp = 1.0 -((y/yproc) * 0.01);
//print(&format!("x {} y {} w {} h {}",&x,&y,width,height),"r");
    NscriptVar::newvec("windowupdated",vec!(xp.to_string(),yp.to_string()))
}

pub fn nscriptfn_setcamerapos(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let var = storage.getvar(&args[1], block);
     storage.objectsetprop(&name,"x",var);
    let var = storage.getvar(&args[2], block);
     storage.objectsetprop(&name,"y",var);
    let var = storage.getvar(&args[3], block);
     storage.objectsetprop(&name,"z",var);

    let var = storage.getvar(&args[4], block);
     storage.objectsetprop(&name,"targetx",var);
    let var = storage.getvar(&args[5], block);
     storage.objectsetprop(&name,"targety",var);
    let var = storage.getvar(&args[6], block);
     storage.objectsetprop(&name,"targetz",var);
    storage.customdata.static_vec_bool[Q_CAMERA] = true;
    NscriptVar::new("camera")
}
pub fn nscriptfn_objectsetcolor(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let a = storage.getargstring(&args[1], block);
    let r = storage.getargstring(&args[2], block);
    let g = storage.getargstring(&args[3], block);
    let b = storage.getargstring(&args[4], block);
    storage.customdata.static_vec_vec_vec_string[Q_COLOR].push(vec!(name,a,r,g,b));
    NscriptVar::new("v")
}
pub fn nscriptfn_objectsetposition(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let x = Nstring::f32(&storage.getargstring(&args[1], block));
    let y = Nstring::f32(&storage.getargstring(&args[2], block));
    let z = Nstring::f32(&storage.getargstring(&args[3], block));
    storage.customdata.static_vec_vec_string_vector3_32[Q_POSITION].push((name,x,y,z));
    NscriptVar::new("v")
}
pub fn nscriptfn_objectsetrotation(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let x = Nstring::f32(&storage.getargstring(&args[1], block));
    let y = Nstring::f32(&storage.getargstring(&args[2], block));
    let z = Nstring::f32(&storage.getargstring(&args[3], block));
    storage.customdata.static_vec_vec_string_vector3_32[Q_ROTATION].push((name,x,y,z));
    NscriptVar::new("v")
}
pub fn nscriptfn_objectsetscale(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let x = Nstring::f32(&storage.getargstring(&args[1], block));
    let y = Nstring::f32(&storage.getargstring(&args[2], block));
    let z = Nstring::f32(&storage.getargstring(&args[3], block));
    storage.customdata.static_vec_vec_string_vector3_32[Q_SCALE].push((name,x,y,z));
    NscriptVar::new("v")
}
pub fn nscriptfn_square(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let x = Nstring::f32(&storage.getargstring(&args[1], block));
    let y = Nstring::f32(&storage.getargstring(&args[2], block));
    let z = Nstring::f32(&storage.getargstring(&args[3], block));
    storage.customdata.static_vec_vec_string_vector3_32[Q_SQUARE].push((name.to_string(),x,y,z));
    NscriptVar::newstring("v",name)
}
pub fn nscriptfn_triangle(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let x = Nstring::f32(&storage.getargstring(&args[1], block));
    let y = Nstring::f32(&storage.getargstring(&args[2], block));
    let z = Nstring::f32(&storage.getargstring(&args[3], block));
    storage.customdata.static_vec_vec_string_vector3_32[Q_TRIANGLE].push((name.to_string(),x,y,z));
    NscriptVar::newstring("v",name)
}
pub fn nscriptfn_cube(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let x = Nstring::f32(&storage.getargstring(&args[1], block));
    let y = Nstring::f32(&storage.getargstring(&args[2], block));
    let z = Nstring::f32(&storage.getargstring(&args[3], block));
    storage.customdata.static_vec_vec_string_vector3_32[Q_CUBE].push((name.to_string(),x,y,z));
    NscriptVar::newstring("v",name)
}
pub fn nscriptfn_loadtexture(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    storage.customdata.static_vec_vec_string[Q_LOADTEXTURE].push(name.to_string());
    NscriptVar::newstring("v",name)
}
pub fn nscriptfn_settexture(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    let path = storage.getargstring(&args[1], block);
    storage.customdata.static_vec_vec_vec_string[Q_SETTEXTURE].push(vec!(name,path.to_string()));
    NscriptVar::newstring("v",path)
}
pub fn nscriptfn_objectremove(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let name = storage.getargstring(&args[0], block);
    storage.customdata.static_vec_vec_string[Q_DELETE].push(name.to_string());
    NscriptVar::newstring("v",name)
}
