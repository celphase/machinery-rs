use machinery_api::foundation::Vec4T;
use ultraviolet::Rotor3;

pub fn quaternion_to_rotor(input: Vec4T) -> Rotor3 {
    Rotor3::from_quaternion_array([input.x, input.y, input.z, input.w])
}

pub fn rotor_to_quaternion(input: Rotor3) -> Vec4T {
    let array = input.into_quaternion_array();
    Vec4T {
        x: array[0],
        y: array[1],
        z: array[2],
        w: array[3],
    }
}
