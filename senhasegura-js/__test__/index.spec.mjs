import test from "ava";

import { SenhaseguraClient } from "../index.js";

test("instantiate SenhaseguraClient", (t) => {
    const client = SenhaseguraClient.create({
        baseUrl: "https://senhasegura.acme.com",
        clientId: "client_id",
        clientSecret: "client_secret",
    });

    t.assert(client instanceof SenhaseguraClient);
    t.throws(() => client.accessProtectedInformation(28), {
        instanceOf: Error,
        message: "Request failed",
    });

    client.createProtectedInformation({});
});
