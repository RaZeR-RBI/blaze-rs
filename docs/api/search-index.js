var N=null,E="",T="t",U="u",searchIndex={};
var R=["blaze_rs","X coordinate","Y coordinate","callresult","initflags","option","string","vector2","texture","spritebatchopts","rectangle","spriteflip","lower_draw","Lower-level drawing function, which allows specifying a…","spritequad","rendertarget","uniformloc","Sets a uniform value for the currently used shader.","Pushes a sprite into the batch using the specified…","Flushes the batch onto the screen, drawing everything.    ","get_options","Returns options which were used when this object was…","staticbatchopts","Width in pixels","Height in pixels","Default flags","Returns an empty set of flags.","Returns the set containing all flags.","imageflags","Returns the raw value of the flags currently stored.","from_bits","Convert from underlying bit representation, unless that…","from_bits_truncate","Convert from underlying bit representation, dropping any…","is_empty","Returns `true` if no flags are currently stored.","Returns `true` if all flags are currently set.","intersects","Returns `true` if there are flags common to both `self`…","contains","Returns `true` all of the flags in `other` are contained…","Inserts the specified flags in-place.","Removes the specified flags in-place.","Toggles the specified flags in-place.","Inserts or removes the specified flags depending on the…","texturefilter","texturewrap","result","imagechannels","to_owned","clone_into","try_from","borrow_mut","try_into","type_id","borrow","typeid","blaze_rs::blend","blaze_rs::dynamic","blaze_rs::immediate","blaze_rs::rendertarget","blaze_rs::shader","blaze_rs::static","blaze_rs::texture","partial_cmp","ordering","blendmode","saveimageformat","blendfactor","Returns the set difference of the two sets of flags.","sub_assign","Disables all flags enabled in the set.","Returns the complement of this set of flags.","Returns the intersection between the two sets of flags.","Returns the union of the two sets of flags.","Returns the left flags, but with all the right flags…","bitand_assign","Disables all flags disabled in the set.","bitor_assign","Adds the set of flags.","bitxor_assign","Toggles the set of flags.","from_iter","formatter","from_i64","from_u64","Rectangle","SpriteQuad","SpriteFlip","BlendMode","BlendFactor","SpriteBatchOpts","InitFlags","SpriteBatch","Immediate","RenderTarget","StaticBatchOpts","StaticBatch","ImageChannels","SaveImageFormat","TextureFilter","TextureWrap","ImageFlags"];

searchIndex[R[0]]={"doc":"This crate wraps the blaze library which is geared towards…","i":[[3,R[86],R[0],"A rectangle which has it's top-left corner position, width…",N,N],[12,"x",E,"X position of top left corner",0,N],[12,"y",E,"Y position of top left corner",0,N],[12,"w",E,"Width",0,N],[12,"h",E,"Height",0,N],[3,R[87],E,"Underlying sprite quad data structure used by VAOs (vertex…",N,N],[12,"vertices",E,"Four vertices defining the quad, defined in the following…",1,N],[3,"Vector2",E,"Two-dimensional float vector.",N,N],[12,"x",E,R[1],2,N],[12,"y",E,R[2],2,N],[3,"Vector4",E,"Four-dimensional float vector.",N,N],[12,"x",E,R[1],3,N],[12,"y",E,R[2],3,N],[12,"z",E,"Z coordinate",3,N],[12,"w",E,"W coordinate",3,N],[3,"Vertex",E,"Underlying vertex data structure used by VAOs (vertex…",N,N],[12,"x",E,"X position on the screen",4,N],[12,"y",E,"Y position on the screen",4,N],[12,U,E,"Texture coordinate (U)",4,N],[12,"v",E,"Texture coordinate (V)",4,N],[12,"r",E,"Color (R)",4,N],[12,"g",E,"Color (G)",4,N],[12,"b",E,"Color (B)",4,N],[12,"a",E,"Color (alpha)",4,N],[3,"Color",E,"Represents a RGBA color.",N,N],[12,"r",E,"Red component, defined in range from 0.0 to 1.0",5,N],[12,"g",E,"Green component, defined in range from 0.0 to 1.0",5,N],[12,"b",E,"Blue component, defined in range from 0.0 to 1.0",5,N],[12,"a",E,"Alpha component, defined in range from 0.0 to 1.0",5,N],[4,R[88],E,"Defines supported flip modes that can be used when the…",N,N],[13,"None",E,"Draws the sprite texture as it is",6,N],[13,"FlipH",E,"Flips the texture horizontally",6,N],[13,"FlipV",E,"Flips the texture vertically",6,N],[13,"Both",E,"Flips the texture both horizontally and vertically",6,N],[5,"get_last_error",E,"Returns last API error information string. The same string…",N,[[],[R[5],[R[6]]]]],[5,"load",E,"Loads OpenGL functions used by this library. Requires an…",N,[[["glprocloader"]],[R[3]]]],[5,"set_viewport",E,"Sets viewport dimensions. All sprite position and size…",N,[[["u32"],["u32"]],[R[3]]]],[5,"set_clear_color",E,"Sets the color which is used for clearing the framebuffer.",N,[[["color"]]]],[5,"clear",E,"Clears the current framebuffer.",N,[[]]],[0,"blend",E,"Defines blending-related functionality.",N,N],[3,R[89],R[57],"Defines a pair of blend factors (F_src and F_dst) for the…",N,N],[12,"src",E,"Source blend factor",7,N],[12,"dst",E,"Destination blend factor",7,N],[4,R[90],E,"Defines a blend factor (F_src or F_dst) for the blending…",N,N],[13,"Zero",E,"Color is discarded (multiplied by 0)",8,N],[13,"One",E,"Color is used as is (multiplied by 1)",8,N],[13,"SrcColor",E,"Color is multiplied by source color",8,N],[13,"OneMinusSrcColor",E,"Color is multiplied by (vec(1) - source color)",8,N],[13,"DstColor",E,"Color is multiplied by destination color",8,N],[13,"OneMinusDstColor",E,"Color is multiplied by (vec(1) - destination color)",8,N],[13,"SrcAlpha",E,"Color is multiplied by source alpha value",8,N],[13,"OneMinusSrcAlpha",E,"Color is multiplied by (1 - source alpha)",8,N],[13,"DstAlpha",E,"Color is multiplied by destination alpha value",8,N],[13,"OneMinusDstAlpha",E,"Color is multiplied by (1 - destination alpha)",8,N],[5,"set_blend_mode",E,"Sets the current blending mode to be used. Note - the…",N,[[[R[66]]]]],[17,"NORMAL",E,"Normal blending mode (alpha-blend)",N,N],[17,"ADDITIVE",E,"Additive blending mode",N,N],[17,"MULTIPLY",E,"Multiplicative blending mode",N,N],[0,"dynamic",R[0],"Dynamic batched drawing. Designed for moving sprites.",N,N],[3,R[93],R[58],"Defines a dynamic sprite batching object. ",N,N],[3,R[91],E,"Defines creation options for dynamic sprite batching…",N,N],[12,"max_buckets",E,"Maximum count of buckets (one bucket shares same texture)",9,N],[12,"max_sprites_per_bucket",E,"Maximum count of sprites per bucket",9,N],[12,"flags",E,"Additional flags",9,N],[3,R[92],E,"Defines flags that can be used for SpriteBatch creation.",N,N],[18,"Default",E,R[25],10,N],[18,"NoBuffering",E,"Disables VAO buffering",10,N],[11,"empty",E,R[26],10,[[],[R[4]]]],[11,"all",E,R[27],10,[[],[R[4]]]],[11,"bits",E,R[29],10,[[["self"]],["u32"]]],[11,R[30],E,R[31],10,[[["u32"]],[R[5],[R[4]]]]],[11,R[32],E,R[33],10,[[["u32"]],[R[4]]]],[11,R[34],E,R[35],10,[[["self"]],["bool"]]],[11,"is_all",E,R[36],10,[[["self"]],["bool"]]],[11,R[37],E,R[38],10,[[["self"],[R[4]]],["bool"]]],[11,R[39],E,R[40],10,[[["self"],[R[4]]],["bool"]]],[11,"insert",E,R[41],10,[[["self"],[R[4]]]]],[11,"remove",E,R[42],10,[[["self"],[R[4]]]]],[11,"toggle",E,R[43],10,[[["self"],[R[4]]]]],[11,"set",E,R[44],10,[[["self"],[R[4]],["bool"]]]],[11,"new",E,"Creates a new SpriteBatch object using specified parameters.",11,[[[R[9]]],[R[47],["spritebatch",R[6]]]]],[11,"draw",E,R[18],11,[[["self"],[R[8]],[R[7]],[R[5],[R[10]]],["f32"],[R[5],[R[7]]],[R[5],[R[7]]],["color"],[R[11]]],[R[3]]]],[11,R[12],E,R[13],11,[[["self"],[R[8]],[R[14]]],[R[3]]]],[11,"present",E,R[19],11,[[["self"]],[R[3]]]],[11,R[20],E,R[21],11,[[["self"]],[R[9]]]],[0,"immediate",R[0],"Immediate-mode drawing.",N,N],[3,R[94],R[59],"Defines static methods used for immediate-mode drawing…",N,N],[11,"draw",E,"Immediately draws a sprite to the screen using specified…",12,[[[R[8]],[R[7]],[R[5],[R[10]]],["f32"],[R[5],[R[7]]],[R[5],[R[7]]],["color"],[R[11]]],[R[3]]]],[11,R[12],E,R[13],12,[[[R[8]],[R[14]]],[R[3]]]],[0,R[15],R[0],"Render target support.",N,N],[3,R[95],R[60],"Defines a render target, which is, basically, an offscreen…",N,N],[12,R[8],E,"Underlying texture",13,N],[12,"width",E,R[23],13,N],[12,"height",E,R[24],13,N],[11,"create",E,"Creates a render target with specified size.",13,[[["u32"],["u32"]],[R[47],[R[15],R[6]]]]],[11,"bind",E,"Binds or unbinds (if None is passed) a render target as…",13,[[[R[5],[R[15]]]],[R[3]]]],[0,"shader",R[0],"Custom shader support.",N,N],[3,"Shader",R[61],"Defines a handle to a custom shader.",N,N],[12,"is_default",E,"Indicates if this is a default shader or not.",14,N],[6,"UniformLoc",E,"Defines type for shader uniform location type.",N,N],[11,"compile",E,"Compiles a new shader.",14,[[["str"],["str"]],[R[47],["shader",R[6]]]]],[11,"get_default",E,"Returns the default shader used by library.",14,[[],["shader"]]],[11,"make_current",E,"Sets this shader as current.",14,[[["self"]],[R[3]]]],[11,"get_uniform_location",E,"Returns the location of an uniform variable with specified…",14,[[["self"],["str"]],[R[5],[R[16]]]]],[11,"set_uniform1f",E,R[17],14,[[[R[16]],["f32"]]]],[11,"set_uniform2f",E,R[17],14,[[[R[16]],["f32"],["f32"]]]],[11,"set_uniform3f",E,R[17],14,[[[R[16]],["f32"],["f32"],["f32"]]]],[11,"set_uniform4f",E,R[17],14,[[[R[16]],["f32"],["f32"],["f32"],["f32"]]]],[11,"set_uniform1i",E,R[17],14,[[[R[16]],["i32"]]]],[11,"set_uniform2i",E,R[17],14,[[[R[16]],["i32"],["i32"]]]],[11,"set_uniform3i",E,R[17],14,[[[R[16]],["i32"],["i32"],["i32"]]]],[11,"set_uniform4i",E,R[17],14,[[[R[16]],["i32"],["i32"],["i32"],["i32"]]]],[11,"set_uniform1ui",E,R[17],14,[[[R[16]],["u32"]]]],[11,"set_uniform2ui",E,R[17],14,[[[R[16]],["u32"],["u32"]]]],[11,"set_uniform3ui",E,R[17],14,[[[R[16]],["u32"],["u32"],["u32"]]]],[11,"set_uniform4ui",E,R[17],14,[[[R[16]],["u32"],["u32"],["u32"],["u32"]]]],[11,"set_uniform_matrix_2fv",E,R[17],14,N],[11,"set_uniform_matrix_3fv",E,R[17],14,N],[11,"set_uniform_matrix_4fv",E,R[17],14,N],[11,"set_uniform_matrix_2x3fv",E,R[17],14,N],[11,"set_uniform_matrix_3x2fv",E,R[17],14,N],[11,"set_uniform_matrix_2x4fv",E,R[17],14,N],[11,"set_uniform_matrix_4x2fv",E,R[17],14,N],[11,"set_uniform_matrix_3x4fv",E,R[17],14,N],[11,"set_uniform_matrix_4x3fv",E,R[17],14,N],[0,"static",R[0],"Static batched drawing. Designed for static geometry.",N,N],[3,R[97],R[62],"Defines a static sprite batching object.",N,N],[3,R[96],E,"Defines creation options for static sprite batching object…",N,N],[12,R[8],E,"Texture that should be used by this batch",15,N],[12,"max_sprites",E,"Maximum sprite count",15,N],[11,"new",E,"Creates a new StaticBatch using specified options.",16,[[[R[22]]],[R[47],["staticbatch",R[6]]]]],[11,"draw",E,R[18],16,[[["self"],[R[7]],[R[5],[R[10]]],["f32"],[R[5],[R[7]]],[R[5],[R[7]]],["color"],[R[11]]],[R[3]]]],[11,R[12],E,R[13],16,[[["self"],[R[14]]],[R[3]]]],[11,"present",E,R[19],16,N],[11,R[20],E,R[21],16,[[["self"]],[R[22]]]],[0,R[8],R[0],"Texture loading and saving.",N,N],[3,"Texture",R[63],"Defines a texture.",N,N],[12,"id",E,"OpenGL texture ID",17,N],[12,"width",E,R[23],17,N],[12,"height",E,R[24],17,N],[3,R[102],E,"Defines various flags that can be supplied to image loader.",N,N],[4,R[98],E,"Defines which channels the image loader should load when…",N,N],[13,"Auto",E,"Load channels as defined by image data (mostly used)",18,N],[13,"Grayscale",E,"Load as grayscale",18,N],[13,"GrayscaleAlpha",E,"Load as grayscale alpha",18,N],[13,"RGB",E,"Load red, green and blue channels",18,N],[13,"RGBA",E,"Load red, green, blue and alpha channels",18,N],[4,R[99],E,"Defines supported formats for saving images.",N,N],[13,"TGA",E,"TGA",19,N],[13,"BMP",E,"BMP",19,N],[13,"DDS",E,"DDS",19,N],[4,R[100],E,"Defines texture filtering options.",N,N],[13,"Nearest",E,"Use nearest filtering (good for pixel-art)",20,N],[13,"Linear",E,"Use linear filtering (smoothes scaled textures)",20,N],[4,R[101],E,"Defines texture wrapping options.",N,N],[13,"ClampToEdge",E,"Clamp to edge (no repeat)",21,N],[13,"Repeat",E,"Repeat",21,N],[13,"MirroredRepeat",E,"Repeat with mirroring",21,N],[5,"save_screenshot",E,"Saves a screenshot of specified window region to file.",N,[[["str"],[R[67]],["u32"],["u32"],["u32"],["u32"]],[R[3]]]],[18,"None",E,R[25],22,N],[18,"PowerOfTwo",E,"Force power-of-two textures",22,N],[18,"Mipmaps",E,"Generate mipmaps",22,N],[18,"Repeats",E,"Make texture repeated",22,N],[18,"MultiplyAlpha",E,"Multiply alpha",22,N],[18,"InvertY",E,"Invert texture by Y axis (vertically)",22,N],[18,"CompressToDXT",E,"Compress to DXT format",22,N],[18,"DDSLoadDirect",E,"Load as DDS",22,N],[18,"NTSCSafeRGB",E,"Force safe RGB values",22,N],[18,"CoCgY",E,"Load as CoCgY",22,N],[18,"TextureRectangle",E,"Use texture rectangle OpenGL extension",22,N],[11,"empty",E,R[26],22,[[],[R[28]]]],[11,"all",E,R[27],22,[[],[R[28]]]],[11,"bits",E,R[29],22,[[["self"]],["u32"]]],[11,R[30],E,R[31],22,[[["u32"]],[R[5],[R[28]]]]],[11,R[32],E,R[33],22,[[["u32"]],[R[28]]]],[11,R[34],E,R[35],22,[[["self"]],["bool"]]],[11,"is_all",E,R[36],22,[[["self"]],["bool"]]],[11,R[37],E,R[38],22,[[["self"],[R[28]]],["bool"]]],[11,R[39],E,R[40],22,[[["self"],[R[28]]],["bool"]]],[11,"insert",E,R[41],22,[[["self"],[R[28]]]]],[11,"remove",E,R[42],22,[[["self"],[R[28]]]]],[11,"toggle",E,R[43],22,[[["self"],[R[28]]]]],[11,"set",E,R[44],22,[[["self"],[R[28]],["bool"]]]],[11,"set_filtering",E,"Sets the texture filtering options for this texture when…",17,[[["self"],[R[45]],[R[45]]],[R[3]]]],[11,"set_wrap",E,"Sets the texture wrapping options for each axis.",17,[[["self"],[R[46]],[R[46]]],[R[3]]]],[11,"from_memory",E,"Loads a texture from memory. A supported image file data…",17,[[["bytes"],[R[48]],[R[5],["u32"]],[R[28]]],[R[47],[R[8],R[6]]]]],[11,"from_file",E,"Loads a texture from file. A supported image file must be…",17,[[["str"],[R[48]],[R[5],["u32"]],[R[28]]],[R[47],[R[8],R[6]]]]],[11,"get_max_slots",E,"Returns maximum available slots for multitexturing.",17,[[],["u32"]]],[11,"bind",E,"Binds or unbinds a texture to specified slot, starting…",17,[[[R[5],[R[8]]],["u32"]],[R[3]]]],[6,"GLProcLoader",R[0],"Defines a type for OpenGL procedure loader.",N,N],[6,"CallResult",E,"Alias for `Result<(), String>`.",N,N],[11,R[49],E,E,0,[[["self"]],[T]]],[11,R[50],E,E,0,N],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[["self"]],[U]]],[11,R[51],E,E,0,[[[U]],[R[47]]]],[11,R[55],E,E,0,[[["self"]],[T]]],[11,R[54],E,E,0,[[["self"]],[R[56]]]],[11,R[52],E,E,0,[[["self"]],[T]]],[11,R[53],E,E,0,[[["self"]],[R[47]]]],[11,R[49],E,E,1,[[["self"]],[T]]],[11,R[50],E,E,1,N],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[["self"]],[U]]],[11,R[51],E,E,1,[[[U]],[R[47]]]],[11,R[55],E,E,1,[[["self"]],[T]]],[11,R[54],E,E,1,[[["self"]],[R[56]]]],[11,R[52],E,E,1,[[["self"]],[T]]],[11,R[53],E,E,1,[[["self"]],[R[47]]]],[11,R[49],E,E,2,[[["self"]],[T]]],[11,R[50],E,E,2,N],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[["self"]],[U]]],[11,R[51],E,E,2,[[[U]],[R[47]]]],[11,R[55],E,E,2,[[["self"]],[T]]],[11,R[54],E,E,2,[[["self"]],[R[56]]]],[11,R[52],E,E,2,[[["self"]],[T]]],[11,R[53],E,E,2,[[["self"]],[R[47]]]],[11,R[49],E,E,3,[[["self"]],[T]]],[11,R[50],E,E,3,N],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[["self"]],[U]]],[11,R[51],E,E,3,[[[U]],[R[47]]]],[11,R[55],E,E,3,[[["self"]],[T]]],[11,R[54],E,E,3,[[["self"]],[R[56]]]],[11,R[52],E,E,3,[[["self"]],[T]]],[11,R[53],E,E,3,[[["self"]],[R[47]]]],[11,R[49],E,E,4,[[["self"]],[T]]],[11,R[50],E,E,4,N],[11,"from",E,E,4,[[[T]],[T]]],[11,"into",E,E,4,[[["self"]],[U]]],[11,R[51],E,E,4,[[[U]],[R[47]]]],[11,R[55],E,E,4,[[["self"]],[T]]],[11,R[54],E,E,4,[[["self"]],[R[56]]]],[11,R[52],E,E,4,[[["self"]],[T]]],[11,R[53],E,E,4,[[["self"]],[R[47]]]],[11,R[49],E,E,5,[[["self"]],[T]]],[11,R[50],E,E,5,N],[11,"from",E,E,5,[[[T]],[T]]],[11,"into",E,E,5,[[["self"]],[U]]],[11,R[51],E,E,5,[[[U]],[R[47]]]],[11,R[55],E,E,5,[[["self"]],[T]]],[11,R[54],E,E,5,[[["self"]],[R[56]]]],[11,R[52],E,E,5,[[["self"]],[T]]],[11,R[53],E,E,5,[[["self"]],[R[47]]]],[11,R[49],E,E,6,[[["self"]],[T]]],[11,R[50],E,E,6,N],[11,"from",E,E,6,[[[T]],[T]]],[11,"into",E,E,6,[[["self"]],[U]]],[11,R[51],E,E,6,[[[U]],[R[47]]]],[11,R[55],E,E,6,[[["self"]],[T]]],[11,R[54],E,E,6,[[["self"]],[R[56]]]],[11,R[52],E,E,6,[[["self"]],[T]]],[11,R[53],E,E,6,[[["self"]],[R[47]]]],[11,R[49],R[57],E,7,[[["self"]],[T]]],[11,R[50],E,E,7,N],[11,"from",E,E,7,[[[T]],[T]]],[11,"into",E,E,7,[[["self"]],[U]]],[11,R[51],E,E,7,[[[U]],[R[47]]]],[11,R[55],E,E,7,[[["self"]],[T]]],[11,R[54],E,E,7,[[["self"]],[R[56]]]],[11,R[52],E,E,7,[[["self"]],[T]]],[11,R[53],E,E,7,[[["self"]],[R[47]]]],[11,R[49],E,E,8,[[["self"]],[T]]],[11,R[50],E,E,8,N],[11,"from",E,E,8,[[[T]],[T]]],[11,"into",E,E,8,[[["self"]],[U]]],[11,R[51],E,E,8,[[[U]],[R[47]]]],[11,R[55],E,E,8,[[["self"]],[T]]],[11,R[54],E,E,8,[[["self"]],[R[56]]]],[11,R[52],E,E,8,[[["self"]],[T]]],[11,R[53],E,E,8,[[["self"]],[R[47]]]],[11,"from",R[58],E,11,[[[T]],[T]]],[11,"into",E,E,11,[[["self"]],[U]]],[11,R[51],E,E,11,[[[U]],[R[47]]]],[11,R[55],E,E,11,[[["self"]],[T]]],[11,R[54],E,E,11,[[["self"]],[R[56]]]],[11,R[52],E,E,11,[[["self"]],[T]]],[11,R[53],E,E,11,[[["self"]],[R[47]]]],[11,R[49],E,E,9,[[["self"]],[T]]],[11,R[50],E,E,9,N],[11,"from",E,E,9,[[[T]],[T]]],[11,"into",E,E,9,[[["self"]],[U]]],[11,R[51],E,E,9,[[[U]],[R[47]]]],[11,R[55],E,E,9,[[["self"]],[T]]],[11,R[54],E,E,9,[[["self"]],[R[56]]]],[11,R[52],E,E,9,[[["self"]],[T]]],[11,R[53],E,E,9,[[["self"]],[R[47]]]],[11,R[49],E,E,10,[[["self"]],[T]]],[11,R[50],E,E,10,N],[11,"from",E,E,10,[[[T]],[T]]],[11,"into",E,E,10,[[["self"]],[U]]],[11,R[51],E,E,10,[[[U]],[R[47]]]],[11,R[55],E,E,10,[[["self"]],[T]]],[11,R[54],E,E,10,[[["self"]],[R[56]]]],[11,R[52],E,E,10,[[["self"]],[T]]],[11,R[53],E,E,10,[[["self"]],[R[47]]]],[11,"from",R[59],E,12,[[[T]],[T]]],[11,"into",E,E,12,[[["self"]],[U]]],[11,R[51],E,E,12,[[[U]],[R[47]]]],[11,R[55],E,E,12,[[["self"]],[T]]],[11,R[54],E,E,12,[[["self"]],[R[56]]]],[11,R[52],E,E,12,[[["self"]],[T]]],[11,R[53],E,E,12,[[["self"]],[R[47]]]],[11,"from",R[60],E,13,[[[T]],[T]]],[11,"into",E,E,13,[[["self"]],[U]]],[11,R[51],E,E,13,[[[U]],[R[47]]]],[11,R[55],E,E,13,[[["self"]],[T]]],[11,R[54],E,E,13,[[["self"]],[R[56]]]],[11,R[52],E,E,13,[[["self"]],[T]]],[11,R[53],E,E,13,[[["self"]],[R[47]]]],[11,"from",R[61],E,14,[[[T]],[T]]],[11,"into",E,E,14,[[["self"]],[U]]],[11,R[51],E,E,14,[[[U]],[R[47]]]],[11,R[55],E,E,14,[[["self"]],[T]]],[11,R[54],E,E,14,[[["self"]],[R[56]]]],[11,R[52],E,E,14,[[["self"]],[T]]],[11,R[53],E,E,14,[[["self"]],[R[47]]]],[11,"from",R[62],E,16,[[[T]],[T]]],[11,"into",E,E,16,[[["self"]],[U]]],[11,R[51],E,E,16,[[[U]],[R[47]]]],[11,R[55],E,E,16,[[["self"]],[T]]],[11,R[54],E,E,16,[[["self"]],[R[56]]]],[11,R[52],E,E,16,[[["self"]],[T]]],[11,R[53],E,E,16,[[["self"]],[R[47]]]],[11,R[49],E,E,15,[[["self"]],[T]]],[11,R[50],E,E,15,N],[11,"from",E,E,15,[[[T]],[T]]],[11,"into",E,E,15,[[["self"]],[U]]],[11,R[51],E,E,15,[[[U]],[R[47]]]],[11,R[55],E,E,15,[[["self"]],[T]]],[11,R[54],E,E,15,[[["self"]],[R[56]]]],[11,R[52],E,E,15,[[["self"]],[T]]],[11,R[53],E,E,15,[[["self"]],[R[47]]]],[11,"from",R[63],E,17,[[[T]],[T]]],[11,"into",E,E,17,[[["self"]],[U]]],[11,R[51],E,E,17,[[[U]],[R[47]]]],[11,R[55],E,E,17,[[["self"]],[T]]],[11,R[54],E,E,17,[[["self"]],[R[56]]]],[11,R[52],E,E,17,[[["self"]],[T]]],[11,R[53],E,E,17,[[["self"]],[R[47]]]],[11,R[49],E,E,22,[[["self"]],[T]]],[11,R[50],E,E,22,N],[11,"from",E,E,22,[[[T]],[T]]],[11,"into",E,E,22,[[["self"]],[U]]],[11,R[51],E,E,22,[[[U]],[R[47]]]],[11,R[55],E,E,22,[[["self"]],[T]]],[11,R[54],E,E,22,[[["self"]],[R[56]]]],[11,R[52],E,E,22,[[["self"]],[T]]],[11,R[53],E,E,22,[[["self"]],[R[47]]]],[11,"from",E,E,18,[[[T]],[T]]],[11,"into",E,E,18,[[["self"]],[U]]],[11,R[51],E,E,18,[[[U]],[R[47]]]],[11,R[55],E,E,18,[[["self"]],[T]]],[11,R[54],E,E,18,[[["self"]],[R[56]]]],[11,R[52],E,E,18,[[["self"]],[T]]],[11,R[53],E,E,18,[[["self"]],[R[47]]]],[11,"from",E,E,19,[[[T]],[T]]],[11,"into",E,E,19,[[["self"]],[U]]],[11,R[51],E,E,19,[[[U]],[R[47]]]],[11,R[55],E,E,19,[[["self"]],[T]]],[11,R[54],E,E,19,[[["self"]],[R[56]]]],[11,R[52],E,E,19,[[["self"]],[T]]],[11,R[53],E,E,19,[[["self"]],[R[47]]]],[11,"from",E,E,20,[[[T]],[T]]],[11,"into",E,E,20,[[["self"]],[U]]],[11,R[51],E,E,20,[[[U]],[R[47]]]],[11,R[55],E,E,20,[[["self"]],[T]]],[11,R[54],E,E,20,[[["self"]],[R[56]]]],[11,R[52],E,E,20,[[["self"]],[T]]],[11,R[53],E,E,20,[[["self"]],[R[47]]]],[11,"from",E,E,21,[[[T]],[T]]],[11,"into",E,E,21,[[["self"]],[U]]],[11,R[51],E,E,21,[[[U]],[R[47]]]],[11,R[55],E,E,21,[[["self"]],[T]]],[11,R[54],E,E,21,[[["self"]],[R[56]]]],[11,R[52],E,E,21,[[["self"]],[T]]],[11,R[53],E,E,21,[[["self"]],[R[47]]]],[11,R[64],R[58],E,10,[[["self"],[R[4]]],[R[5],[R[65]]]]],[11,"lt",E,E,10,[[["self"],[R[4]]],["bool"]]],[11,"le",E,E,10,[[["self"],[R[4]]],["bool"]]],[11,"gt",E,E,10,[[["self"],[R[4]]],["bool"]]],[11,"ge",E,E,10,[[["self"],[R[4]]],["bool"]]],[11,R[64],R[63],E,22,[[["self"],[R[28]]],[R[5],[R[65]]]]],[11,"lt",E,E,22,[[["self"],[R[28]]],["bool"]]],[11,"le",E,E,22,[[["self"],[R[28]]],["bool"]]],[11,"gt",E,E,22,[[["self"],[R[28]]],["bool"]]],[11,"ge",E,E,22,[[["self"],[R[28]]],["bool"]]],[11,"eq",R[57],E,8,[[["self"],[R[68]]],["bool"]]],[11,"eq",E,E,7,[[["self"],[R[66]]],["bool"]]],[11,"ne",E,E,7,[[["self"],[R[66]]],["bool"]]],[11,"eq",R[58],E,9,[[["self"],[R[9]]],["bool"]]],[11,"ne",E,E,9,[[["self"],[R[9]]],["bool"]]],[11,"eq",E,E,10,[[["self"],[R[4]]],["bool"]]],[11,"ne",E,E,10,[[["self"],[R[4]]],["bool"]]],[11,"eq",R[62],E,15,[[["self"],[R[22]]],["bool"]]],[11,"ne",E,E,15,[[["self"],[R[22]]],["bool"]]],[11,"eq",R[63],E,17,[[["self"],[R[8]]],["bool"]]],[11,"ne",E,E,17,[[["self"],[R[8]]],["bool"]]],[11,"eq",E,E,18,[[["self"],[R[48]]],["bool"]]],[11,"eq",E,E,22,[[["self"],[R[28]]],["bool"]]],[11,"ne",E,E,22,[[["self"],[R[28]]],["bool"]]],[11,"eq",E,E,19,[[["self"],[R[67]]],["bool"]]],[11,"eq",E,E,20,[[["self"],[R[45]]],["bool"]]],[11,"eq",E,E,21,[[["self"],[R[46]]],["bool"]]],[11,"cmp",R[58],E,10,[[["self"],[R[4]]],[R[65]]]],[11,"cmp",R[63],E,22,[[["self"],[R[28]]],[R[65]]]],[11,"from",R[0],E,5,[[["vector4"]],["self"]]],[11,"from",E,E,3,[[["color"]],["self"]]],[11,"clone",R[57],E,8,[[["self"]],[R[68]]]],[11,"clone",E,E,7,[[["self"]],[R[66]]]],[11,"clone",R[58],E,9,[[["self"]],[R[9]]]],[11,"clone",E,E,10,[[["self"]],[R[4]]]],[11,"clone",R[62],E,15,[[["self"]],[R[22]]]],[11,"clone",R[63],E,22,[[["self"]],[R[28]]]],[11,"clone",R[0],E,0,[[["self"]],[R[10]]]],[11,"clone",E,E,1,[[["self"]],[R[14]]]],[11,"clone",E,E,2,[[["self"]],[R[7]]]],[11,"clone",E,E,3,[[["self"]],["vector4"]]],[11,"clone",E,E,4,[[["self"]],["vertex"]]],[11,"clone",E,E,5,[[["self"]],["color"]]],[11,"clone",E,E,6,[[["self"]],[R[11]]]],[11,"drop",R[58],E,11,[[["self"]]]],[11,"drop",R[60],E,13,[[["self"]]]],[11,"drop",R[61],E,14,[[["self"]]]],[11,"drop",R[62],E,16,[[["self"]]]],[11,"drop",R[63],E,17,[[["self"]]]],[11,"extend",R[58],E,10,[[["self"],[T]]]],[11,"extend",R[63],E,22,[[["self"],[T]]]],[11,"hash",R[58],E,10,N],[11,"hash",R[63],E,22,N],[11,"sub",R[58],R[69],10,[[["self"],[R[4]]],[R[4]]]],[11,"sub",R[63],R[69],22,[[["self"],[R[28]]],[R[28]]]],[11,R[70],R[58],R[71],10,[[["self"],[R[4]]]]],[11,R[70],R[63],R[71],22,[[["self"],[R[28]]]]],[11,"not",R[58],R[72],10,[[["self"]],[R[4]]]],[11,"not",R[63],R[72],22,[[["self"]],[R[28]]]],[11,"bitand",R[58],R[73],10,[[["self"],[R[4]]],[R[4]]]],[11,"bitand",R[63],R[73],22,[[["self"],[R[28]]],[R[28]]]],[11,"bitor",R[58],R[74],10,[[["self"],[R[4]]],[R[4]]]],[11,"bitor",R[63],R[74],22,[[["self"],[R[28]]],[R[28]]]],[11,"bitxor",R[58],R[75],10,[[["self"],[R[4]]],[R[4]]]],[11,"bitxor",R[63],R[75],22,[[["self"],[R[28]]],[R[28]]]],[11,R[76],R[58],R[77],10,[[["self"],[R[4]]]]],[11,R[76],R[63],R[77],22,[[["self"],[R[28]]]]],[11,R[78],R[58],R[79],10,[[["self"],[R[4]]]]],[11,R[78],R[63],R[79],22,[[["self"],[R[28]]]]],[11,R[80],R[58],R[81],10,[[["self"],[R[4]]]]],[11,R[80],R[63],R[81],22,[[["self"],[R[28]]]]],[11,R[82],R[58],E,10,[[[T]],[R[4]]]],[11,R[82],R[63],E,22,[[[T]],[R[28]]]],[11,"fmt",R[57],E,8,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,7,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[58],E,9,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,10,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[62],E,15,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[63],E,17,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,18,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,22,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,19,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,20,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,21,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[0],E,0,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,1,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,2,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,3,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,4,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,5,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",E,E,6,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[58],E,10,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[63],E,22,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[58],E,10,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[63],E,22,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[58],E,10,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[63],E,22,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[58],E,10,[[["self"],[R[83]]],[R[47]]]],[11,"fmt",R[63],E,22,[[["self"],[R[83]]],[R[47]]]],[11,R[84],R[57],E,8,[[["i64"]],[R[5]]]],[11,R[85],E,E,8,[[["u64"]],[R[5]]]],[11,R[84],R[63],E,18,[[["i64"]],[R[5]]]],[11,R[85],E,E,18,[[["u64"]],[R[5]]]],[11,R[84],E,E,19,[[["i64"]],[R[5]]]],[11,R[85],E,E,19,[[["u64"]],[R[5]]]],[11,R[84],E,E,20,[[["i64"]],[R[5]]]],[11,R[85],E,E,20,[[["u64"]],[R[5]]]],[11,R[84],E,E,21,[[["i64"]],[R[5]]]],[11,R[85],E,E,21,[[["u64"]],[R[5]]]],[11,R[84],R[0],E,6,[[["i64"]],[R[5]]]],[11,R[85],E,E,6,[[["u64"]],[R[5]]]]],"p":[[3,R[86]],[3,R[87]],[3,"Vector2"],[3,"Vector4"],[3,"Vertex"],[3,"Color"],[4,R[88]],[3,R[89]],[4,R[90]],[3,R[91]],[3,R[92]],[3,R[93]],[3,R[94]],[3,R[95]],[3,"Shader"],[3,R[96]],[3,R[97]],[3,"Texture"],[4,R[98]],[4,R[99]],[4,R[100]],[4,R[101]],[3,R[102]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);