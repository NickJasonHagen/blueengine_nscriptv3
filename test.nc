

print("starting nc")
camera.x = 0.0
camera.y = 0.0
camera.z = 10.0
camera.targetx = 0.0
camera.targety = 0.0
camera.targetz = 0.0
speed = 0.3
controllertimer = timerinit()
spritebase = cat(@scriptdir,"assets/dude1")
spritebase = loadsprite(spritebase)


mysprite = "mysprite"
mysprite = square(mysprite,0.0,3.0,0.0)
objectsetscale(mysprite,0.5,1.0,0.5)
mysprite : *spritebase
addsprite("mysprite")
spritesetanimation("mysprite","anim_right")
for xtile in listdir(cat(@scriptdir,"assets/mariotiles"),true){
    print("loaded texture",loadtexture(xtile),"by")
}
for x to 1{
    xtile = cat("tile",x)
    square(xtile,x,0.0,0.0)
    objectsetscale(xtile,0.5,0.5,0.5)
    settexture(xtile,cat(@scriptdir,"assets/mariotiles/grass.png"))
    xtile = cat("tiler2",x)
    square(xtile,x,-1.0,0.0)
    objectsetscale(xtile,0.5,0.5,0.5)
    settexture(xtile,cat(@scriptdir,"assets/mariotiles/dirt.png"))
}
xtile = "background"
square(xtile,x,20.0,-20.0)
objectsetscale(xtile,400.5,70.5,10.5)
settexture(xtile,cat(@scriptdir,"assets/mariotiles/background.png"))
print(*spritebase.anim_right,"by") !
coroutine "gamecontrols" each 26{
    if key.event == true{
        if key.d == "down" {
            if *mysprite.side != "right"{
                spritesetanimation("mysprite","anim_right")
                *mysprite.side = "right"
            }
            camera.x += speed
            camera.targetx += speed
        }
        if key.a == "down" {
            if *mysprite.side != "left"{
                spritesetanimation("mysprite","anim_left")
                *mysprite.side = "left"
            }
            camera.x  -= speed
            camera.targetx -= speed
        }
        if key.w == "down" {
            camera.z -= speed
            camera.targetz -= speed
        }
        if key.s == "down" {
            camera.z += speed
            camera.targetz += speed
        }
        if key.space == "down" {
            camera.y += speed
            camera.targety += speed
        }
        if key.c == "down" {
            camera.y -= speed
            camera.targety -= speed
        }
        if key.i == "down" {
            objectsetrotation("triangle",30.0,35.0,34.0) // working
        }
        if key.o == "down" {
            objectsetrotation("triangle",0.0,0.0,0.0)
        }
        if key.q == "down" {
            spritesetanimation("mysprite","anim_left")
            print(join(mysprite.anim_right,"+"),"r")
            print(cat(camera.x," - ",camera.y," - ",camera.z),"bg")
        }
        if key.e == "down" {
            spritesetanimation("mysprite","anim_right")
            print(join(mysprite.anim_right,"+"),"r")
            print(cat(camera.x," - ",camera.y," - ",camera.z),"bg")
        }
        setcamerapos("camera",camera.x,camera.y,camera.z,camera.targetx,camera.targety,camera.targetz)
        objectsetposition("mysprite",camera.x,1.5,0.0)
    }
}
xup = -50.0
bloodimg = loadtexture("assets/200.gif")
for xobj to 1{
    xup = xup + 2.0
   // print(xup)
    obj = cat("testobject",xobj,"ok")
    print(square(obj,xup,0.0,0.0))
    settexture(obj,bloodimg)
    *obj.rx = random(0.0,350.0,2)
    *obj.ry = random(0.0,350.0,2)
    sleep(1)
    coroutine obj each 90{
            *obj.rx += 0.05
            if *obj.rx > 359.0{
                *obj.rx = 0.0
            }
            objectsetrotation(
                obj,
                *obj.ry,
                *obj.rx,
                0.0
            )
    }

}


*spritebase.anim_runright !!

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
