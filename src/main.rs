mod models {
    pub mod file;
    pub mod replica;
}

use models::file::File;
use models::replica::Replica;

fn main() {
    let new_file = File {
        id: 1,
        filepath: "path/to/file".to_string(),
        size: 100,
        file_date: chrono::Utc::now().naive_utc(),
        replicas: "replica1,replica2".to_string(),
        to_delete: false,
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let new_replica = Replica {
        id: 1,
        name: "replica1".to_string(),
        ip: "192.168.1.1".to_string(),
        port: 8080,
        connected_at: chrono::Utc::now().naive_utc(),
        is_online: true,
    };

    println!("File: {:?}", new_file);
    println!("Replica: {:?}", new_replica);
}