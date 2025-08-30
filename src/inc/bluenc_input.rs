use crate::*;

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
/// A example struct impl for nscript
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

