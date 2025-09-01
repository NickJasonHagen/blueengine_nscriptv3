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
    pub fn camerasetposition(storage:&mut NscriptStorage){
        // vmap.setvar("camera.x".to_owned(), &x.to_string());
        // vmap.setvar("camera.y".to_owned(), &y.to_string());
        // vmap.setvar("camera.z".to_owned(), &z.to_string());
        // vmap.setvar("camera.targetx".to_owned(), &targetx.to_string());
        // vmap.setvar("camera.targety".to_owned(), &targety.to_string());
        // vmap.setvar("camera.targetz".to_owned(), &targetz.to_string());
        let pitch = Nstring::f32(&storage.objectgetprop("camera","pitch").stringdata);
        let yaw = Nstring::f32(&storage.objectgetprop("camera","pitch").stringdata);
        //BluencCamera::camerasetrotation(pitch, yaw, storage);
    }
    pub fn camerasettargetposition(targetx:f32,targety:f32,targetz:f32,storage:&mut NscriptStorage){
       storage.objectsetprop("camera","targetx", NscriptVar::newstring("targetx", targetx.to_string()));
        storage.objectsetprop("camera","targety", NscriptVar::newstring("targety", targety.to_string()));
        storage.objectsetprop("camera","targetz", NscriptVar::newstring("targetz", targetz.to_string()));
        BluencCamera::cameraupdate(storage)
    }
    pub fn calculate_pitch_yaw(
        cam_x: f32, cam_y: f32, cam_z: f32,
        target_x: f32, target_y: f32, target_z: f32
    ) -> (f32, f32) {
        // Step 1: Calculate direction vector from camera to target
        let dir_x = target_x - cam_x;
        let dir_y = target_y - cam_y;
        let dir_z = target_z - cam_z;

        // Step 2: Calculate yaw (rotation around the y-axis, horizontal rotation)
        let mut yaw = dir_z.atan2(dir_x).to_degrees();  // atan2 gives the angle in radians

        // Step 3: Normalize yaw to be within 0.0 to 360.0 degrees
        if yaw < 0.0 {
            yaw += 360.0;
        }

        // Step 4: Calculate pitch (rotation around the x-axis, vertical rotation)
        let horizontal_distance = (dir_x.powi(2) + dir_z.powi(2)).sqrt();  // Hypotenuse of x and z
        let pitch = dir_y.atan2(horizontal_distance).to_degrees();

        // Return the pitch and normalized yaw
        (pitch, yaw)
    }
    pub fn camera_targetbydistance(distance: f32,storage:&mut NscriptStorage) {
        // Convert degrees to radians
        let pitch = Nstring::f32(&storage.objectgetprop("camera","pitch").stringdata);
        let yaw = Nstring::f32(&storage.objectgetprop("camera","yaw").stringdata);
        let pitch = pitch.to_radians();
        let yaw = yaw.to_radians();

        // Step 1: Calculate the direction vector based on pitch and yaw
        let direction_x = pitch.cos() * yaw.sin();
        let direction_y = pitch.sin();
        let direction_z = pitch.cos() * yaw.cos();

        // Step 2: Normalize the direction vector (not strictly necessary if you just want to scale it by distance)
        let magnitude = (direction_x.powi(2) + direction_y.powi(2) + direction_z.powi(2)).sqrt();
        let direction_x = direction_x / magnitude;
        let direction_y = direction_y / magnitude;
        let direction_z = direction_z / magnitude;

        // Step 3: Scale the direction by the desired distance
        let scaled_direction_x = direction_x * distance;
        let scaled_direction_y = direction_y * distance;
        let scaled_direction_z = direction_z * distance;

        // Assuming the camera has a position (cam_x, cam_y, cam_z)
        let cam_x =Nstring::f32(&storage.objectgetprop("camera","x").stringdata);
        let cam_y = Nstring::f32(&storage.objectgetprop("camera","y").stringdata);
        let cam_z =Nstring::f32(&storage.objectgetprop("camera","z").stringdata);
        // Step 4: Calculate the target position
        let target_x =  cam_x + scaled_direction_x;
        let target_y = cam_y  + scaled_direction_y;
        let target_z = cam_z + scaled_direction_z;

        // Step 5: Set the new target further away along the direction vector

        BluencCamera::camerasettargetposition(target_x, target_y, target_z,storage);
    }
    pub fn camerasetrotation(pitch: f32, yaw: f32, storage:&mut NscriptStorage) {
        // pitch = vertical axis
        // yaw = horizontal axis
        // Convert degrees to radians

        let mut newpitch = pitch;
        if newpitch < -70.0 {
            newpitch = -70.0;
        }
        if newpitch > 70.0 {
            newpitch = 70.0;
        }
        let mut newyaw = yaw;
        if newyaw < 0.0 {
            newyaw = newyaw + 360.0;
        }
        if newyaw > 360.0 {
            newyaw = newyaw - 360.0;
        }
        storage.objectsetprop("camera","pitch",NscriptVar::newstring("pitch", newpitch.to_string()));
        storage.objectsetprop("camera","yaw",NscriptVar::newstring("yaw", newyaw.to_string()));
        let pitch = newpitch.to_radians();
        let yaw = newyaw.to_radians();

        // Calculate the direction vector based on yaw and pitch
        let direction_x = pitch.cos() * yaw.sin();
        let direction_y = pitch.sin();
        let direction_z = pitch.cos() * yaw.cos();

        // Assuming the camera has a position (cam_x, cam_y, cam_z)
        let cam_x = Nstring::f32(&storage.objectgetprop("camera","x").stringdata);
        let cam_y = Nstring::f32(&storage.objectgetprop("camera","y").stringdata);
        let cam_z = Nstring::f32(&storage.objectgetprop("camera","z").stringdata);        // Set the target relative to the camera's position
        let target_x = cam_x+ direction_x;
        let target_y = cam_y  + direction_y;
        let target_z = cam_z  + direction_z;
        //let newtarget = BluencCamera::camera_gettarget(pitch, yaw, 200.0, (cam_x,cam_y,cam_z));
        storage.objectsetprop("camera","targetx",NscriptVar::newstring("targetx", target_x.to_string()));
        storage.objectsetprop("camera","targety",NscriptVar::newstring("targety", target_y.to_string()));
        storage.objectsetprop("camera","targetz",NscriptVar::newstring("targetz", target_z.to_string()));
        // Update the camera's target position
        //BluencCamera::camerasettargetposition(target_x, target_y,target_z,storage);

    }

    pub fn camerastrafe(side:&str,distance:f32,storage:&mut NscriptStorage){
        // forward / back / left / right are mapped in bindings
        let pitch = Nstring::f32(&storage.objectgetprop("camera", "pitch").stringdata);
        let yaw = Nstring::f32(&storage.objectgetprop("camera", "yaw").stringdata);
        let newpos = BluencCamera::get_camera_strafe_targets(side,distance,pitch,yaw,storage);
        storage.objectsetprop("camera","x",NscriptVar::newstring("x", newpos.0.to_string()));
        storage.objectsetprop("camera","y",NscriptVar::newstring("y", newpos.1.to_string()));
        storage.objectsetprop("camera","z",NscriptVar::newstring("z", newpos.2.to_string()));

        let newtarget = BluencCamera::camera_gettarget(pitch, yaw, distance*2.0, newpos);
        storage.objectsetprop("camera","targetx",NscriptVar::newstring("targetx", newtarget.0.to_string()));
        storage.objectsetprop("camera","targety",NscriptVar::newstring("targety", newtarget.1.to_string()));
        storage.objectsetprop("camera","targetz",NscriptVar::newstring("targetz", newtarget.2.to_string()));
        //BluencCamera::camerasetposition(storage);
    }
    pub fn camera_gettarget(pitch: f32, yaw: f32, distance: f32, cam_position: (f32, f32, f32)) -> (f32, f32, f32) {
        let pitch = pitch.to_radians();
        let yaw = yaw.to_radians();

        let direction_x = pitch.cos() * yaw.sin();
        let direction_y = pitch.sin();
        let direction_z = pitch.cos() * yaw.cos();

        let target_x = cam_position.0 + direction_x * distance;
        let target_y = cam_position.1 + direction_y * distance;
        let target_z = cam_position.2 + direction_z * distance;

        (target_x, target_y, target_z)
    }
    pub fn get_camera_strafe_targets(
        towards:&str, distance: f32,pitch:f32,yaw:f32,storage:&mut NscriptStorage,
    ) -> (f32, f32, f32) {
        // Helper function to calculate target position based on pitch, yaw, and distance
        fn calculate_target(pitch: f32, yaw: f32, distance: f32, cam_position: (f32, f32, f32)) -> (f32, f32, f32) {
            let pitch = pitch.to_radians();
            let yaw = yaw.to_radians();

            let direction_x = pitch.cos() * yaw.sin();
            let direction_y = pitch.sin();
            let direction_z = pitch.cos() * yaw.cos();

            let target_x = cam_position.0 + direction_x * distance;
            let target_y = cam_position.1 + direction_y * distance;
            let target_z = cam_position.2 + direction_z * distance;

            (target_x, target_y, target_z)
        }
        //let pitch =Nstring::f32(&storage.objectgetprop("camera","pitch").stringdata);
        //let yaw = Nstring::f32(&storage.objectgetprop("camera","yaw").stringdata);
        let cam_position = (Nstring::f32(&storage.objectgetprop("camera","x").stringdata),Nstring::f32(&storage.objectgetprop("camera","y").stringdata),Nstring::f32(&storage.objectgetprop("camera","z").stringdata));

        match towards{

            "left" => {
                return calculate_target(0.0, yaw + 90.0, distance, cam_position);
            }
            "right" => {
                return calculate_target(0.0, yaw - 90.0, distance, cam_position);
            }
            "back" => {
                return calculate_target(0.0, yaw + 180.0, distance, cam_position);
            }
            "forward" =>{
                return calculate_target(0.0, yaw, distance, cam_position);
            }
            "up" =>{
                return calculate_target(90.0, yaw, distance, cam_position);
            }
            "down" =>{
                return calculate_target(-90.0, yaw, distance, cam_position);
            }
            "fly" =>{
                return calculate_target(pitch, yaw, distance, cam_position);
            }
            _ => {
                print!("error on the strafe part, no match for towards={}", &towards);
                return calculate_target(pitch, yaw, distance, cam_position);
            }
        }

}
    pub fn cameraupdate(_storage:&mut NscriptStorage){
        // let x =Nstring::f32(&storage.objectgetprop("camera","x").stringdata);
        // let y = Nstring::f32(&storage.objectgetprop("camera","y").stringdata);
        // let z =Nstring::f32(&storage.objectgetprop("camera","z").stringdata);
        // let targetx =Nstring::f32(&storage.objectgetprop("camera","x").stringdata);
        // let targety = Nstring::f32(&storage.objectgetprop("camera","y").stringdata);
        // let targetz =Nstring::f32(&storage.objectgetprop("camera","z").stringdata);
        // let toset = "".to_owned() + &x.to_string() + "," + &y.to_string() + "," +  &z.to_string() + "," + &targetx.to_string() + "," + &targety.to_string() + "," +  &targetz.to_string();
        //vmap.setvar("blueengine.camera_q".to_owned(), &toset);
        //vmap.pushstaticarray(BNC_CAMERA_Q, toset);
        //BluencCamera::skyboxmove(vmap);
    }
}


pub fn nscriptfn_camerasetrotation(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let pitch = storage.getargstring(&args[0], block);
    let yaw = storage.getargstring(&args[1], block);
    storage.customdata.static_vec_bool[Q_CAMERA] = true;
    BluencCamera::camerasetrotation(Nstring::f32(&pitch), Nstring::f32(&yaw), storage);
    NscriptVar::new("v")
}
pub fn nscriptfn_camerastrafe(args:&Vec<&str>,block:&mut NscriptCodeBlock,storage:&mut NscriptStorage) -> NscriptVar{
    let side = storage.getargstring(&args[0], block);
    let range = storage.getargstring(&args[1], block);
    storage.customdata.static_vec_bool[Q_CAMERA] = true;
    BluencCamera::camerastrafe(&side, Nstring::f32(&range), storage);
    NscriptVar::new("v")
}
