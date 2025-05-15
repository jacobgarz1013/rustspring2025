# Final Project: Website Status Checker

## Jacob Garza
## CSCI-3334-01-Spring 2025

## Project Overview
This project checks the statues of the entered URLs and returns other relevant information such as:
HTTP status code
Response time from URL
Errors encountered

## Build Instructions

To build the tool in release mode:

```bash
cargo build --release
```

# Usage
```
[--file <sites.txt>] [URL ...] [--workers <N>] [--timeout <S>] [--retries <N>] [--period <S>] [--assert-header <key=value>]
```

## Running

### 1.) From the sites.txt file
```
 cargo run -- --file sites.txt
```
### 2.) Direct URL testing
```
 cargo run -- https://www.google.com https://httpstat.us/404
```
### 3.) Run any of the above all with options
```
 cargo run -- --file sites.txt --workers 8 --timeout 3 --retries 2 --period 10 --assert-header X-Frame-Options=DENY
```

## Input
For using the sites.txt to test URLs, simply put the desired URL in a new line for each URL

## Output Format
The result from the program will return the following information to the console:

```
 [System Time] URL - Response TIme - HTTP Status/Error 

 Summary: min = #ms, max = #ms, avg = #ms
```

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

# Bonus Features
This project has the following bonus features implemented:

### --period <sec>
When this option is used, keeps on looping the site checking function indefinitely every n seconds of the users choosing.

### Summary Statistics
This program keeps track of the minimum, the maximum, and the average response times and prints them every round this program runs for.

### HTTP Header Assertions
Checks if the response includes a header with the indicated key and value.