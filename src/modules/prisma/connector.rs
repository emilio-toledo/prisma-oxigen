use serde::Deserialize;

#[derive(Deserialize, Debug)]
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
