
class textures{
    for xtile in listdir("assets/tiles",true){
        print("loaded texture",loadtexture(xtile),"by")
    }
}
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
}
class camera{
    self.x = 3.0
    self.y = 4.0
    self.z = 6.0
    self.offy = 2.0

    self.targetx = 3.0
    self.targety = 1.5
    self.targetz = 0.0
}
class tiles{

}
class maptiles{
    func colgroup(x,z){
        return cat(splitselect(x,".",0),"_",splitselect(z,".",0))
    }
    func spawn(xtile,texture,posx,posy,posz,coligroup){
        *xtile.x = posx
        *xtile.y = posy
        *xtile.z = posz
        cube(xtile,posx,posy,posz)
        objectsetscale(xtile,0.5,0.5,0.5)
        settexture(xtile,texture)  
        aabb_newbox(xtile)
        aabb_setscale(xtile,0.5,0.5,0.5)
        aabb_setposition(xtile,posx,posy,posz)
        aabb_addtogroup(xtile,"maptiles")
        

        
    }
    func collisioncheck(collisionid,x,y,z){
        colgroup = maptiles.colgroup(x,z)
        x2 = *collisionid.x
        y2 = *collisionid.y
        z2 = *collisionid.z
        y -= 0.25
        aabb_setposition(collisionid,x,y,z)
        col = arraymerge(aabb_getcollisions(collisionid,colgroup),aabb_getcollisions(collisionid,"maptiles"))
        if len(col) > 0{
            return true
        }
            y -= 0.37
            aabb_setposition(collisionid,x,y,z)

            col = arraymerge(aabb_getcollisions(collisionid,colgroup),aabb_getcollisions(collisionid,"maptiles"))
            if len(col) > 0{
                y = player.y + 0.12
                //player.setposition(x2,y,z2)
                //mysprite.x = x2
                mysprite.y = y
                //mysprite.z = z2
            }
            else{
                aabb_setposition(collisioncheck,x2,y2,z2)
            }

        return false
    }
    
}
func argb(argbcolor){
    replacebyref(argbcolor,"rgb(","")
    replacebyref(argbcolor,")",",,,")
    replacebyref(argbcolor," ","")
    rgbvec = split(argbcolor,",")
    // rgbvec[0] = 0
    // rgbvec[1] = trim(rgbvec[0])
    // rgbvec[2] = trim(rgbvec[1])
    // rgbvec[3] = trim(rgbvec[2])
    rgbvec
}
class gravity{
    func construct(){
        this = self
        coroutine this each 25{
            *this.pull()
        }
    }
    func pull(){
        colgroup = maptiles.colgroup(player.x,player.z)
        if self.gravitymax > self.gravitycurrentpull{
            self.gravitycurrentpull += self.gravitypullstrenght 
        }
        if self.gravitycurrentpull < 0.0 {
            newy = self.y //+ 1.0
            self.gravityside = "up"
            self.animate(cat("anim_jump",mysprite.side))
            
        
        }
        else{
            self.gravityside = "down"
            newy = self.y - 0.6
        }
        
        aabb_setposition(self,self.x,newy,self.z)  
        col = arraymerge(aabb_getcollisions(self,colgroup),aabb_getcollisions(self,"maptiles"))
        if len(col) > 0{
            //col !
            self.ontile = true
            self.ontileid = col[0]
            self.gravitycurrentpull = -0.0
            aabb_setposition(self,self.x,self.y,self.z)
        }
        else{
            self.ontileid = ""
            self.ontile = false
            if self.gravitycurrentpull > 0.05 {
                self.animate(cat("anim_fall",mysprite.side))
            }
            self.setposition(self.x,subtract(self.y,self.gravitycurrentpull),self.z)
        }
        //self.gravitycurrentpull !
        //"ok"

    }
    self.gravitymax = 0.45
    self.gravitycurrentpull = 0.0
    self.gravitypullstrenght = 0.033
}
class player : gravity{
    func animate(anim){
        mysprite.animate(anim)
    }
    func setposition(x,y,z){
        camy = y + camera.offy
        setcamerapos("camera",x,camy,camera.z,x,y,z)
        //setcamerapos("camera",camera.x,camera.y,camera.z,camera.targetx,0.0,camera.targetz)
        //objectsetposition("mysprite",camera.x,1.5,0.0)
        self.x = x
        self.y = y
        self.z = z
        *$mysprite.setposition(x,y,z)
        aabb_setposition(self,x,y,z)
        $stack = arrayroll($stack,cat(*$mysprite.currentanim,",",x,",",y,",",z))
         split = split($stack[9],",")
         //
         draakje.animate(split[0])
         petx = split[1] - 0.25
         pety = split[2] - 0.2
         petz = split[3] - 0.1
         
         draakje.setposition(petx,pety,petz)

    }
    func jump(){
        if self.gravityside == "down" && self.ontile == true{
            self.gravitycurrentpull = -0.3
        }

    }
    self.aabb = aabb_newbox(self)
    aabb_setscale(self.aabb,0.5,0.3,0.5)
    func init(){
        $mysprite = "mysprite"
        $stack = arraynew()
        draakje = "draakje"
        for x to 10{
            $stack = arraypush($stack,"anim_right,0.0,0.0,0.0")
        }
        base = cat(@scriptdir,"assets/oogbal")
        sprite.load(base)
        draakje : sprite
        draakje = sprite.spawn("draakje",base)
        draakje.animate("anim_right").setscale(0.8,0.7,0.6)
    }
    self.x = 3.0
    self.y = 3.0
    self.z = 3.0
    self.jumping = false
    self.init()
}
print("starting nc")



speed = 0.15
controllertimer = timerinit()
spritebase = cat(@scriptdir,"assets/meisje")

sprite.load(spritebase)
//sprite.load(spritebase)
mysprite = "mysprite"
*mysprite : sprite
mysprite = sprite.spawn("mysprite",spritebase)
*mysprite.animate("anim_right").setposition(player.x,player.y,player.z).setscale(1.0,0.8,1.0)
spritebase = cat(@scriptdir,"assets/meisje")
sprite.load(spritebase)

// oi = "oi"
// oi = sprite.spawn("oi",spritebase)
// coroutine "oi" each 20{
//     *oi.animate("anim_down").setposition(4.0,2.0,0.0)
//     break self
// }
i = 0
z = 0.0
for x to 1{
    i ++
    if i > 9 {
        z -= 10.0
        i = 0
    }
    s = cat("sprite",x)
    *s : *spritebase
    s = sprite.spawn(s,spritebase)
    p = i * 1.2
    *s.ranvec = [
        "anim_left",
        "anim_right",
        "anim_up",
        "anim_down"
        ] 
//*s.ranvec !
    //*s.animate("anim_left").setposition(p,2.0,z)
    *s.animate("anim_left").setposition(random(-15.1,15,4),2.0,random(-25.1,25.1,4))//.setscale(0.5,0.3,0.5)
    coroutine *s each random(100,150,0){
        r = random(0,3,0)
        *s.animate(*s.ranvec[r])
        *s.x += 0.1
        *s.setposition(*s.x,*s.y,*s.z)
        //break self
    }
}

// spritebase = loadsprite(spritebase)


// mysprite = "mysprite"
// mysprite = square(mysprite,0.0,3.0,0.0)
// objectsetscale(mysprite,0.5,1.0,0.5)
// mysprite : *spritebase
// addsprite("mysprite")
// spritesetanimation("mysprite","anim_right")
class map{
    func createfile(){
        z = 1.0
        x = 1.0
        y = 1.6
        fdata = ""
        for xi to 15000{
            x ++
            model  = "./assets/tiles/grass.png"
            if x > 150 {
                z ++    
                x = 1.0
                if z < 15.0 {
                    //y -= 0.15
                }
                if z > 15.0 {
                    //y += 0.15
                }
                
            }
            if x < 2 || z < 2 {
                model  = "./assets/tiles/mud.png"
            }
            if x > 10 && x < 13 {
                model  = "./assets/tiles/street.png"
            }

            x = x * 1.0
            z = z * 1.0
            fdata = cat(fdata,model,",",x,",",y,",",z,",-90.0,0.0,0.0,0.5,0.5,0.5,square,,",@lf)
            xtile = cat("maptile_",xi)
            
            aabb_newbox(xtile)
            aabb_setscale(xtile,0.5,0.1,0.5)
            aabb_setposition(xtile,x,y,z)
            aabb_addtogroup(xtile,maptiles.colgroup(x,z))
        }
        filewrite("./assets/tmpmap",fdata)
    }

    loadtexture("./assets/tiles/mud.png")
    loadtexture("./assets/tiles/grass.png")
    loadtexture("./assets/tiles/street.png")
    
    map.createfile()
    coroutine "aft"{
    batchedmodel_modelspawn(
        "map",
        "assets/tmpmap"
        )
        break self
    }

}

// z = 1.0
// x = 1.0
// y = 1.6
// for xi to 100{
//     x ++
//     if x > 11 {
//         z ++    
//         x = 1.0
//         y -= 0.35
//     }
//     x = x * 1.0
//     z = z * 1.0
    
    
//     xtile = cat("tile",xi)
//     //cube(xtile,x,0.0,z)
//     //objectsetscale(xtile,0.5,0.5,0.5)
//     //objectsetrotation(xtile,90.0,0.0,0.0)
//     if z > 3.0 && z < 7.0 {
//         //settexture(xtile,cat(@scriptdir,"assets/tiles/street.png"))  
//         maptiles.spawn(xtile,cat(@scriptdir,"assets/tiles/street.png"),x,0.1,z)
//     }
//     else{
//         maptiles.spawn(xtile,cat(@scriptdir,"assets/tiles/grass.png"),x,y,z)
//         //settexture(xtile,cat(@scriptdir,"assets/tiles/grass.png"))       
//     }
//     //settexture(xtile,cat(@scriptdir,"assets/tiles/street.png"))
//     // xtile = cat("tiler2",x)
//     // square(xtile,x,-0.5,1.0)
//     // objectsetscale(xtile,0.5,0.5,0.5)
//     // objectsetrotation(xtile,-90.0,0.0,0.0)
//     // settexture(xtile,cat(@scriptdir,"assets/tiles/street.png"))
// }
        base = cat(@scriptdir,"assets/tree")
        sprite.load(base)
        for x to 1{
            xcoin = cat("tree_",x)
            sprite.spawn(xcoin,base)
            aabb_newbox(xcoin)
            aabb_setscale(xcoin,0.1,2.0,0.1)
            
            *xcoin.animate("anim_run").setposition(random(0.0,70.1),2.8,random(0.0,70.2)).setscale(2.0,2.0,2.0)
            aabb_setposition(xcoin,*xcoin.x,*xcoin.y,*xcoin.z)
            aabb_addtogroup(xcoin,"maptiles")

        }
        base = cat(@scriptdir,"assets/gem_green")
        sprite.load(base)
        for x to 1{
            xcoin = cat("coin_",x)
            sprite.spawn(xcoin,base)
            *xcoin.animate("anim_run").setposition(random(0.0,10.2),random(0.0,4.1),random(0.0,10.1)).setscale(0.4,0.4,0.25)

        }
        loadtexture("assets/tree/1.png")
//buffer = batchedmodel_buildfromfile("assets/models/tree")a
coroutine "pres"{
        for xtree to 800{
            xt = cat("xtree_",xtree)
            x = random(0.1,151.10,4)
            y = 1.7
            z = random(1.0,120.3,3)
            batchedmodel_spawntobuffer(
                "forrestmap",
                "assets/models/tree",
                x,y,z,
                0.0,0.0,0.0,
                1.0,random(0.8,1.1),1.0
            )

            
            //batchedmodel_setposition(xt,x,y,z)
            aabb_newbox(xt)
            aabb_setposition(xt,x,y,z)
            aabb_setscale(xt,0.25,3,0.25)
            aabb_addtogroup(xt,maptiles.colgroup(x,z))
            
        }
        batchedmodel_modelbuffertofile(
            "forrestmap",
            "./assets/forrestmap"
            )
            batchedmodel_modelspawn(
                "forrestmap",
                "./assets/forrestmap"
            )
        break self
}

        base = cat(@scriptdir,"assets/qbox")
        sprite.load(base)
for x to 10 {
    xtile = cat("ranblock",x)
    sprite.spawn(xtile,base)
    //*xtile.path = 0
    *xtile.pathretour = false
    *xtile.animate("anim_run")
    *xtile.path = random(-1.0,1.0)
    *xtile.beginx = random(1.51,105.0)
    maptiles.spawn(xtile,cat(@scriptdir,"assets/tiles/street.png"),*xtile.beginx,random(1.53,5.0),random(0.54,13.5))
    gain = 0.1
    coroutine xtile each 80{
        
        if *xtile.pathretour == true{
            *xtile.path -= gain
            gainx = 0.0 - gain
            if *xtile.path < -10.0{
                *xtile.pathretour = false
            }
        }
        else{
            *xtile.path += gain
            gainx = gain
            if *xtile.path > 10.0{
                *xtile.pathretour = true
            }
        }

        onx = *xtile.beginx + *xtile.path
        if player.ontileid = xtile{
            px = player.x + gainx
            player.setposition(px,player.y,player.z)    
        }
        *xtile.setposition(onx,*xtile.y,*xtile.z)
    }
    objectsetscale(xtile,2.0,1.0,2.0)
}
xtile = "background"
square(xtile,x,-35.0,-20.0)
objectsetscale(xtile,400.5,100.5,10.5)
loadtexture(cat(@scriptdir,"./assets/background.png"))
settexture(xtile,cat(@scriptdir,"./assets/background.png"))
print(*spritebase.anim_right,"by")

loadtexture("assets/tiles/mud.png")
loadtexture("assets/tiles/grass.png")

//batch = batchedmodel_buildfromfile("./assets/models/street")
batchedmodel_spawntobuffer(
    "testme",
    "assets/models/street",
    2.0,4.0,2.0,
    0.0,0.0,0.0,
    1.0,10.0,1.0
)
batchedmodel_spawntobuffer(
    "testme",
    "assets/models/tree",
    -2.0,4.0,2.0,
    0.0,40.0,0.0,
    1.0,10.0,1.0
)
batchedmodel_modelbuffertofile(
    "testme",
    "assets/testmap.txt"
)
// batchedmodel_modelbuffertofile("testme", "./assets/tmpmap")
//batchedmodel_spawnfrombuffer("testme",batch)
//batchedmodel_spawnfrombuffer(batch,batch)
// mdl = batchedmodel_buildfromfile("./assets/tmpmap")
batchedmodel_spawnfrombuffer(
    "oioi",
    "assets/models/tree"
    )
// batchedmodel_setscale("test2",2.0,20.0,2.0)
//batchedmodel_modelspawnfrombuffer("testme",thismodel)
batchedmodel_setposition("oioi",-3.0,1.0,2.0)
//batchedmodel_setrotation("oioi",2.0,67.0,2.0)
coroutine "gamecontrols" each 26{
    if key.event == true{
        if key.d == "down" {
            //if *mysprite.side != "right"{
                //spritesetanimation("mysprite","anim_right")
                if player.gravityside == "down" && player.ontile == true{
                    mysprite.animate("anim_right")
                }
                *mysprite.side = "right"
            //}
            onx = player.x + 0.5
            ony = player.y + 0.1
            if maptiles.collisioncheck(player.aabb,onx,ony,player.z) == false{
                camera.x += speed
                mysprite.x += speed
                camera.targetx += speed
            }
        }
        if key.a == "down" {
            if player.gravityside == "down" && player.ontile == true{
                mysprite.animate("anim_left")
            }
            onx = player.x - 0.5
            ony = player.y + 0.1
            if maptiles.collisioncheck(player.aabb,onx,ony,player.z) == false{
                *mysprite.side = "left"
                camera.x -= speed
                mysprite.x -= speed
                camera.targetx -= speed
            }

        }
        if key.w == "down" {
            if player.gravityside == "down" && player.ontile == true{
                mysprite.animate("anim_up")
            }
            
            *mysprite.side = "up"
            onz = player.z - 0.5
            ony = player.y + 0.1
            if maptiles.collisioncheck(player.aabb,player.x,ony,onz) == false{
                camera.z -= speed
                camera.targetz -= speed
                mysprite.z -= speed
            }


        }
        if key.space == "down"{
            if player.jumping == false {
                player.jump()
            }
        }
        if key.s == "down" {
            if player.gravityside == "down" && player.ontile == true{
                mysprite.animate("anim_down")
            }
            mysprite.animate("anim_down")
            *mysprite.side = "down"
            onz = player.z + 0.5
            ony = player.y + 0.1
            if maptiles.collisioncheck(player.aabb,player.x,ony,onz) == false{
            camera.z += speed
            camera.targetz += speed
            mysprite.z += speed
            }

        }
        if key.up == "down" {
            camera.offy += speed
            camera.targety += speed
            camera.z += speed
        }
        if key.down == "down" {
            camera.offy -= speed
            camera.targety -= speed
            camera.z -= speed
        }
        if key.i == "down" {
            objectsetrotation("triangle",30.0,35.0,34.0) // working
        }
        if key.o == "down" {
            objectsetrotation("triangle",0.0,0.0,0.0)
        }
        if key.q == "down" {
            // spritesetanimation("oi","anim_left")
            // print(join(oi.anim_right,"+"),"r")
            mycolor = argb("rgb(47, 35, 98)")
            mycolor !
            objectsetcolor("draakje",mycolor[0],mycolor[1],mycolor[2],1.0)
            batchedmodel_delete("map")
            print(cat(camera.x," - ",camera.y," - ",camera.z),"bg")
        }
        if key.e == "down" {
            spritesetanimation("mysprite","anim_right")
            print(join(mysprite.anim_right,"+"),"r")
            print(cat(camera.x," - ",camera.y," - ",camera.z),"bg")
        }
        mysprite.idletimer = timerinit()

        //*mysprite2.setposition(camera.x,4.5,0.0)
    }

    //objectsetposition("mysprite",camera.x,1.5,0.0)
    //*mysprite.setposition(camera.x,1.5,mysprite.z)

    player.setposition(mysprite.x,mysprite.y,mysprite.z)
    if mysprite.y < -20.0 {
        mysprite.x = 10.0
        mysprite.y = 10.0
        mysprite.z = 3.0 
        camera.z = mysprite.z + 3.0
        player.setposition(mysprite.x,mysprite.y,mysprite.z)
        setcamerapos("camera",mysprite.x,camy,camera.z,mysprite.x,mysprite.y,mysprite.z)
    }
    if timerdiff(mysprite.idletimer) > 100 {
        if player.gravityside == "down" && player.ontile == true{
            mysprite.animate(cat("anim_idle",*mysprite.side))
        }

        mysprite.idletimer = timerinit()
    }
}
// xup = -50.0
// bloodimg = loadtexture("assets/200.gif")
// for xobj to 1{
//     xup = xup + 2.0
//    // print(xup)
//     obj = cat("testobject",xobj,"ok")
//     print(square(obj,xup,0.0,0.0))
//     settexture(obj,bloodimg)
//     *obj.rx = random(0.0,350.0,2)
//     *obj.ry = random(0.0,350.0,2)
//     sleep(1)
//     coroutine obj each 90{
//             *obj.rx += 0.05
//             if *obj.rx > 359.0{
//                 *obj.rx = 0.0
//             }
//             objectsetrotation(
//                 obj,
//                 *obj.ry,
//                 *obj.rx,
//                 0.0
//             )
//     }

// }


//*spritebase.anim_runright !!

timer = timerinit()
fps = 0
fpscounter = 0
fpstimer = timerinit()
uptime = 0
coroutine "fpscounter"{
    fpscounter ++
    if timerdiff(fpstimer) > 2999{
        fps = fpscounter / 3
        uptime += 3
        uptimeinmin = round(devide(uptime,60),3)

        fpscounter = 0
        print(cat("fps:",round(fps,3)," uptime: ",uptimeinmin,"min"),"bg")
        fpstimer = timerinit()
    }
}
coroutine "game2" each 10000{
    $testtriangle = "go"
    break self
}
