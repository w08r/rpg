use crate::reader::Reader;
pub struct PostgresOpts<'a> {
    pub dbname: &'a str
}

pub fn upload_to_postgresql(_reader: Reader, opts: PostgresOpts) -> () {
    println!("Writing data to {}", opts.dbname)
}
