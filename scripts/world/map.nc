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
class map{
    func createfile(){
        z = 1.0
        x = 1.0
        y = 1.6
        fdata = ""
        for xi to 20000{
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
            if x > 30 && x < 120 && z > 10 && z < 120 {
                model  = "./assets/tiles/mud.png"
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