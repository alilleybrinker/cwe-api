<div align="center">

# `cwe-api`
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)][license]

</div>

This is a crate for convenient use of the [Common Weakness Enumeration (CWE)
API][cwe_api]. This is __not__ an official crate produced by the CWE program.

The code in this crate is generated using [Progenitor][progenitor], based on
the published [CWE OpenAPI specification][cwe_openapi].

## Packages

This repository includes two Rust crates:

- `cwe-api`: a library crate for accessing the CWE API.
- `cwe-api-cli`: a binary crate that provides a CLI for accessing the CWE API.

## Installing the CLI

```sh
$ git clone https://github.com/alilleybrinker/cwe-api.git
$ cargo install --path cwe-api-cli
```

This requires `git` and a [Rust toolchain installation][rust_install].

## CLI Example

```sh
$ cwe-api cwe info 230
[
  {
    "ID": "230",
    "Type": "variant_weakness"
  }
]
$ cwe-api cwe parents 230
[
  {
    "ID": "229",
    "Primary_Parent": true,
    "Type": "base_weakness",
    "ViewID": "1000"
  },
  {
    "ID": "1407",
    "Type": "category",
    "ViewID": "1400"
  },
  {
    "ID": "851",
    "Type": "category",
    "ViewID": "844"
  },
  {
    "ID": "993",
    "Type": "category",
    "ViewID": "888"
  }
]
```

## License

The code is licensed under the MIT license. You can view the full contents
in the [LICENSE][license] file.

[cwe_api]: https://github.com/CWE-CAPEC/REST-API-wg/blob/main/Quick%20Start.md
[cwe_openapi]: https://github.com/CWE-CAPEC/REST-API-wg/blob/main/openapi.json
[progenitor]: https://github.com/oxidecomputer/progenitor
[license]: https://github.com/alilleybrinker/cwe-api/blob/main/LICENSE
[rust_install]: https://www.rust-lang.org/tools/install
