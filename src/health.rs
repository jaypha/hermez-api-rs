use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Health {
    #[serde(rename = "historyDB")]
    pub history_db: ConnectionStatus,
    #[serde(rename = "l2DB")]
    pub l2_db: ConnectionStatus,
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct ConnectionStatus {
    pub last_migration: String,
    pub status: String,
    pub version: String,
}

mod test {
    #[test]
    fn test_health() {
        let data = r#"{
          "historyDB": {
            "last_migration": "0002.sql",
            "status": "UP",
            "version": "PostgreSQL 13.2 (Debian 13.2-1.pgdg100+1) on x86_64-pc-linux-gnu"
          },
          "l2DB": {
            "last_migration": "0002.sql",
            "status": "UP",
            "version": "PostgreSQL 13.2 (Debian 13.2-1.pgdg100+1) on x86_64-pc-linux-gnu"
          },
          "status": "UP",
          "timestamp": "2021-06-17T01:30:10.248Z",
          "version": "v1.2.0"
        }"#;
        let _structure: super::Health = serde_json::from_str(data).unwrap();
    }
}
