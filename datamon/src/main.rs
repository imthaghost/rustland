// internal imports
use std::io::prelude::*;
use std::io::Cursor;
// external imports
use polars::prelude::*;
use rand::Rng;


// create a named pipe
fn mkpipe(name: String) -> String {
    let prefix = "/tmp/datamon/";
    let path = format!("{}{}", prefix, name);
    std::fs::create_dir_all(prefix)
        .expect(&format!("Error: couldn't create the folder {}", prefix));
    let filename = std::ffi::CString::new(path.clone()).unwrap();
    unsafe { libc::mkfifo(filename.as_ptr(), 0o600); }
    return path
}

// write an entire buffer to a file
fn tofile(mut f:&std::fs::File, data:&[u8]) {
    f.write_all(data)
    .expect("Error: couldn't write the named pipe");
}

// create a random data frame
fn mkframe() -> DataFrame {
    let mut rng = rand::thread_rng();

    // Create vectors with random data
    let a_values = vec![rng.gen::<u32>(), rng.gen::<u32>(), rng.gen::<u32>()];
    let b_values = vec![rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()];

    // Use new_from_vec to create ChunkedArrays
    let a = UInt32Chunked::from_vec("a", a_values).into_series();
    let b = Float64Chunked::from_vec("b", b_values).into_series();

    DataFrame::new(vec![a, b]).unwrap()
}


fn main() {
    
    // check if named pipe is already created otherwise create
    let path:String = mkpipe("data".to_string());
    // open fifo
    let file = std::fs::OpenOptions::new().write(true).open(path)
      .expect("Error: couldn't open the named pipe");
    // loop
    let mut i = 0;
    while i < 100 {
        println!("Count: {}", i);    
        let mut df = mkframe();
        // create in memory buffer
        let mut buf:Cursor<Vec<u8>> = Cursor::new(Vec::new());
        // write to the in memory buffer
        IpcWriter::new(&mut buf).finish(&mut df)
            .expect("ipc writer");

        println!("{:?}", df);
        tofile(&file, buf.get_ref());
        
        let waitabit = std::time::Duration::from_millis(30);
        std::thread::sleep(waitabit);   
        
        i = i + 1;
    }
 
    
}

