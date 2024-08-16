# senhasegura-c

Senhasegura API client for C

## Usage

Assuming `libsenhasegura_c.so` is at the root directory of your project, link against it while
compiling your application and import the auto-generated header file `senhasegura_c.h` to access the
FFI:

```sh
gcc main.c -L. -lm -lsenhasegura_c -o main
LD_LIBRARY_PATH=. ./main
```

```c
#include <stdio.h>

#include "senhasegura_c.h"

static char response_message[255];

static char info_content[8192], info_tag[256], info_type[256];

static AccessProtectedInformationApiResponse response = {
    .response = {
        .message = response_message,
    },
    .info = {
        .content = info_content,
        .tag = info_tag,
        .type = info_type,
    }};

static char exception_message[256], exception_detail[256];

static ApiError error = {
    .response = {
        .message = response_message,
    },
    .exception = {
        .message = exception_message,
        .detail = exception_detail,
    }};

int main()
{
    SenhaseguraClient *client;
    ErrorCode err;

    err = create_senhasegura_client(&client, &(SenhaseguraClientProps){
                                                 .base_url = "http://localhost:5000",
                                                 .client_id = "client_id",
                                                 .client_secret = "client_secret",
                                                 .request_timeout = 10,
                                                 .base_retry_delay_secs = 2,
                                                 .max_n_retries = 3,
                                             });
    if (err != OK)
    {
        printf("Error creating Senhasegura client: %d\n", err);
        return err;
    }

    err = access_protected_information(client, "28", &response, &error);
    if (err != OK)
    {
        printf("Error accessing protected information: %d\n", err);

        if (err == API)
        {
            printf("Exception: %s\n", error.exception.message);
        }

        return err;
    }

    printf("Response: status=%d message=\"%s\" error=%s error_code=%d\n", response.response.status, response.response.message, response.response.error ? "true" : "false", response.response.error_code);
    printf("Info: id=%d tag=\"%s\" type=\"%s\" content=\"%s\"\n", response.info.id, response.info.tag, response.info.type, response.info.content);

    destroy_senhasegura_client(client);

    return err;
}
```

### More

See the Rust [documentation](https://docs.rs/senhasegura-rs/) for more usage information.

## License

`senhasegura-c` is provided under the MIT license. See [LICENSE](LICENSE).
