#Final Project: Website Status Checker

Jacob Garza
CSCI-3334-01-Spring 2025

#Running

1.) From the sites.txt file
```cargo run -- --file sites.txt

2.) Direct URL testing
```cargo run -- https://www.google.com https://httpstat.us/404

3.) Run any of the above with options
```cargo run -- --file sites.txt --workers 8 --timeout 3 --retries 2

#Input
For using the sites.txt to test URLs, simply put the desired URL in a new line for each URL

#Output Format
The result from the program will return the following information to the console:

``` [System Time] URL - Response TIme - HTTP Status/Error

After the program is completed, the results are saved into status.json with an array of the following objects:

```
[
  {
    "url": ,
    "status": ,
    "response_time_ms": ,
    "timestamp":
  }
}
```
