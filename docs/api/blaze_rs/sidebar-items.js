initSidebarItems({"enum":[["SpriteFlip","Defines supported flip modes that can be used when the sprite is drawn."]],"fn":[["clear","Clears the current framebuffer."],["get_last_error","Returns last API error information string. The same string is returned from API calls which return `Result<..., String>`. Note: error string may be empty even if an API call failed, the output `Result<..., String>` will return an `Err(\"Unknown error\")` in that case."],["load","Loads OpenGL functions used by this library. Requires an active window with an OpenGL context (version 3.0 core and above). # Example (using SDL2) ```    use blaze_rs::load;    use sdl2::video::GLProfile;"],["set_clear_color","Sets the color which is used for clearing the framebuffer."],["set_viewport","Sets viewport dimensions. All sprite position and size calculations will be based on them. In most cases, you should pass the window size in pixels here, or a scaled value for pixel-art based games, for example."]],"mod":[["blend","Defines blending-related functionality."],["dynamic","Dynamic batched drawing. Designed for moving sprites."],["immediate","Immediate-mode drawing."],["rendertarget","Render target support."],["shader","Custom shader support."],["static","Static batched drawing. Designed for static geometry."],["texture","Texture loading and saving."]],"struct":[["Color","Represents a RGBA color."],["Rectangle","A rectangle which has it's top-left corner position, width and height expressed in floats."],["SpriteQuad","Underlying sprite quad data structure used by VAOs (vertex array objects)."],["Vector2","Two-dimensional float vector."],["Vector4","Four-dimensional float vector."],["Vertex","Underlying vertex data structure used by VAOs (vertex array objects)."]],"type":[["CallResult","Alias for `Result<(), String>`."],["GLProcLoader","Defines a type for OpenGL procedure loader."]]});