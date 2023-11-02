use chrono::Utc;
use tarantool::{proc, space::Space};
use nanoid::nanoid;

#[proc]
fn start() -> Result<bool, String> {
    let start_time = Utc::now().time();
    
    let space = Space::find("file").expect("Can't find space plan_item");
    let row = (nanoid!(), 0, "file.png");
    space.replace(&row).expect("Error insert file");
    
    let ms = (Utc::now().time() - start_time).num_milliseconds();
    println!("Time in ms: {:?}", ms);
    Ok(true)
}
