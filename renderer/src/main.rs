fn main() {
    //visual render start loop
    //storing size as max width (x) and max length (y), should return as number of pixels
    let mut fov: f32 = 70.0; //angle of sight
    let mut screensize: [u64; 2] = getscreensize(); //chashes the current size should only run if window is changed,
    let mut basecompare: [u64; 2] = setcentercompare(screensize); //finds the center of x and y to determine angle displacment later, should only run if window is changed
    let mut windowart: [ [ [u64; 4]; screensize[0] ]; screensize[1] ]; //the pixels colors for a 2D grid in an array
    let i: u64 = 0;  //for x for loop
    for _ in 0..screensize[0] {
        //code
        let n: u64 = 0;  //for y for loop
        for _ in 0..screensize[1] {
            //code
            let angledisplacmenthorizontil: i64 = getangledisplacment(distacefromcenter); //dosn't need to be mutable because the life time will run out at end of loop
            let angledisplacmentvertical: i64 = getangledisplacment(distacefromcenter);
            //itarate n for y value
            n += 1;
        }
        //iterate i for x value
        i += 1;
    }
}

//gets the screen size
fn getscreensize() -> [u64; 2] { //returns x and y size as and array of intagers
    
}

fn setcentercompare(screensize: [u64; 2]) -> [u64; 2] {

}

fn getangledisplacment(distacefromcenter: u64) -> i64 {

}

fn raymarch(anglehorizontile: i64, anglevertical: i64) -> [u64; 4] { //returns the rgba color value of pixle

}