class blueengine{
    self.vsync = false
    self.powermode = true
    self.renderwidth = 1024
    self.renderheight = 600
    self.title = "blueengine nscriptv3 test"
    if @OS == "Unix"{
        self.render = "Primary"
    }
    else{
        self.render = "DX12"
    }
}
class gamepad{
    self.dpadleft = 0.0
    self.dpadright = 0.0
    self.dpadup = 0.0
    self.dpaddown = 0.0
    self.leftstickx = 0.0
    self.leftsticky = 0.0
    self.rightstickx = 0.0
    self.rightsticky = 0.0
    self.lefttrigger = 0.0
    self.lefttrigger1 = 0.0
    self.righttrigger = 0.0
    self.righttrigger1 = 0.0
    self.south = 0.0
    self.north = 0.0
    self.west = 0.0
    self.east = 0.0
    gamepad0 : gamepad
    gamepad1 : gamepad
    gamepad2 : gamepad
    gamepad3 : gamepad
}

class camera{
    self.x = 0.0
    self.y = 0.0
    self.z = 0.0
    self.targetx = 0.0
    self.targety = 0.0
    self.targetz = 0.0   
    self.pitch = 0.0
    self.yaw = 0.0
    self.roll = 0.0
}