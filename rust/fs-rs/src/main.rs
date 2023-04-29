use uuid::Uuid;
use std::collections::HashMap;
use std::{thread, time};
use rand::{thread_rng, Rng};


fn main() {
    let mut rng = thread_rng();
    let mut feats: HashMap<String, f32> = HashMap::new();

    for _x in 1..80_000_000 {
        let id = Uuid::new_v4().to_string();
        let value = rng.gen::<f32>();
        feats.insert(id, value);
    }

    // for (id, value) in &feats {
    //     println!("{}, {}", id, value)
    // }

    println!("Done.");

    thread::sleep(time::Duration::from_secs(60*10));

    // println!("UUID: {}", id);
}
