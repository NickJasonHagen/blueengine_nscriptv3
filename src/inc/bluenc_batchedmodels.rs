// batchedmodels , by Nick Jason Hagen aka skorm 2024, in use for Blueengine / BlueNc
// so with this multiple plains or cubes can be buffered into 1 object so that you can load it and
// keep a higher fps, however a batch can only be using 1 texture, so this will create 1 batch per
// texture and the system will return a array of created identifiers so you can adjust move and
// rotate them as a group at your end !
use crate::*;
use cgmath::{Matrix4, Vector3,Transform, Quaternion, Euler, Deg, Point3};
use std::io::{ Write};
use std::{fs, usize};
//use std::path::Path;
use std::fs::File;
pub struct BatchedModels{
    //availible shapes defaults
    vertices_square: std::vec::Vec<blue_engine::Vertex>,
    indices_square: Vec<u16>,
    vertices_triangle: std::vec::Vec<blue_engine::Vertex>,
    indices_triangle: Vec<u16>,
    vertices_cube: std::vec::Vec<blue_engine::Vertex>,
    indices_cube: Vec<u16>,
    // data storage part.
    objects_partmap: HashMap<String,Vec<String>>,// storage for object's subparts identifiers
    objects_texturemap: HashMap<String,Vec<String>>,// storage for object's subparts identifiers
    objects_vertices: HashMap<String,std::vec::Vec<blue_engine::Vertex>>,// stores subparts
    objects_indices: HashMap<String,Vec<u16>>, // also for subparts
    objects_rawmodeldata: HashMap<String,String>,//store filedata,used for batchtobatch buffer

}
impl BatchedModels{
    pub fn new() -> BatchedModels{
        let this = BatchedModels{
            vertices_square: vec![
                blue_engine::Vertex {
                position: [1.0, 1.0, 0.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, -1.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, -1.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, 1.0, 0.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
            ],
            indices_square: vec![2, 1, 0, 2, 0, 3],
            vertices_triangle:  vec![
                Vertex {
                position: [0.0, 1.0, 0.0],
                uv: [0.5, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, -1.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, -1.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
            ],
            indices_triangle: vec![0,1,2],
            vertices_cube: vec![
                // Front Face
                Vertex {
                position: [-1.0, -1.0, 1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, -1.0, 1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                // Back Face
                Vertex {
                position: [-1.0, 1.0, -1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, 1.0, -1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, -1.0, -1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, -1.0, -1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                // Right face
                Vertex {
                position: [1.0, -1.0, -1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, 1.0, -1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, 1.0, 1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, -1.0, 1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                // Left Face
                Vertex {
                position: [-1.0, -1.0, 1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, 1.0, 1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, 1.0, -1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, -1.0, -1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                // Top Face
                Vertex {
                position: [1.0, 1.0, -1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, 1.0, -1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, 1.0, 1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                // Bottom Face
                Vertex {
                position: [1.0, -1.0, 1.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, -1.0, 1.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [-1.0, -1.0, -1.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
                Vertex {
                position: [1.0, -1.0, -1.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
                },
            ],
            indices_cube: vec![
                0, 1, 2, 2, 3, 0, // top
                4, 5, 6, 6, 7, 4, // bottom
                8, 9, 10, 10, 11, 8, // right
                12, 13, 14, 14, 15, 12, // left
                16, 17, 18, 18, 19, 16, // front
                20, 21, 22, 22, 23, 20, // back
            ],
            objects_partmap: HashMap::new(),
            objects_texturemap: HashMap::new(),
            objects_vertices: HashMap::new(),
            objects_indices: HashMap::new(),
            objects_rawmodeldata: HashMap::new(),
        };
        return this;
    }

     fn getmodelparts(&mut self,modelname : &str) -> Vec<String>{
        let getid = self.objects_partmap.get_key_value(modelname);
        match getid {
            None => {
                Vec::new()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
     fn getmodelvertices(&mut self,modelname : &str) -> std::vec::Vec<blue_engine::Vertex>{
        let getid = self.objects_vertices.get_key_value(modelname);
        match getid {
            None => {
                Vec::new()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
     fn getmodelindices(&mut self,modelname : &str) -> std::vec::Vec<u16>{
        let getid = self.objects_indices.get_key_value(modelname);
        match getid {
            None => {
                Vec::new()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }

     fn getmodeltextures(&mut self,modelname : &str) -> std::vec::Vec<String>{
        let getid = self.objects_texturemap.get_key_value(modelname);
        match getid {
            None => {
                Vec::new()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
     fn getmodelfiledata(&mut self,modelname : &str) -> String{
        let getid = self.objects_rawmodeldata.get_key_value(modelname);
        match getid {
            None => {
                String::new()
            }
            Some((_i, res)) =>{
                res.to_owned()
            }
        }
    }
    /// rawbuffer, this adds full models to the modelbuffer to combine them into 1 big model
    /// the offsets for pos rot and scale will be added from 0 point ( like tiling)
    /// -- can be saved to file using: buffertofile(buffername,filepath)
    pub fn spawntobuffer(&mut self, bufferobjectname:&str,modeltoadd:&str, posx:&f32,posy:&f32,posz:&f32,posrx:&f32,posry:&f32,posrz:&f32,possx:&f32,possy:&f32,possz:&f32 ){
        let mut bufferdata = self.getmodelfiledata(&bufferobjectname);
        let rawmodelfile = self.getmodelfiledata(&modeltoadd);
        let lines:Vec<&str> = rawmodelfile.split("\n").collect();
        for line in lines{
            let  dataparts: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
            if dataparts.len() >10 {
                let newx = self.f32(&dataparts[1]) + posx;
                let newy = self.f32(&dataparts[2]) + posy;
                let newz = self.f32(&dataparts[3]) + posz;
                let newrx = self.f32(&dataparts[4]) + posrx;
                let newry = self.f32(&dataparts[5]) + posry;
                let newrz = self.f32(&dataparts[6]) + posrz;
                let newsx = self.f32(&dataparts[7]) + possx;
                let newsy = self.f32(&dataparts[8]) + possy;
                let newsz = self.f32(&dataparts[9]) + possz;
                 bufferdata = bufferdata + &dataparts[0]
                + "," + &newx.to_string()
                + "," + &newy.to_string()
                + "," + &newz.to_string()
                + "," + &newrx.to_string()
                + "," + &newry.to_string()
                + "," + &newrz.to_string()
                + "," + &newsx.to_string()
                + "," + &newsy.to_string()
                + "," + &newsz.to_string() + ",\n";
            }
        }
        self.objects_rawmodeldata.insert(bufferobjectname.to_string(), bufferdata);
    }
    /// uses data made by spawntobuffer() and writes the modelfiledata to the filepath
    pub fn buffertofile(&mut self,bufferobjectname:&str,filepath:&str) -> String {
        if filepath == "" || bufferobjectname == "" {
            println!("cannot write empty buffer or empty filename!");
            return "Error given argument is empty".to_string();
        } // cant be empty names
        let modeldata = self.getmodelfiledata(bufferobjectname);
        if modeldata == "" || modeldata.is_empty() {return "error no data in the buffer, preventing to write the file with empty data,  function returns".to_string();}
        if std::path::Path::new(&filepath).exists(){
            let  _error = match fs::remove_file(filepath) {
                Ok(_) => format!("File deleted successfully"),
                Err(err) =>{
                    return format!("Error writing a file ( cant delete,before write): {}", err);
                } ,
            };
        }
        let mut f = match File::create(filepath) {
            Ok(file) => file,
            Err(err) => return err.to_string(),
        };

        if let Err(err) = f.write_all(modeldata.as_bytes()) {
            return err.to_string();
        }

        if let Err(err) = f.sync_all() {
            return err.to_string();
        }
        "done".to_string()
    }
    /// uses spawntobuffer() data, this builds the model from the buffer
    /// returns the tuple with 3 vectors for iteration
    /// (Vertices , Indices , objectnames for textures referencing )
    pub fn spawnmodelfrombuffer(&mut self,objectbuffername:&str)->(Vec<Vec<blue_engine::Vertex>> , Vec<Vec<u16>>,Vec<String>){
        let data = self.getmodelfiledata(&objectbuffername);
        self.buildmodel(&objectbuffername, &data);
        self.spawnmodel(objectbuffername)
    }
    /// returns a tuple with 2 vectors for the model parts indices and vertices
    /// model must be loaded by buildmodel() otherwise it will do it at runtime, well up to you
    pub fn spawnmodel(&mut self,modelfilepath:&str) -> (Vec<Vec<blue_engine::Vertex>> , Vec<Vec<u16>>,Vec<String>){
        // assuming you have the model loaded by buildmode() if not it will do it in runtime
        let mut getparts = self.getmodelparts(modelfilepath);
        if getparts.len() < 1 {
            self.buildmodelfromfile(modelfilepath);
            getparts = self.getmodelparts(modelfilepath);
        }
        let mut vertices: Vec<Vec<blue_engine::Vertex>> = Vec::new();
        let mut indices: Vec<Vec<u16>> = Vec::new();
        let mut counter = 0;
        for part in getparts{
            vertices.push(Vec::new());
            vertices[counter] = self.getmodelvertices(&part).clone();
            indices.push(Vec::new());
            indices[counter] = self.getmodelindices(&part).clone();

            counter +=1;
        }
        let textures = self.getmodeltextures(&modelfilepath);
        (vertices,indices,textures)
    }
    /// build a model to the structs cache from a filepath,
    /// required for spawning models files into the world.
    pub fn buildmodelfromfile(&mut self,modelfilepath:&str) -> String{
        let filedata = Nfile::read(modelfilepath);
        self.buildmodel(&modelfilepath, &filedata)
    }
    // pub fn deletemodel(object:&str,vmap:&mut Varmap){
    //
    //     let objectparts = object.to_owned() + ".parts";
    //     if objectparts != "" && objectparts != ".parts"{
    //         let part = "delete,".to_string() + &object + ",";
    //         vmap.pushstaticarray(BNC_DELETE_Q,part.to_string());
    //         //vmap.setvar("blueengine.deletion_q".to_owned(),&deletionq);
    //         vmap.delobj(object);
    //     }
    // }
    /// build model will build the model from a file and store it into the struct.
    /// you can use spawnmod
    pub fn buildmodel(&mut self,modelfilepath:&str,filedata:&str) -> String{
        // storage buffers
        // hashmap used to identify buffers with as meshesh can be mixed
        let mut id_map: HashMap<String,usize> = HashMap::new();
        let mut all_ids:Vec<String> = Vec::new();
        let mut all_textures:Vec<String> = Vec::new();
        let mut id_counter: usize = 0;
        self.objects_rawmodeldata.insert(modelfilepath.to_string(), filedata.to_string());
        //let mut part_i : usize = 0;
        let mut subpartscontainer: Vec<std::vec::Vec<String>> = Vec::new();

        //fileparser: into subarrays to feed the partloader
        let filedatasplit:Vec<&str> =  filedata.split("\n").collect();
        for parts in filedatasplit{//.split("\n").collect(){
            let quadparts :Vec<String> = parts.split(",").map(|s| s.to_string()).collect();
            if quadparts.len() > 10{

                // For each texture used in the mode we create a submesh
                // // this sorts them into the arrys dynamicly, none creates a new vec some pushes
                // if exists.
                let subnode = modelfilepath.to_string() + "__" + &quadparts[0];
                let getid = id_map.get_key_value(&subnode);
                match getid {
                    None => {
                        //create new id to tag storage vecs with
                        let id = id_counter.clone();
                        id_counter +=1;
                        id_map.insert(subnode.to_string(), id.clone());
                        all_ids.push(subnode.to_string());
                        all_textures.push(quadparts[0].to_string());
                        subpartscontainer.push(Vec::new());
                        subpartscontainer[id].push(parts.to_string());
                    }
                    Some((_i, res)) =>{
                        let id = res.to_owned();
                        subpartscontainer[id].push(parts.to_string());
                    }
                }
            }
        }

        // add the loaded model data to the struct so next load it can return these faster.
        let mut idcounter = 0;
        for submodel in all_ids{
            let (partvertices,partindices) = self.loadmodelpart(&submodel, &subpartscontainer[idcounter]);
            //println!("Loaded part{} vertices:{}",&submodel,&partindices.len());
            let mut getparts = self.getmodelparts(modelfilepath);
            getparts.push(submodel.clone());
            self.objects_partmap.insert(modelfilepath.to_string(), getparts);
            self.objects_vertices.insert(submodel.to_string(),partvertices);
            self.objects_indices.insert(submodel.to_string(),partindices);
            idcounter +=1;
        }
        self.objects_texturemap.insert(modelfilepath.to_string(),all_textures);
        modelfilepath.to_string()
    }

    /// this loads the final objects sub part, for each texture used on the model 1 subpart object will
    /// be created
     fn loadmodelpart(&mut self, _objectname: &str, dataarray: &Vec<String>) -> (std::vec::Vec<blue_engine::Vertex>, Vec<u16>) {
        let mut newindices: Vec<u16> = Vec::new();
        let mut vertex_offset = 0;  // Track the total number of vertices processed
        let mut newvertex: std::vec::Vec<blue_engine::Vertex> = Vec::new();

        for entrees in dataarray {
            let modelpartsdata: Vec<&str> = entrees.split(",").collect();
            if modelpartsdata.len() > 10 {
                // Set default indices and vertices used for shaping
                let indices = match modelpartsdata[10] {
                    "triangle" => self.indices_triangle.clone(),
                    "square" => self.indices_square.clone(),
                    "cube" => self.indices_cube.clone(),
                    _ => self.indices_square.clone(),
                };
                let vertices = match modelpartsdata[10] {
                    "triangle" => self.vertices_triangle.clone(),
                    "square" => self.vertices_square.clone(),
                    "cube" => self.vertices_cube.clone(),
                    _ => self.vertices_square.clone(),
                };

                let position = [
                    self.f32(&modelpartsdata[1]),
                    self.f32(&modelpartsdata[2]),
                    self.f32(&modelpartsdata[3]),
                ];
                let rotation = [
                    self.f32(&modelpartsdata[4]),
                    self.f32(&modelpartsdata[5]),
                    self.f32(&modelpartsdata[6]),
                ];
                let scale = [
                    self.f32(&modelpartsdata[7]),
                    self.f32(&modelpartsdata[8]),
                    self.f32(&modelpartsdata[9]),
                ];

                // Calculate transformed vertices and indices
                let (verti, indi) = BatchedModels::calculate_transformed_vertices(
                    vertices.clone(),
                    indices.clone(),
                    position,
                    rotation,
                    scale,
                );

                // Add vertices to the return buffer
                newvertex.extend(verti);

                // Add indices to the return buffer with correct offset
                for index in indi {
                    newindices.push(index + vertex_offset);
                }

                // Update the vertex offset for the next model part
                vertex_offset += vertices.len() as u16;
            }
        }

        (newvertex, newindices)
    }
    fn f32(&mut self,intasstr: &str) -> f32 {
        let onerr: f32 = 0.0;
        match intasstr.parse::<f32>() {
            Ok(n) => return n,
            Err(_) => return onerr,
        }
    }

    fn calculate_transformed_vertices(
        vertices: std::vec::Vec<blue_engine::Vertex>,
        indices: Vec<u16>,
        position: [f32; 3],
        rotation: [f32; 3], // [pitch, yaw, roll]
        scale: [f32; 3],
    ) -> (std::vec::Vec<blue_engine::Vertex>, Vec<u16>) {
        let position_vec = Vector3::new(position[0], position[1], position[2]);
        let scale_vec = Vector3::new(scale[0], scale[1], scale[2]);

        // Convert Euler angles from degrees to radians and create a quaternion
        let euler_rotation = Euler {
            x: Deg(rotation[0]), // pitch (X)
            y: Deg(rotation[1]), // yaw (Y)
            z: Deg(rotation[2]), // roll (Z)
        };
        let rotation_quaternion: Quaternion<f32> = Quaternion::from(euler_rotation);

        // Create transformation matrices
        let translation_matrix = Matrix4::from_translation(position_vec);
        let rotation_matrix = Matrix4::from(rotation_quaternion);
        let scale_matrix = Matrix4::from_nonuniform_scale(scale_vec.x, scale_vec.y, scale_vec.z);

        // Combine into a single transformation matrix in the correct order: Scale -> Rotate -> Translate
        let transformation_matrix = translation_matrix * rotation_matrix * scale_matrix;

        // Transform all vertices by applying the transformation matrix
        let transformed_vertices: Vec<Vertex> = vertices
            .into_iter()
            .map(|vertex| {
                let position_vec = Point3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
                let transformed_position = transformation_matrix.transform_point(position_vec);

                Vertex {
                    position: [transformed_position.x, transformed_position.y, transformed_position.z],
                    uv: vertex.uv,
                    normal: vertex.normal,
                }
            })
            .collect();

        // Return the transformed vertices and the same indices
        (transformed_vertices, indices)
    }


}
