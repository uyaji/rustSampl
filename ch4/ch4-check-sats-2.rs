#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(
    sat_id: CubeSat
) -> StatusMessage {
    StatusMessage::Ok
}

impl Copy for CubeSat {}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {
    let sat_a = CubeSat{ id: 0 };
    let sat_b = CubeSat{ id: 1 };
    let sat_c = CubeSat{ id:  2};

    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
    //
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
