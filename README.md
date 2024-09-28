# Overview
Very simple examples of AWS Lambda functions used for performance testing. This came from the need to evaluate whether Rust is a suitable and better performing replacement for Java for AWS Lambda functions.

The problem with Java is that it suffers from a cold-start issue, where it needs to load classes from disk into memory when they're first used (JIT). Even in simple cases, it can take a couple of seconds or more for the initial start causing high user-perceived latency.

You can either [jump straight to the results of a test run that I did](results/README.md), or can learn how to execute the tests below.

# Running the tests

TODO:
* Need to fully rewrite this doc with instructions.
* How to create an account (link to docs for this).
* How to setup the AWS ClI (link to docs for this).
* How to setup the AWS CDK  (link to docs for this).