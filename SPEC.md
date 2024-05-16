## Ray Marching

### Language / Libraries
* Rust
* egui
* GLSL
* OpenGL

### MoSCoW

#### Must
* Resizable window
* Spheres
Render a scene with at least one sphere
* Render objects of different color
* Basic shading (create a 3D feel)
Shading based on single light source (like a "sun") creating basic shading on objects. 
* Easy to run on any machine that supports OpenGL
should be able to compile with cargo, run the program with `cargo run`
* Real-time performance
minimum 60fps

#### Should
* Render arbitrary number of objects
Render multiple objects at a time, dynamically change number of objects in the scene (deletion and instertion of objects).
* Move objects in scene
Support for moving object around in code
* Triangles
Render scene with arbitrary number of triangles
* Import 3D models
Import .obj file 
* Movement controls
Allow user to move the camera in the scene (similar to Blender/CAD program)
* FPS counter
display FPS on screen

#### Could
* GUI elements
User can spawn/delete object by interacting with a GUI. Buttons/Text fields
* User-Object interaction
User can move objects in scene by using a mouse
* Skybox
Use a image as a background texture
* Textures
Apply arbitrary textures for objects
* Render Animations
Render a specified scene as a gif. Define camera movement, object movement etc.
* Shadows
Objects cast shadows on each other
* Different rendering "modes"
Normal ray-marching, intersection rendering, glow effect etc.


#### Would
* Volumetric fog
(CS2 smoke)
* Render higher resolution + post-processing effects
Make post-processing effects for cleaner renders

### Workflow
git
* Master branch with working product
* Development branches

Keep it simple since we are only two people on the team. 

### Work Distribution

Since the scope of project is narrow, it is difficult to generalize the distribution of work. Instead, tasks will be divided as they appear. This allows for improved flexibility in the sense that whichever person has more experience with a specific subject may choose to work on that part of the project. 

