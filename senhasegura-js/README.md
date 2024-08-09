# senhasegura-js

Senhasegura API client for Node.js

## Installation

```sh
# NPM
npm install senhasegura-js

# Yarn
yarn add senhasegura-js

# PNPM
pnpm add senhasegura-js
```

## Usage

```sh
const { SenhaseguraClient } = require('senhasegura-js');

const client = SenhaseguraClient.create({
  baseUrl: "https://senhasegura.acme.com",
  clientId: "client_id",
  clientSecret: "client_secret",
});

const result = await client.accessProtectedInformation(28);
console.log(result);
```

### More

See the Rust [documentation](https://docs.rs/senhasegura-rs/) for more usage information.

## License

`senhasegura-js` is provided under the MIT license. See [LICENSE](LICENSE).
