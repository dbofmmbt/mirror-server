import httpx

proxy = httpx.Client(base_url="http://proxy:8080")


def test_hello_proxy():
    assert proxy.get("/hello").text == "world!"


def test_health():
    assert proxy.get("/health").is_success


def test_backend():
    request_body = {"name": "Bond, James Bond", "profession": "secret agent"}
    response = proxy.post("/backend/save",
                          json=request_body)

    received_on_backend = response.json()["request_body"]

    for key in request_body.keys():
        assert received_on_backend[key] == request_body[key]


def test_mocking_a_real_backend():
    assert proxy.get("/any-domain").json()["server_name"] == "ANY_DOMAIN_MOCK"
