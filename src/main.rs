use std::{
    env::args_os,
    fs::{create_dir, File},
    io::{Read, Result, Write},
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

fn backup_dir_all(path: PathBuf, from: String, to: String) -> Result<()> {
    let backup_path = PathBuf::from(path.to_string_lossy().replacen(&from, &to, 1));

    //  Result isn't necessary
    drop(create_dir(backup_path.clone()));

    for rde in path.read_dir()? {
        let de = rde?;

        //  Recurse if directory
        if de.file_type()?.is_dir() {
            backup_dir_all(de.path(), from.clone(), to.clone())?
        } else {
            let mut from = File::open(de.path())?;
            let mut to = File::create({
                let mut path = backup_path.clone().to_owned();
                path.push(de.file_name());
                path
            })?;
            let mut buf = Vec::new();
            from.read_to_end(&mut buf)?;
            to.write_all(&buf)?
        }
    }
    Ok(())
}

fn main() {
    if let Some(path) = args_os().nth(1) {
        //  Provided directory
        let path = PathBuf::from(path);

        if path.is_dir() {
            //  Default duration of `Timer` cycle is 10 minutes
            const DUR: Duration = Duration::from_secs(600);
            let from = path.clone().to_string_lossy().to_string();
            let to = {
                let mut path = from.clone().to_owned();
                path.push_str(".backup");
                path
            };

            loop {
                //  Attempt backup process
                if let Err(error) = backup_dir_all(path.clone(), from.clone(), to.clone()) {
                    println!("{:?}", error)
                }
                //  Timer
                sleep(DUR)
            }
        }
    } else {
        println!("Missing argument")
    }
}
