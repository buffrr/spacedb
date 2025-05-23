use spacedb::{Result, db::Database };
use spacedb::tx::ProofType;

fn main() -> Result<()> {
    let db = Database::memory()?;

    // Insert some data
    let mut tx = db.begin_write()?;
    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        tx = tx.insert(db.hash(key.as_bytes()), value.into_bytes())?;
    }
    tx.commit()?;

    // Get the committed snapshot
    let mut snapshot = db.begin_read()?;
    println!("Tree root: {}", hex::encode(snapshot.compute_root()?));

    // Prove a subset of the keys
    let keys_to_prove: Vec<_> = (0..10)
        .map(|i| format!("key{}", i))
        // prove exclusion of some other keys
        .chain((0..5).map(|i| format!("other{}", i)))
        .map(|key| db.hash(key.as_bytes()))
        .collect();

    // reveal the relevant nodes needed to prove the specified set of keys
    let subtree = snapshot.prove(&keys_to_prove, ProofType::Standard)?;

    // Will have the exact same root as the snapshot
    println!("Subtree root: {}", hex::encode(subtree.compute_root().unwrap()));

    // Prove inclusion
    assert!(subtree.contains(&db.hash("key0".as_bytes())).unwrap());

    // Prove exclusion
    assert!(!subtree.contains(&db.hash("other0".as_bytes())).unwrap());

    // We don't have enough data to prove key "other100" is not in the subtree
    // as the relevant branches needed to prove it are not included
    assert!(subtree.contains(&db.hash("other100".as_bytes())).is_err());
    Ok(())
}
