use std::path::Path;
use rocksdb::{DB, Options};


pub trait AccessDatabase<Key, Value> {
    type Database;

    fn open(path: String) -> Self::Database;

    fn get_value(&mut self, key: &Key) -> Result<Value,()>;

    fn put(&mut self, key: &Key, value: &Value) -> Result<(),()>;

    fn delete_key(&mut self, key: &Key) -> Result<Value,()>;
}


impl<Key, Value> AccessDatabase<Key, Value> for DB
where
    Key: AsRef<[u8]> + From<Vec<u8>>,
    Value: AsRef<[u8]> + From<Vec<u8>>,
{
    type Database = DB;

    fn open(path: String) -> Self::Database {
        DB::open_default(path).unwrap()
    }

    fn get_value(&mut self, key: &Key) -> Result<Value, ()> {
        match self.get(&key){
            Ok(Some(value)) => Ok(value.into()),
            Ok(None) => Ok(vec!().into()),
            Err(e) => Err(println!("{:?}", e)),
        }
    }

    fn put(&mut self, key: &Key, value: &Value) -> Result<(), ()> {
        match self.put(key, value){
            Ok(_) => Ok(()),
            Err(e) => Err(println!("{:?}", e)),
        }
    }

    fn delete_key(&mut self, key: &Key) -> Result<Value, ()> {
        let value = self.get(key);
        match self.delete(key){
            Ok(_) => match value {
                Ok(Some(value)) => Ok(value.into()),
                _ => Err(())
            },
            Err(e) => Err(println!("{:?}", e)),
        }
    }
}


#[test]
fn rocksdb_test(){
    use rocksdb::{DB, Options};
    let path = ".";
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key"){
            Ok(Some(value)) => println!("Retrieved {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("empty value"),
            Err(e) => println!("{:?}", e),
        }
        db.delete(b"my key").unwrap()
    }
    let _ = DB::destroy(&Options::default(), path);
}