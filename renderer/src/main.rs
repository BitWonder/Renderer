struct Camera {
    position_x: f64, //where it is on x-axis
    position_y: f64, //where it is on y-axis
    position_z: f64, //where it is in 3D space - set to 0 for 2D game
    horizontal: f64, //horizontal angle - set to 0 for 2D game
    vertical_a: f64, //vertical angle - set to 0 for 2D game
    camera_fov: f64, //the angle of max sight of total view
    cam_f_stop: u64, //saturation of color
}

struct Screen {
    width: u64,
    height: u64,
    display: [ [ [f32; 3]; self.width ]; self.height],
}

fn main() {
    //see if screen size has changed and change num of drawn pixels
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
}