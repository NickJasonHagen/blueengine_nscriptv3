use crate::*;
use gilrs::{Button, Event, GamepadId, Gilrs};

use enigo::{
    Coordinate,
    //Direction::{Click, Press, Release},
    Enigo, Mouse, Settings,
};
pub struct BlueNcMouse {
    enigo: Enigo
}
impl BlueNcMouse{
    pub fn new()->BlueNcMouse{
        BlueNcMouse{
            enigo:Enigo::new(&Settings::default()).unwrap()
        }
    }
}
impl NscriptStructBinding for BlueNcMouse {
    fn nscript_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>, _storage :&mut NscriptStorage) -> NscriptVar{
        let mut retvar = NscriptVar::new("nothing");
        match tocall{
            // in nscript can be called as mystruct::helloworld()
            "getpos" => {
                let pos = self.enigo.location().unwrap_or((0,0));
                //vec!(pos.0.to_string().into(),pos.1.to_string().into())//ncpos;
                retvar.stringvec =  vec!(pos.0.to_string().into(),pos.1.to_string().into());//.push("vars also are vectors".to_string());
            }
            // print the given argument's stringdata!
            "setpos" => {
                let _ = self.enigo.move_mouse(Nstring::i32(&args[0].stringdata), Nstring::i32(&args[1].stringdata), Coordinate::Abs);
                retvar.stringdata = "ok".to_string();
            }
            // handle unknown calls if you like
            _ =>{
                print(&format!("Ohh cant find the call in mouse::{}",&tocall),"g");
            }
        }
        return  retvar;
    }
}

pub struct BlueNcGamePad{
    gilrs: Gilrs,
    activegamepad:Vec<Option<GamepadId>>,

}

impl BlueNcGamePad{
    pub fn new()->BlueNcGamePad{
        BlueNcGamePad { gilrs:Gilrs::new().unwrap() ,activegamepad:Vec::new()}
    }
}
impl NscriptStructBinding for BlueNcGamePad {
    fn nscript_exec(&mut self,tocall:&str,args: &Vec<NscriptVar>, storage :&mut NscriptStorage) -> NscriptVar{
        let  retvar = NscriptVar::new("nothing");
        match tocall{
            // in nscript can be called as mystruct::helloworld()
            "checkdevice" => {
                self.activegamepad = Vec::new();
                let mut keyeventbool = NscriptVar::new("gamepad");
                let mut block = NscriptCodeBlock::new("_");
                while let Some(Event { id, event,.. }) = self.gilrs.next_event() {
                    let getinputs = format!("{}: {:?}",  id, event);
                    let isid = Nstring::prefix(&getinputs);
                    let isbutton = Nstring::stringbetween(&getinputs,"(",", Code(");
                    let isevents = split(&isbutton,", ");
                    if Nstring::instring(&getinputs, "Pressed"){
                        keyeventbool.stringdata = "true".to_string();
                        storage.setprop(&format!("gamepad{}",&isid), &isevents[0].trim().to_lowercase(), NscriptVar::newstring("k","1.0".to_string()), &mut block);
                    }else{
                        if isevents.len() > 1 {
                            keyeventbool.stringdata = "true".to_string();
                            storage.setprop(&format!("gamepad{}",&isid), &isevents[0].trim().to_lowercase(), NscriptVar::newstring("k",isevents[1].trim().to_string()), &mut block);
                            self.activegamepad.push(Some(id)) ;
                            //print(&getinputs,"g");
                            //print(&isevents[0].trim().to_lowercase(),"pink");
                            //print(&isevents[1].trim(),"grey");
                        }
                        // else{
                        //     print(&getinputs,"r");
                        // }
                    }
                }
                storage.setprop("gamepad", "event", keyeventbool, &mut block);
            }
            // print the given argument's stringdata!
            "checkinput" => {
                let thispadid = Nstring::usize(&args[0].stringdata);
                if let Some(gamepad) = self.activegamepad[thispadid].map(|id| self.gilrs.gamepad(id)) {
                    let keyvec = vec!(Button::South,Button::West,Button::North,Button::East,Button::DPadUp,Button::DPadLeft,Button::DPadDown,Button::DPadRight);
                    let namevec = vec!("south","west","north","east","dpadup","dpadleft","dpaddown","dpadright");
                    let mut i = 0;
                    let mut block = NscriptCodeBlock::new("_");
                    for xbutton in keyvec{
                        if gamepad.is_pressed(xbutton) {
                            storage.setprop("gamepad", &namevec[i], NscriptVar::newvar("k","down".to_string(),Vec::new()),&mut block );
                        }
                        else{
                            storage.setprop("gamepad", &namevec[i], NscriptVar::newvar("k","up".to_string(),Vec::new()),&mut block );
                        }
                        i +=1;
                    }
                    // if gamepad.is_pressed(Button::South) {
                    //     println!("Button South is pressed (XBox - A, PS - X)");
                    // }
                }
            }
            // handle unknown calls if you like
            _ =>{
                print(&format!("Ohh cant find the call in mouse::{}",&tocall),"g");
            }
        }
        return  retvar;
    }
}
