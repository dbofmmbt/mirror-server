# Mirror Server

A simple mock server which returns data from the request received. It's meant to be used for proxy testing.

In the [example](./example/) folder, you can find a working proxy test setup that you can adapt to your needs, with drawings and detailed explanation. Be sure to read its [README](./example/README.md) to know how to use it.

You can spawn multiple services, each one with a different server name. Then, you can make a request to a proxy and based on the response you can check if the request went where you wanted it to go.

It works with both HTTP and HTTPS. Beware that it listens on ports `80` and `443` for HTTP and HTTPS, respectively, by default.

## Response structure

```json
{
  "server_name": "SERVER_NAME",
  "request_path": "/example/path",
  "request_headers": {
    "content_type": ["example/type"],
    "x_forwarded_proto": ["ip1", "ip2", "ip3"]
  },
  "request_body": "valid json, or null if the server couldn't parse the body"
}
```

## Configuration

- `SERVER_NAME`
  - if you wish to identify to which service your request went, you can set this environment variable, which will be present on every response
- `SERVER_PORT`
  - if you need to make the service on another port. Default is `80`
- `SERVER_PORT_HTTPS`
  - specify where to listen HTTPS traffic. Default is `443`
