# Overview
Very simple examples of AWS Lambda functions used for performance testing. This came from the need to evaluate whether Rust is a suitable and better performing replacement for Java for AWS Lambda functions.

The problem with Java is that it suffers from a cold-start issue, where it needs to load classes from disk into memory when they're first used (JIT). Even in simple cases, it can take a couple of seconds or more for the initial start causing high user-perceived latency.

# What are they doing?
Each of the implemented Lambdas simply makes a call to Dynamo to determine the tables that it has access to, and returns that list of table names in its response.

Care has been taken to front-load any initialization-related code during the actual instantiation of the function, and they don't wait until trying to respond to a request to do so.

# Latency Samples
Each of the calls whose latencies are listed below, are based on values that were exposed from calling the functions via Lambda's AWS console. All results are in milliseconds.

## Initialization
These results are specific to the cold start time. That is, an instance of the Lambda function being called is not available for serving requests, so it needs to be initialized before responding.

I should note that the cold start was so minimal for Rust, that I dind't call out the init and duration separately.

|Java|Rust|
|----|----|
|Init: 2736ms, Duration: 501ms|119.29ms|
|Init: 2901ms, Duration: 535ms||

## After Initialization
|Java|Rust|
|----|----|
|219   |34   |
|137   |12   |
|214   |8   |
|219   |16   |
|353   |11   |
|129   |9   |
|107   |10   |
|76   |11   |
|90   |8   |
|88   |9   |
|215   |10   |
|382   |10   |
|75   |   |
|57   |   |
