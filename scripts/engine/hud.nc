class hud{
    func loadfont(fontdir){
        if fromright(fontdir,1) == "/"{
            fontdir = trimright(fontdir,1)
        }
        for x in listdir(fontdir){
            if fromright(x,4) == ".png"{
                xitem = cat(fontdir,x)
                print("loading font ",fontdir," -chr>",xitem,"g")
                loadtexture(xitem)
            }
        }
    }
    func testremove(){
        textnodedelete("testtext")
    }

}
class cursor{
    func move(){
        //if mouse.diff[0] != 0.0 || mouse.diff[1] != 0.0{
            $coords = pixelstocoords(mouse.pos,$windowsize)
            objectsetposition("cursor",$coords[0],$coords[1],0.0)
        //}
    }
    func init(){
        texture = loadtexture("assets/cursor.png")
        image2d("cursor", "assets/cursor.png",self.x,self.y,0.02,0.02,5)
    }
    self.x = 0.0
    self.y = 0.0
}
class gui{
    func spawn(title,xp,yp,wp,hp){
        pos = pixelstocoords(vec(xp,yp),vec(100,100))
        self.posx = pos[0]
        self.posy = pos[1]
        self.width = wp / 2
        self.height = hp / 2
        self.title = title
    }
    func addtext(text,x,y,fn){
        self.totalcontrols ++
        controlid = cat(self,"_ctrl",self.totalcontrols)
        *controlid.type = "text"
        *controlid.text = text
        *controlid.x = x
        *controlid.y = y
        *controlid.fn = fn
        self.controlids = arraypush(self.controlids,controlid)
    }
    func addbutton(text,x,y,fn){
        self.totalcontrols ++
        controlid = cat(self,"_ctrl",self.totalcontrols)
        *controlid.type = "button"
        *controlid.text = text
        *controlid.x = x
        *controlid.y = y
        *controlid.fn = fn
        self.controlids = arraypush(self.controlids,controlid)
    }
    func addimage(path,x,y,w,h,fn){
        self.totalcontrols ++
        controlid = cat(self,"_ctrl",self.totalcontrols)
        *controlid.type = "image"
        *controlid.path = path
        *controlid.x = x
        *controlid.y = y
        *controlid.width = w
        *controlid.height = h
        *controlid.fn = fn
        self.controlids = arraypush(self.controlids,controlid)
    }
    func controlsonclick(){
        pos = self.getposinmenu()
        for xid in self.controlids{
            tox = *xid.x + 0.15
            fromy = *xid.y + 0.025
            toy = *xid.y - 0.025
            if pos[0] > *xid.x && pos[0] < tox && pos[1] < fromy && pos[1] > toy{
                print("clicked on",xid,@lf,"y")
                nscript::rawcode(*xid.fn)
            }
            else{
               // print("y=",pos[1],@lf,"y")
            }
        } 
    }
    func movecontrols(){
        for xid in self.controlids{
            nx = self.posx - self.width + *xid.x
            halfheight = self.height // 2
            ny = self.posy + halfheight + *xid.y
            match *xid.type{
                "text" =>{
                    textnodeupdate(xid,*xid.text,nx,ny,21,"assets/font_rngwhite")
                }
                "button" =>{
                    framex = nx + 0.09
                    objectsetposition(xid,framex,ny,0.0)
                    textx = nx + 0.01
                    textnodeupdate(xid,*xid.text,textx,ny,21,"assets/font_rngwhite")
                }
                "image" =>{
                    framex = nx + 0.09
                    objectsetposition(xid,framex,ny,0.0)
                }
            }
        } 
    }
    func spawncontrols(){
        for xid in self.controlids{
            nx = self.posx - self.width + *xid.x
            halfheight = self.height // 2
            ny = self.posy + halfheight + *xid.y
            match *xid.type{
                "text" =>{
                    textnode(xid,*xid.text,nx,ny,21,"assets/font_rngwhite")
                }
                "button" =>{
                    framex = nx + 0.09
                    image2d(xid,self.button_frame,framex,ny,0.1,0.025,2)
                    textx = nx + 0.01
                    textnode(xid,*xid.text,textx,ny,21,"assets/font_rngwhite")
                }
                "image" =>{
                    framex = nx + 0.09
                    image2d(xid,*xid.path,framex,ny,*xid.width,*xid.height,2)
                }
            }
        } 
    }
    func deletecontrols(){
        for xid in self.controlids{
            match *xid.type{
                "text" =>{
                    textnodedelete(xid)
                }
                "button" =>{
                    textnodedelete(xid)
                    objectremove(xid)
                }
                "image" =>{
                    objectremove(xid)
                }
            }
        } 
    }
    func move(){
        if self.active == true{
            pos = pixelstocoords(mouse.pos,$windowsize)
            correctx = self.width / 2
            self.posx = pos[0] - correctx  + 0.15// + correctx //self.width  - correctx - 0.05
            self.posy = pos[1] - self.height + 0.05
            //self.open()
            objectsetposition(cat(self,"_backframe"),self.posx,self.posy,0.0)
            textx = self.posx - devide(self.width,2) - 0.05
            ny = self.posy + self.height - 0.05
            textnodeupdate(cat(self,"_title"),self.title,textx,ny,21,"assets/font_rngwhite")
            nx = self.posx - 0.05
            objectsetposition(cat(self,"_bar"),self.posx,ny,0.0)
            nx = self.posx + self.width  - self.topbuttonwidth
            objectsetposition(cat(self,"_close"),nx,ny,0.0)
            nx = self.posx + self.width  - self.topbuttonwidth - multiply(self.topbuttonwidth,2)
            objectsetposition(cat(self,"_minimize"),nx,ny,0.0)
            self.movecontrols()
        }

    }
    func open(){
        if self.active == false{
            image2d(cat(self,"_backframe"),self.backframe,self.posx,self.posy,self.width,self.height,1)
            ny = self.posy + self.height - 0.025
            nx = self.posx - 0.05
            textx = self.posx - devide(self.width,2) - 0.05
            textnodeupdate(cat(self,"_title"),self.title,textx,ny,21,"assets/font_rngwhite")
            image2d(cat(self,"_bar"),self.button_bar,self.posx,ny,self.width,0.04,2)
            //image2d(cat(self,"_bar"),self.button_bar,self.posx,self.posy,self.width,0.025,2)
            nx = self.posx + self.width - self.topbuttonwidth
            image2d(cat(self,"_close"),self.button_close,nx,ny,self.topbuttonwidth,self.topbuttonheight,3)
            nx = self.posx + self.width - self.topbuttonwidth - multiply(self.topbuttonwidth,2)
            image2d(cat(self,"_minimize"),self.button_minimize,nx,ny,self.topbuttonwidth,self.topbuttonheight,3)
            self.active = true
            self.spawncontrols()
            thismenu = self // set it before spawning thecontrolid coroutine
            coroutine self each 12{
                if mousekey.right == "down"{
                    *thismenu.getposinmenu()
                }
                if mousekey.left == "down"{
                    *thismenu.controlsonclick()
                    pos = pixelstocoords(mouse.pos,$windowsize)
                    fromx = *thismenu.posx + *thismenu.width - 0.05
                    tox = fromx + 0.05
                    fromy = *thismenu.posy + *thismenu.height - 0.1
                    toy = fromy + 0.2
                    if pos[0] > fromx && pos[0] < tox && pos[1] > fromy && pos[1] < toy{
                        *thismenu.close()
                        break self
                    }
                    correctx = *thismenu.width / 2
                    fromx = *thismenu.posx - 0.2
                    tox = fromx + correctx + 0.15
                    if pos[0] > fromx && pos[0] < tox && pos[1] > fromy && pos[1] < toy{
                        *thismenu.move()
                    }
                }
            }
        }
    }
    func getposinmenu(){
        pos = pixelstocoords(mouse.pos,$windowsize)
        fromx = self.posx - self.width
        tox = fromx + multiply(self.width,2)
        fromy = self.posy + self.height
        toy = fromy - multiply(self.height,2)
        result = [-9.9,-9.9]// set impossible , ( so no errors)
        if pos[0] > fromx && pos[0] < tox && pos[1] < fromy && pos[1] > toy{
            rex = pos[0] - fromx
            rey = pos[1] - fromy
            result = [rex,rey]
            //print(self," ->",join(result,"-"),"g")
        }
        result
    }
    func close(){
        if self.active == true{
            objectremove(cat(self,"_backframe"))
            objectremove(cat(self,"_bar"))
            objectremove(cat(self,"_close"))
            objectremove(cat(self,"_minimize"))
            self.active = false
            textnodedelete(cat(self,"_title"))
            self.deletecontrols()
            //break cat("coroutine_",self)
        }

    }
    func toggle(){
        if timerdiff(self.toggletimer) > 300{
            if self.active == false{
                self.open()
            }
            else{
                self.close()
            }  
            self.toggletimer = timerinit()   
        }
    }
    self.toggletimer = timerinit()
    self.title = "ui undifined"
    self.backframe = loadtexture("assets/guivista/backframe.png")
    self.button_bar = loadtexture("assets/guivista/bar.png")
    self.button_close = loadtexture("assets/guivista/close.png")
    self.button_minimize = loadtexture("assets/guivista/minimize.png")
    self.button_frame = loadtexture("assets/guivista/buttonframe.png")
    self.topbuttonwidth = 0.03
    self.topbuttonheight = 0.04
    self.active = false
}
gui0 : gui
gui0.spawn("first ui",70,30,0.7,0.5).addtext("settings",0.05,-0.2,"
gui3.toggle()
print(\"hellow\")
").addbutton("friends",0.05,-0.3,"gui2.toggle()")
gui2 : gui
gui2.spawn("tall ui",20,50,0.4,0.8).addtext("sjaakie",0.01,-0.15).addtext("420blazin",0.01,-0.2).addtext("sjefke",0.01,-0.25).addtext("flippie",0.01,-0.3)

gui3 : gui
gui3.spawn("mini",70,10,0.4,0.4).addtext("vsync : true",0.01,-0.15).addtext("performancemode : true",0.01,-0.2).addimage("assets/tree/1.png",0.2,-0.2,0.025,0.025)
cursor.init()
loadtexture("assets/menuframe2.png")
loadtexture("assets/menuframe.png")
image2d("testbackframe2","assets/tree/1.png",-0.25,0.85,0.08,0.08,2)
loadtexture("assets/menuframe.png")
image2d("testbackframe","assets/menuframe2.png",-0.55,0.85,0.45,0.15,1)


// coroutine "backframe" each 120{
//     objectsetscale("testbackframe",0.1,0.2,0.0)
//     settexture("testbackframe", "assets/tiles/street.png")
//     break self
// }


hud.loadfont("assets/font_rngwhite")
textnode("testtext",cat("blueengine nscript : hello ",@lf,123),-0.99,0.95,35,"assets/font_rngwhite")
textnode("fpsmsg",$fpsmsg,-0.92,0.95,35,"assets/font_rngwhite")
textnodesetcolor("fpsmsg",1.0,0.1,0.5,0.99)
posstart = [1310,780]
// coroutine "cursorspawn" each 200{
//     coroutine "movecursor" each 250{
//     if mousekey.right == "down"{
//         cursor.move()
//     }

//     }
//     break self
// }
coroutine "some" each 999{
    updatewindow()
    textnodeupdate("testtext",cat("wsize = {",join($windowsize,","),"}cur:",join($coords,",")),-0.92,0.95,35,"assets/font_rngwhite")

    textnodeupdate("fpsmsg",$fpsmsg,-0.92,0.90,25,"assets/font_rngwhite")
    //textnodedelete("testtext")
//textnode("testtext",cat("blueengine nscript : hello ",@lf,123),-0.99,0.95,35,"assets/font_rngwhite")
}
gamepad::checkdevice()
coroutine "gamepad" each 20{
    gamepad::checkdevice()
    //gamepad::checkinput(0)
}