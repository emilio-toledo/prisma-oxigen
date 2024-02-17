use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Connector {
    Mysql,
    Mongodb,
    Sqlite,
    Postgresql,
    Postgres,
    Sqlserver,
    Cockroachdb,
}
