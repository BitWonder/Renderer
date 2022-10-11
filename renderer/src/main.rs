struct Camera {
    position_x: f64, //where it is on x-axis
    position_y: f64, //where it is on y-axis
    position_z: f64, //where it is in 3D space - set to 0 for 2D game
    horizontal: f64, //horizontal angle - set to 0 for 2D game
    vertical_a: f64, //vertical angle - set to 0 for 2D game
    camera_fov: f64, //the angle of max sight of total view
    cam_f_stop: f64, //saturation of color
}

fn main() {
    //see if screen size has changed and change num of drawn pixels
    screensize = getscreensize(); //top left is 0,0
    //get camera pos and angle
    camera = getcamera();
}

fn getscreensize() -> [u64; 2] {

}

fn getcamera() -> [u64; 6] { //returns (in order) camera pos x, y, z, and camera angle horizonitle, vertical, camera FOV

}