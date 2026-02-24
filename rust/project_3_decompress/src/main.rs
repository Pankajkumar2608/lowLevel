
use::std::io;
use::std::fs;
// decompression code 

fn main () {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args:Vec<_> = std::env::args().collect();
    if (args.len() != 3 ){
        eprintln!("Usage : {} <filename>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = !file.comment();
            if( !comment.is_empty(){
                prinln!("File {} comment: {}", i , comment);
            })
        }
        if(*file.name()).ends_with("/"){
            prinln!("File {} extracted to \"{}\"", i , output.display());
            fs::create_dir_all(&output).unwrap();
        }else{
            println!("file {} extracted to \"{}\" ({}bytes)",i , output.display(), file.size)
            if let some(p) = outpath.parent(){
                if !p.exists(){
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        };
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionExt;
            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            };
        };
    };
};