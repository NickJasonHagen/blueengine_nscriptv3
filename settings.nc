class blueengine{
    self.vsync = false
    self.powermode = true
    self.renderwidth = 1024
    self.renderheight = 600
    if @OS == "Unix"{
        self.render = "Vulkan"
    }
    else{
        self.render = "DX12"
    }
}