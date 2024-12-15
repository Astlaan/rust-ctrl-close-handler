use ctrlc;
use std::fs::File;
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

// fn register_ctrl_handler(exit_flag: Arc<AtomicBool>) {
//     ctrlc::set_handler(move || {
//         println!("CTRL+C detected. Handler started...");
//         let handler_start_time = std::time::Instant::now();

//         for i in 0..5 {
//             let elapsed_secs = handler_start_time.elapsed().as_secs();
//             let file_name = format!("m_{}.txt", elapsed_secs);
//             let mut file = File::create(&file_name).expect("Unable to create handler file");
//             writeln!(file, "Handler elapsed seconds: {}", elapsed_secs)
//                 .expect("Unable to write to handler file");
//             println!("Handler saved file: {}", file_name);
//             thread::sleep(Duration::from_secs(1));
//         }

//         exit_flag.store(true, Ordering::SeqCst);
//         println!("Handler finished execution.");
//     })
//     .expect("Error setting up CTRL+C handler");
// }

// fn main() {
//     let start_time = std::time::Instant::now();
//     let exit_flag = Arc::new(AtomicBool::new(false));
//     register_ctrl_handler(Arc::clone(&exit_flag));

//     while !exit_flag.load(Ordering::SeqCst) {
//         let elapsed_secs = start_time.elapsed().as_secs();
//         let file_name = format!("{}.txt", elapsed_secs);
//         let mut file = File::create(&file_name).expect("Unable to create file");
//         writeln!(file, "Elapsed seconds: {}", elapsed_secs).expect("Unable to write to file");
//         println!("Saved file: {}", file_name);
//         thread::sleep(Duration::from_secs(1));
//     }

//     println!("Program exited gracefully.");
// }

use winapi::shared::minwindef::{BOOL, TRUE};
use winapi::um::consoleapi::SetConsoleCtrlHandler; // SetConsoleCtrlHandler is in consoleapi module
use winapi::um::wincon::PHANDLER_ROUTINE; // PHANDLER_ROUTINE is in minwinbase module // This one was correct

unsafe extern "system" fn ctrl_handler(_ctrl_type: u32) -> BOOL {
    println!("CTRL+C detected. Handler started...");
    let handler_start_time = std::time::Instant::now();

    for i in 0..5 {
        let elapsed_secs = handler_start_time.elapsed().as_secs();
        let file_name = format!("m_{}.txt", elapsed_secs);
        if let Ok(mut file) = File::create(&file_name) {
            writeln!(file, "Handler elapsed seconds: {}", elapsed_secs).ok();
            println!("Handler saved file: {}", file_name);
        }
        thread::sleep(Duration::from_secs(1));
    }

    println!("Handler finished execution.");
    TRUE
}

fn register_ctrl_handler(_exit_flag: Arc<AtomicBool>) {
    unsafe {
        if SetConsoleCtrlHandler(Some(ctrl_handler), TRUE) == 0 {
            panic!("Error setting up CTRL+C handler");
        }
    }
}

fn main() {
    let start_time = std::time::Instant::now();
    let exit_flag = Arc::new(AtomicBool::new(false));
    register_ctrl_handler(Arc::clone(&exit_flag));

    while !exit_flag.load(Ordering::SeqCst) {
        let elapsed_secs = start_time.elapsed().as_secs();
        let file_name = format!("{}.txt", elapsed_secs);
        let mut file = File::create(&file_name).expect("Unable to create file");
        writeln!(file, "Elapsed seconds: {}", elapsed_secs).expect("Unable to write to file");
        println!("Saved file: {}", file_name);
        thread::sleep(Duration::from_secs(1));
    }

    println!("Program exited gracefully.");
}
