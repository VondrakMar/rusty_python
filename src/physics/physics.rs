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
    let m1 = 1;
    let m2 = 1;
    let mut f_x = 0;
    let mut f_y = 0;
    let r_x = (planet1.head.x()-planet2.head.x());
    let r_y = (planet1.head.y()-planet2.head.y());
    let force_constant = 2;
    if r_x != 0{
        f_x = force_constant*(m1*m2)/r_x;
    }
    if r_y != 0{
        f_y = force_constant*(m1*m2)/r_y;
    }
    println!("{f_x} {f_y}");
    planet1.v_x += f_x;
    planet1.v_y += f_y;
}
