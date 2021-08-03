use std::{
    collections::HashSet,
    error::Error,
    sync::{Arc, RwLock},
};

fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["柴", "トイプードル"].iter().cloned().collect();
    // RwLock: mutable, Arc: share between thread
    let dogs = Arc::new(RwLock::new(dogs));

    fn stringify(x: impl ToString) -> String {
        x.to_string()
    }

    {
        let ds = dogs.read().map_err(stringify)?;
        assert!(ds.contains("柴"));
        assert!(ds.contains("トイプードル"));
    }

    dogs.write().map_err(stringify)?.insert("ブル・テリア");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move || {
        dogs1
            .write()
            .map(|mut ds| ds.insert("コーギー"))
            .map_err(stringify)
    })
    .join()
    .expect("Thread error")?;

    // cannot read when write lock exists
    // let temp = dogs.write();
    // assert!(dogs.try_read().map_err(stringify)?.contains("ブル・テリア"));

    assert!(dogs.read().map_err(stringify)?.contains("ブル・テリア"));
    assert!(dogs.read().map_err(stringify)?.contains("コーギー"));
    Ok(())
}
