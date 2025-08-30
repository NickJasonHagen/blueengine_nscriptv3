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
        if mouse.diff[0] != 0.0 || mouse.diff[1] != 0.0{
            $coords = pixelstocoords(mouse.pos,$windowsize)
            objectsetposition("cursor",$coords[0],$coords[1],0.0)
        }
    }
    func init(){
        texture = loadtexture("assets/cursor.png")
        image2d("cursor", "assets/cursor.png",self.x,self.y,0.02,0.02,3)
    }
    self.x = 0.0
    self.y = 0.0
}
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