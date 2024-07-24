use fake::{faker::name::en::Name, Fake};
use mysql::{prelude::Queryable, Pool, TxOpts};
use sha2::{Digest, Sha512};

const LOOP: usize = 1000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Establish the connection
    let url = "mysql://root:root@localhost/test";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let types = vec!["example_type", "another_type", "yet_another_type"];

    for i in 0..LOOP {
        // Define your bulk data
        let data_to_insert = (0..300)
            .map(|_| {
                let name: String = Name().fake();
                let key = create_hash(name.as_str(), HashType::MD5);
                (key, name, types[i % 3])
            })
            .collect::<Vec<_>>();

        // Prepare the SQL query
        let insert_query = "INSERT IGNORE INTO table_normal (id, name, type) VALUES (?, ?, ?)";
        let insert_query2 =
            "INSERT IGNORE INTO table_partitioned (id, name, type) VALUES (?, ?, ?)";

        // Use a transaction to ensure that all inserts are atomic
        let mut transaction = conn.start_transaction(TxOpts::default())?;
        for data in data_to_insert {
            transaction.exec_drop(insert_query, &data)?;
            transaction.exec_drop(insert_query2, &data)?;
        }
        transaction.commit()?;

        println!("Data inserted successfully! Loop: {}", i);
    }

    println!("Data inserted successfully!");

    Ok(())
}

enum HashType {
    MD5,
    SHA512,
}

fn create_hash(name: &str, hash_type: HashType) -> String {
    match hash_type {
        HashType::MD5 => create_md5(name),
        HashType::SHA512 => create_sha512(name),
    }
}

fn create_md5(name: &str) -> String {
    format!("{:x}", md5::compute(name.as_bytes()))
}

fn create_sha512(name: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(name.as_bytes());
    format!("{:x}", hasher.finalize())
}
