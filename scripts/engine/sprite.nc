class sprite{
    func load(dir){
        loadsprite(dir)
        *dir.loaded = 1
    }
    func construct(){
        self.test = "ok"
        //print(self) !
    }
    func spawn(obj,fromspritedir){
        obj = square(obj,0.0,3.0,0.0)
        objectsetscale(obj,0.5,1.0,0.5)
        return self.add(obj,fromspritedir)
    }
    func add(obj,fromspritedir){
        *obj : sprite
        *obj : *fromspritedir
        //object::toobject(fromspritedir,obj)//obj : *fromspritedir
        addsprite(obj)
        //spritesetanimation(obj,animationstring)
        print(obj," into ",fromspritedir," test=",*obj.test,"pink")
        obj
    }
    func animate(animationstring){
        if self.currentanim != animationstring{
            self.currentanim = animationstring
            spritesetanimation(self,animationstring)
        }

    }
    func setposition(x,y,z){
        self.x = x
        self.y = y
        self.z = z
        objectsetposition(self,x,y,z)
        aabb_setposition(self,x,y,z)

    }
    func setrotation(x,y,z){
        objectsetrotation(self,x,y,z)
    }
    func setscale(x,y,z){
        objectsetscale(self,x,y,z)
    }
    func delete(){
        removesprite(self)
        objectremove(self)
    }
    self.currentanim = "none"
    self.test = "nope"
    self.idletimer = timerinit()
    self.animtimer = timerinit()
    for loadsprite in listdir("assets"){
        file = cat("assets/",loadsprite,"/sprite.cfg")
        if fileexists(file) == true {
            sprite.load(cat("assets",loadsprite))
        }
    }
}