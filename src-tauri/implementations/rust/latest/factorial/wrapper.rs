mod factorial;
extern crate time;
extern crate sys_info;
use factorial::factorial;

fn main() {
    let start_time = time::precise_time_ns();
    let result = factorial(4);
    let end_time = time::precise_time_ns();
    let elapsed_time = (end_time - start_time) as f64 / 1_000_000_000.0;

    let mem_info = sys_info::mem_info().unwrap();
    let mem_usage = mem_info.total - mem_info.free;

    println!("{:?}", result);println!("{}", format!("{:.3}", elapsed_time));println!("{}", format!("{:.1}", mem_usage as f64 / 1024.0 / 1024.0));}