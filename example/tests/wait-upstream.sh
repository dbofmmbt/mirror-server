#!/bin/sh

# This wait script is necessary because otherwise the tests could start before
# the proxy was ready for receiving requests. 

set -eu

timeout 10s bash -c 'until curl --silent --head http://proxy; do
  >&2 echo "Proxy is unavailable - sleeping"
  sleep 1
done' || {
  echo "Error: Proxy is not working or timed out after 10 seconds. Please, check if there's any error in the proxy config"
  exit 1
}
  
>&2 echo "Proxy is up - executing test suite\n"
# Print and execute all other arguments starting with `$1`
# So `exec "$1" "$2" "$3" ...`
exec "$@"