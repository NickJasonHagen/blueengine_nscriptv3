class blueengine{
    self.vsync = true
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
