use rusqlite::Connection;

use crate::error::Result;

pub struct PartialDevice {
    pub device_serial: String,
    pub code_verifier: String,
    pub oauth_url: String,
    pub country_code: String,
    pub domain: String,
    pub market_place_id: String,
}

// // impl PartialDevice {
// //     pub fn insert(
// //         conn: &Connection,
// //         device_serial: &str,
// //         code_verifier: &str,
// //         oauth_url: &str,
// //     ) -> Result<()> {
// //         conn.execute(
// //             "INSERT INTO auth (device_serial, code_verifier, oauth_url) VALUES (?1, ?2, ?3)",
// //             (device_serial, code_verifier, oauth_url),
// //         )?;
// //         Ok(())
// //     }

// //     pub fn find_by_device_serial(conn: &Connection, device_serial: &str) -> Result<Option<Self>> {
// //         let mut stmt = conn.prepare("SELECT * FROM auth WHERE device_serial = ?1")?;
// //         let mut rows = stmt.query(&[device_serial])?;

// //         if let Some(row) = rows.next()? {
// //             Ok(Some(Self {
// //                 device_serial: row.get(0)?,
// //                 code_verifier: row.get(1)?,
// //                 oauth_url: row.get(2)?,
// //             }))
// //         } else {
// //             Ok(None)
// //         }
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::db::new_pool;

//     #[tokio::test]
//     async fn test_insert_device() {
//         let pool = new_pool().unwrap();
//         let txn = pool.get().unwrap().transaction().unwrap();

//         let device_serial = "device_serial";
//         let code_verifier = "code_verifier";
//         let oauth_url = "oauth_url";
//         PartialDevice::insert(&txn, device_serial, code_verifier, oauth_url).unwrap();
//     }
// }
