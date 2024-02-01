use crate::graphcs::models::Python;

// pub fn force_f64(planet1:&mut Python,planet2:&mut Python){
//     let m1 = 1.0;
//     let m2 = 1.0;
//     let f_x = (m1*m2)/(planet1.head.x() as f64-planet2.head.x() as f64);
//     let f_y = (m1*m2)/(planet1.head.y() as f64-planet2.head.y() as f64);
//     println!("{f_x} {f_y}");
//     planet1.v_x -= f_x;
//     planet1.v_y -= f_y;
// }

pub fn force(planet1:&mut Python,planet2:&mut Python){
    let m1 = 1.0;
    let m2 = 1.0;
    let mut f_x = 0.0;
    let mut f_y = 0.0;
    let r_x = planet1.react_x-planet2.react_x;
    let r_y = planet1.react_y-planet2.react_y;
    let d2 = r_x.powi(2) + r_y.powi(2);
    let force_constant = 2.0;
    let min_distance_squared = 1e-10;

    if d2 > min_distance_squared{
        println!("Force activated");
        let f = force_constant * (m1 * m2) / d2; 
        let d = d2.sqrt(); 
        f_x = f * r_x / d;
        println!("f_x: {f_x}");
        f_y = f * r_y / d;
        println!("f_y: {f_y}"); 
        // f_x = force_constant*(m1*m2)/r_x;
        // f_y = force_constant*(m1*m2)/r_y;
    }
    println!("{f_x} {f_y}");
    planet1.v_x -= f_x;
    planet1.v_y -= f_y;
}
