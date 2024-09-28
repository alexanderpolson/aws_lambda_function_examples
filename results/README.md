# Testing Method
All three functions were run through the same series of tests in order to eliminate bias between them.

The first tests were concerned with cold starts. Before each invocation, a simple change is made to the description of the Lambda function in order to provoke a cold start for the following invocation. Cold starts were the primary concern for the testing. 100 invocations for each were executed, though this can easily be changed in the code.

The second set of tests, simply do invocations, one after the other, without trying to eliminate cold starts. Because the previous cold start tests have already "warmed up" the functions, the initial invocations in this second set of tests, should be able to be executed quite rapidly, though functions will need to be rewarmed up periodically which AWS Lambda handles automatically. Similarly to the above, 100 invocations were executed.

Full data and analysis can be found in [this Google Sheet](https://docs.google.com/spreadsheets/d/1J3KpR82wrbcTrCkxgD66l1TYVgBaHP9X_ZUV4lc90Sc/edit?gid=0#gid=0).

## What are the functions doing?
Each of the implemented Lambdas simply makes a call to Dynamo to determine the tables that it has access to, and returns that list of table names in its response.

Care has been taken to front-load any initialization-related code during the actual instantiation of the function, and they don't wait until trying to respond to a request to do so.

## Java SnapStart Implementation
I would have expected the SnapStart implementation to be a general improvement over the non-SnapStart function, so it's very possible I have it configured incorrectly, or am doing something else incorrect. Should I figure it out, I'll come back to retest and update my results.

# Performance Results
The high-level results show a clear winner. Rust performed more than 3 times faster on cold starts than both Java implementations, with similar results when the functions were warmed up first.
```
JavaLambda: avg cold start time: 3058.4, avg warm start time: 289.37
JavaSnapStartLambda: avg cold start time: 2840.71, avg warm start time: 334.65
RustLambda: avg cold start time: 901.56, avg warm start time: 104.95
```

This doesn't tell the whole story though.

At the 90th percentile, Rust performed more than 4 times better than both Java implementations when warmed up.

Finally, and perhaps equally as important as user-perceived latency, is the billing duration. This graph was captured from CloudWatch for the execution of the different functions.
![graphed results](Test%20Results%20Graph.png)

The green, blue and orange represent the Rust, Java, and Java SmartStart implementations, respectfully.

There are several things that stand out about these. First and foremost, is the vast difference between the billed duration for the Rust function than both Jova functions. Second, the Rust tests completedly well before the Java tests. This doesn't say a huge amount, as the invocations were done synchronously.

The real results are the total billed duration, which show a significant difference:
* Java: 122103.15 msec
* JavaSnapStart: 114543.78 msec
* Rust: 12415.97 msec

Rust executed the same number of executions in only 10% of the time it took the Java Lambda to do the same, and only 11% of the SnapStart executions.

## Cold Invocations
* Only 12 took more than 2 seconds for the Rust implementation. The 88 other cold starts took less than 1.35 seconds. Compared with 82, and 74 for the Java and Java SmartStart implementations, respectfully.

## Warmed Invocations
* Only 8 warmed invocations took more than 100 msec for the Rust implementation, and only 15 greater than 60 msec.
* Only 7 warmed invocations took longer than the single fastest Java (both regular and SmartStart) invocations.