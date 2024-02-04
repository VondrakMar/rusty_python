use crate::graphcs::models::SpaceObject;

// pub fn force_f64(planet1:&mut Python,planet2:&mut Python){
//     let m1 = 1.0;
//     let m2 = 1.0;
//     let f_x = (m1*m2)/(planet1.head.x() as f64-planet2.head.x() as f64);
//     let f_y = (m1*m2)/(planet1.head.y() as f64-planet2.head.y() as f64);
//     println!("{f_x} {f_y}");
//     planet1.v_x -= f_x;
//     planet1.v_y -= f_y;
// }

pub fn force(planet1:&mut SpaceObject,planet2:&mut SpaceObject){
    let mut f_x = 0.0;
    let mut f_y = 0.0;
    let r_x = planet1.react_x-planet2.react_x;
    let r_y = planet1.react_y-planet2.react_y;
    let mut d2 = r_x.powi(2) + r_y.powi(2);
    let force_constant = 10.0;
    let min_distance_squared = 1e-10;
    if d2 < 10.0 {d2 = 10.0}
    // println!("d2: {d2}");
    if d2 > min_distance_squared{
        let f = force_constant * (planet1.m * planet2.m) / d2; 
        let d = d2.sqrt(); 
        f_x = f * r_x / d;
        f_y = f * r_y / d;
    }
    // println!("{f_x} {f_y}");
    planet1.v_x -= f_x/planet1.m;
    planet2.v_x += f_x/planet2.m;
    planet1.v_y -= f_y/planet1.m;
    planet2.v_y += f_y/planet2.m;
}
