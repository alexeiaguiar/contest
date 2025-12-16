# Contest - Connectivity Test

This tool takes a configuration yaml file and a list of test cases, and runs connectivity tests based on the provided 
configuration.

## Usage
```shell
contest <config_file>
```

## Example configuration file
```yaml
# Optional parameters
parameters:
  timeout: 5s
tests:
  - name: TCP connected
    tcp:
      host: google.ca
      port: 80
      expected: connected
  - name: TCP timeout
    tcp:
      host: google.ca
      port: 81
      expected: timeout
  - name: HTTP 200
    http:
      url: https://httpbin.org/status/200
      expected: 200
  - name: HTTP 404
    http:
      url: https://httpbin.org/status/404
      expected: 404
  - name: HTTP 400
    http:
      url: https://httpbin.org/status/400
      expected: 400
```

## Results
```
Running tests from config file: test.yaml
✅  Pass - TCP connected - Expected: Connected, Actual: Connected
✅  Pass - TCP timeout - Expected: Timeout, Actual: Timeout
✅  Pass - TCP refused - Expected: Refused, Actual: Refused
✅  Pass - HTTP 200 Expected: Connected with status 200, Actual: Connected with status 200
✅  Pass - HTTP 404 Expected: Connected with status 404, Actual: Connected with status 404
✅  Pass - HTTP timeout Expected: Timeout, Actual: Timeout
✅  Pass - HTTP refused Expected: Refused, Actual: Refused
```