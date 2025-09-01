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
        image2d("cursor", "assets/cursor.png",self.x,self.y,0.02,0.02,3)
    }
    self.x = 0.0
    self.y = 0.0
}
class gui{
    func spawn(xp,yp,wp,hp){
        pos = pixelstocoords(vec(xp,yp),vec(100,100))
        self.posx = pos[0]
        self.posy = pos[1]
        self.width = wp
        self.height = hp
    }
    func move(){
        if self.active == true{
            pos = pixelstocoords(mouse.pos,$windowsize)
            correctx = self.width / 3
            self.posx = pos[0] - correctx + 0.05
            self.posy = pos[1] - self.height + 0.05
            //self.open()
            objectsetposition(cat(self,"_backframe"),self.posx,self.posy,0.0)
            ny = self.posy + self.height - 0.05
            nx = self.posx - 0.05
            objectsetposition(cat(self,"_bar"),nx,ny,0.0)
            nx = self.posx + self.width - 0.05
            objectsetposition(cat(self,"_close"),nx,ny,0.0)
            nx = self.posx + self.width - 0.1
            objectsetposition(cat(self,"_minimize"),nx,ny,0.0)
        }

    }
    func open(){
        if self.active == false{
            image2d(cat(self,"_backframe"),self.backframe,self.posx,self.posy,self.width,self.height,1)
            ny = self.posy + self.height - 0.05
            nx = self.posx - 0.05
            image2d(cat(self,"_bar"),self.button_bar,nx,ny,devide(self.width,3),0.025,2)
            nx = self.posx + self.width - 0.05
            image2d(cat(self,"_close"),self.button_close,nx,ny,self.topbuttonwidth,self.topbuttonheight,2)
            nx = self.posx + self.width - 0.1
            image2d(cat(self,"_minimize"),self.button_minimize,nx,ny,self.topbuttonwidth,self.topbuttonheight,2)
            self.active = true
            thismenu = self // set it before spawning the coroutine
            coroutine self each 12{
                if mousekey.left == "down"{
                    pos = pixelstocoords(mouse.pos,$windowsize)
                    fromx = *thismenu.posx + *thismenu.width - 0.05
                    tox = fromx + 0.05
                    fromy = *thismenu.posy + *thismenu.height - 0.1
                    toy = fromy + 0.1
                    if pos[0] > fromx && pos[0] < tox && pos[1] > fromy && pos[1] < toy{
                        *thismenu.close()
                    }
                    correctx = *thismenu.width / 2
                    fromx = *thismenu.posx - 0.1
                    tox = fromx + correctx + 0.1
                    if pos[0] > fromx && pos[0] < tox && pos[1] > fromy && pos[1] < toy{
                        *thismenu.move()
                    }
                }
            }
        }
    }
    func close(){
        if self.active == true{
            objectremove(cat(self,"_backframe"))
            objectremove(cat(self,"_bar"))
            objectremove(cat(self,"_close"))
            objectremove(cat(self,"_minimize"))
            self.active = false
            break cat("coroutine_",self)
        }

    }
    self.backframe = loadtexture("assets/menuframe2.png")
    self.button_bar = loadtexture("assets/guimenu/bar.png")
    self.button_close = loadtexture("assets/guimenu/close.png")
    self.button_minimize = loadtexture("assets/guimenu/minimize.png")
    self.topbuttonwidth = 0.025
    self.topbuttonheight = 0.025
    self.active = false
}
gui.spawn(70,30,0.35,0.3)
gui2 : gui
gui2.spawn(20,50,0.2,0.3)
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