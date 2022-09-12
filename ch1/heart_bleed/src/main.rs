fn main() {
    let user1 ="";
    let user2 ="";
    let buffer = &mut[0u8; 1024];
    read_secrets(&user1, buffer);
    store_secrets(buffer);

    read_secrets(&user2, buffer);
    store_secrets(buffer);
    println!("Hello, world!");
}

fn read_secrets(_user: &str, _buffer: &mut[u8; 1024]) {
}

fn store_secrets(_buffer: &mut[u8; 1024]) {
}
