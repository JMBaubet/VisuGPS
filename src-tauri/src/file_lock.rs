use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOCKS: Mutex<HashMap<PathBuf, Arc<Mutex<()>>>> = Mutex::new(HashMap::new());
}

/// Obtient un verrou (MutexGuard) pour un chemin de fichier donné.
/// Le verrou est maintenu tant que la garde retournée est en vie.
pub fn get_lock(path: PathBuf) -> Arc<Mutex<()>> {
    let mut locks = LOCKS.lock().unwrap();
    locks.entry(path).or_insert_with(|| Arc::new(Mutex::new(()))).clone()
}
