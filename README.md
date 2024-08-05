<a id="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

<br />
<div align="center">
  <a href="https://github.com/gabrielkim13/senhasegura-rs">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">senhasegura-rs</h3>

  <p align="center">
    A Rust library for interacting with <a href="https://senhasegura.com/">senhasegura</a>'s API.
    <br />
    <a href="https://docs.rs/senhasegura-rs/latest/senhasegura-rs"><strong>Explore the docs »</strong></a>
    <br />
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

## About The Project

A Rust library for interacting with [senhasegura](https://senhasegura.com)'s API.

The goal of this project is to enable interaction with senhasegura's APIs in many languages /
runtimes while maintaining a single core codebase (i.e. `senhasegura-rs`) and several other
libraries bound to it.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* [![Rust][Rust]][Rust-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/learn/get-started)

#### Cross-compilation

In order to cross-compile to Windows MSVC target, install
[cargo-xwin](https://github.com/rust-cross/cargo-xwin):

```sh
cargo install --locked cargo-xwin
rustup target add x86_64-pc-windows-msvc
rustup component add llvm-tools-preview
```

### Installation

#### Rust

```toml
[dependencies]
senhasegura-rs = "0.1"
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

### Rust

```rs
use senhasegura_rs::{AccessProtectedInformationAPI, SenhaseguraClient};

let base_url = "https://senhasegura.acme.com".parse()?;
let client_id = "client_id";
let client_secret = "client_secret";

let client = SenhaseguraClient::builder(base_url, client_id, client_secret).build()?;

// Access protected information
println!("{:#?}", client.access_protected_information(28)?);
```

_For more examples, please refer to the [Documentation](https://docs.rs/senhasegura-rs/latest/senhasegura-rs)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Roadmap

### Senhasegura APIs

- [PAM Core APIs](https://docs.senhasegura.io/docs/a2a-apis-pam-core)
  - [ ] [Credentials API](https://docs.senhasegura.io/docs/a2a-pam-core-credentials-api)
  - [ ] [Devices API](https://docs.senhasegura.io/docs/a2a-pam-core-devices-api)
  - [ ] [Proxy API](https://docs.senhasegura.io/docs/a2a-proxy-api)
  - [ ] [SSH Keys API](https://docs.senhasegura.io/docs/a2a-pam-core-ssh-keys-api)
  - [x] [Protected information API](https://docs.senhasegura.io/docs/en/a2a-pam-core-protected-information-api)
- [ ] [Related users API](https://docs.senhasegura.io/docs/a2a-pam-core-related-users-api)

### Languages

- [x] [Rust](https://www.rust-lang.org/)
    - [ ] Native libraries (i.e. `.so`, `.dylib`, `.dll`)
- [JavaScript](https://developer.mozilla.org/docs/Web/JavaScript) / [TypeScript](https://www.typescriptlang.org/)
    - [x] [Node.js](https://nodejs.org/)
- [ ] [PHP](https://dotnet.microsoft.com/languages/csharp)
- [ ] [Python](https://www.python.org/)
- [ ] [C#](https://dotnet.microsoft.com/languages/csharp)

See the [open issues](https://github.com/gabrielkim13/senhasegura-rs/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the MIT License. See `LICENSE` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contact

Gabriel Kim - [gabrielkim13](https://github.com/gabrielkim13) - gabrielkim13@gmail.com

Project Link: [https://github.com/gabrielkim13/senhasegura-rs](https://github.com/gabrielkim13/senhasegura-rs)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

* [Senhasegura Documentation](https://docs.senhasegura.io/docs)
* [NAPI-RS](https://napi.rs/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

[contributors-shield]: https://img.shields.io/github/contributors/gabrielkim13/senhasegura-rs.svg?style=for-the-badge
[contributors-url]: https://github.com/gabrielkim13/senhasegura-rs/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/gabrielkim13/senhasegura-rs.svg?style=for-the-badge
[forks-url]: https://github.com/gabrielkim13/senhasegura-rs/network/members
[stars-shield]: https://img.shields.io/github/stars/gabrielkim13/senhasegura-rs.svg?style=for-the-badge
[stars-url]: https://github.com/gabrielkim13/senhasegura-rs/stargazers
[issues-shield]: https://img.shields.io/github/issues/gabrielkim13/senhasegura-rs.svg?style=for-the-badge
[issues-url]: https://github.com/gabrielkim13/senhasegura-rs/issues
[license-shield]: https://img.shields.io/github/license/gabrielkim13/senhasegura-rs.svg?style=for-the-badge
[license-url]: https://github.com/gabrielkim13/senhasegura-rs/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/gabrielkimrocha
[Rust]: https://img.shields.io/badge/Rust-CE412B?style=for-the-badge&logo=rust&logoColor=black
[Rust-url]: https://www.rust-lang.org/
