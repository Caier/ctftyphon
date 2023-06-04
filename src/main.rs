use std::io::Read;

use itertools::Itertools;

fn main() {
    let file = std::fs::File::open("./f.zip").unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    let names: Vec<_> = zip.file_names().map(|n| n.to_owned()).collect();
    let perms = names.iter()
        .map(|n| &n[6..])
        .filter(|n| n.len() == 5)
        .permutations(5)
        .map(|p| p.iter().join("").into_bytes());
    
    let mut i = 0;
    for perm in perms {
        let dec = zip.by_name_decrypt("files/!   cGVybXV0NSxuby1kdXBs", &perm).unwrap();

        if let Ok(mut f) = dec {
            let mut s = String::new();
            let res = f.read_to_string(&mut s);
            if let Err(e) = res {
                if e.to_string() == "Invalid checksum" {
                    continue;
                }
                panic!("{}", e);
            }
            println!("SUKCES: {:?}, cont: {:?}", String::from_utf8(perm), s);
            break;
        }

        print!("\r{i}");
        i += 1;
    }
}