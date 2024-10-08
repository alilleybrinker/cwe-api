//! `cwe-api` is a crate for interacting with the [Common Weakness Enumeration
//! (CWE) REST API][cwe_api]. The code in this crate is generated with
//! [Progenitor][progenitor] from the official [CWE OpenAPI JSON
//! specification][cwe_openapi].
//!
//! [cwe_api]: https://github.com/CWE-CAPEC/REST-API-wg/blob/main/Quick%20Start.md
//! [cwe_openapi]: https://github.com/CWE-CAPEC/REST-API-wg/blob/main/openapi.json
//! [progenitor]: https://github.com/oxidecomputer/progenitor

#[cfg(test)]
mod tests;

/// The base URL of the CWE API.
pub const BASE_URL: &str = "https://cwe-api.mitre.org/api/v1";

progenitor::generate_api!("api/cwe-api.json");
