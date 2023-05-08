# Mirror Server

A simple mock server which returns data from the request received. It's meant to be used for proxy testing.

You can spawn multiple services, each one with a different server name. Then, you can make a request to a proxy and based on the response you can check if the request went where you wanted it to go.

It works with both HTTP and HTTPS. Beware that it listens on ports `8080` and `40443` for HTTP and HTTPS, respectively, by default.

## Response structure

```json
{
  "server_name": "SERVER_NAME",
  "request_path": "/example/path",
  "request_headers": {
    "content_type": ["example/type"]
    // ...
  },
  "request_body": "valid json" // It will be null if the server couldn't parse the body.
}
```

## Configuration

- `SERVER_NAME`
  - if you wish to identify to which service your request went, you can set this environment variable, which will be present on every response
- `SERVER_PORT`
  - if you need to make the service on another port. Default is `8080`
- `SERVER_PORT_HTTPS`
  - specify where to listen HTTPS traffic. Default is `40443`
