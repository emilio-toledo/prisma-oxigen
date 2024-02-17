use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Mysql,
    Mongodb,
    Sqlite,
    Postgresql,
    Postgres,
    Sqlserver,
    Cockroachdb,
}
