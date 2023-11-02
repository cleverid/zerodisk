use chrono::Utc;
use tarantool::proc;

#[proc]
fn start() -> Result<bool, String> {
    let start_time = Utc::now().time();
    let ms = (Utc::now().time() - start_time).num_milliseconds();
    println!("Time in ms: {:?}", ms);
    Ok(true)
}
