class blueengine{
    self.vsync = false
    self.powermode = false
    self.renderwidth = 1024
    self.renderheight = 600
    self.title = "blueengine nscriptv3 test"
    if @OS == "Unix"{
        self.render = "Vulkan"
    }
    else{
        self.render = "DX12"
    }
}
