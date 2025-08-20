use blue_engine::{ CameraContainer};

use crate::*;

pub struct BluencCamera{
    pub cameras: Vec<Vec<f32>>,
    pub activecamera:usize,
}
impl BluencCamera{
    pub fn new() -> BluencCamera{
        let mut this = BluencCamera{
            cameras: Vec::new(),
            activecamera:0,
        };

        this.cameras.push(vec!(
            0.0,0.0,0.0,//pos
            0.0,0.0,0.0,//pitch yaw roll
            0.0,0.0,0.0//target
        ));
        this
    }
    pub fn addcamera(&mut self) -> usize{
        self.cameras.push(vec!(
            0.0,0.0,0.0,//pos
            0.0,0.0,0.0,//pitch yaw roll
            0.0,0.0,0.0//target
        ));
        self.cameras.len()-1
    }
    pub fn setactive(&mut self,set:usize){
        self.activecamera = set;
    }
    pub fn queehandler(camera:&mut CameraContainer,storage:&mut NscriptStorage){
        //let mut block = NscriptCodeBlock::new("");


        if storage.customdata.static_vec_bool[Q_CAMERA]{
            let x = Nstring::f32(&storage.objectgetprop("camera","x").stringdata);
            let y = Nstring::f32(&storage.objectgetprop("camera","y").stringdata);
            let z = Nstring::f32(&storage.objectgetprop("camera","z").stringdata);
            let tx = Nstring::f32(&storage.objectgetprop("camera","targetx").stringdata);
            let ty = Nstring::f32(&storage.objectgetprop("camera","targety").stringdata);
            let tz = Nstring::f32(&storage.objectgetprop("camera","targetz").stringdata);
            camera
                .set_position(Vector3::new(x, y, z));
            camera.set_target(Vector3::new(tx,ty,tz));
            storage.customdata.static_vec_bool[Q_CAMERA] = false;
        }
            //.expect("Couldn't update the camera eye");
    }
    // pub fn targetbydistance(distance: f32,storage:&mut NscriptStorage) {
    //     // Convert degrees to radians
    //     let pitch =Nstring::f32(&storage.getvar("camera.pitch").stringdata);
    //     let yaw = nscript_f32(&vmap.getvar("camera.yaw"));
    //     let pitch = pitch.to_radians();
    //     let yaw = yaw.to_radians();
    //
    //     // Step 1: Calculate the direction vector based on pitch and yaw
    //     let direction_x = pitch.cos() * yaw.sin();
    //     let direction_y = pitch.sin();
    //     let direction_z = pitch.cos() * yaw.cos();
    //
    //     // Step 2: Normalize the direction vector (not strictly necessary if you just want to scale it by distance)
    //     let magnitude = (direction_x.powi(2) + direction_y.powi(2) + direction_z.powi(2)).sqrt();
    //     let direction_x = direction_x / magnitude;
    //     let direction_y = direction_y / magnitude;
    //     let direction_z = direction_z / magnitude;
    //
    //     // Step 3: Scale the direction by the desired distance
    //     let scaled_direction_x = direction_x * distance;
    //     let scaled_direction_y = direction_y * distance;
    //     let scaled_direction_z = direction_z * distance;
    //
    //     // Assuming the camera has a position (cam_x, cam_y, cam_z)
    //     let cam_x = nscript_f32(&vmap.getvar("camera.x"));
    //     let cam_y = nscript_f32(&vmap.getvar("camera.y"));
    //     let cam_z = nscript_f32(&vmap.getvar("camera.z"));
    //     // Step 4: Calculate the target position
    //     let target_x =  cam_x + scaled_direction_x;
    //     let target_y = cam_y  + scaled_direction_y;
    //     let target_z = cam_z + scaled_direction_z;
    //
    //     // Step 5: Set the new target further away along the direction vector
    //
    //     Bluenc::camerasettargetposition(&(target_x as f64), &(target_y as f64), &(target_z as f64),vmap);
    // }
}
