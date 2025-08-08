# Contest - Connectivity Test

This tool takes a configuration yaml file and a list of test cases, and runs connectivity tests based on the provided 
configuration.

## Usage
```shell
contest <config_file>
```

## Example configuration file
```yaml
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
✅  TCP connected Expected: Connected, Actual: Connected
✅  TCP timeout Expected: Timeout, Actual: Timeout
✅  HTTP 200 Expected: 200, Actual: 200
✅  HTTP 404 Expected: 404, Actual: 404
✅  HTTP 400 Expected: 400, Actual: 400
```