struct Camera {
    position_x: f64, //where it is on x-axis
    position_y: f64, //where it is on y-axis
    position_z: f64, //where it is in 3D space - set to 0 for 2D game
    horizontal: f64, //horizontal angle - set to 0 for 2D game
    vertical_a: f64, //vertical angle - set to 0 for 2D game
    camera_fov: f32, //the angle of max sight of total view
    cam_f_stop: u64, //saturation of color
}

struct Screen {
    width: u64,
    height: u64,
    display: [ [ [u8; 3]; self.width ]; self.height],
}

fn main() {
    //sets the screen demenchions and stores 2D array of colors of pixels on the screen
    screen = Screen {
        width: 0,
        height: 0,
        display: 0,
    }; //top left is 0,0
    //set up the camera
    camera = Camera {
        position_x: 0.0,
        position_y: 0.0,
        position_z: 0.0,
        horizontal: 0.0,
        vertical_a: 0.0,
        camera_fov: 70.0,
        cam_f_stop: 0,
    };
    //For each Y pixel
        //for each X pixel
            //find displacment angle for the ray to be cast utilizing FOV
            //Ray march to find color in 3D space and return RGB value to the display in memory -- struct Screen
}

//ray march function entry point requireing position in 2D optional Z and optional angle (if doing 3D, requires angle)
fn raymarch(posx: u64, posy: u64, posz: u64, angv: f64, angh: f64) -> [u8; 3] {
    //loop until distance to object unknown object is x < 0.1*10^-5 (really small number)
        //find distance to closest object
        //cast a ray that distance
    //get RGB value from object at the position
    //find nearest light sources & main sources -- like a sun or nuke
    //do prev loop that should be a funtion of it's own
    //if return pos isn't within 2% of light source position then darken by calculated amount based on brightness of source
    //else saturate and apply approprite color changes based on source
    //return the rbg value
}