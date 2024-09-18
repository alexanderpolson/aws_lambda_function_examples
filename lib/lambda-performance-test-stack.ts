import * as cdk from 'aws-cdk-lib';
import {Duration} from 'aws-cdk-lib';
import * as iam from 'aws-cdk-lib/aws-iam'
import {Effect} from 'aws-cdk-lib/aws-iam'
import * as lambda from 'aws-cdk-lib/aws-lambda'
import {Architecture, Code, Runtime} from 'aws-cdk-lib/aws-lambda'
import {Construct} from 'constructs';

// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class LambdaPerformanceTestStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const policy = new iam.Policy(this, "ListDynamoDbTables", {
      statements: [
        new iam.PolicyStatement({
          effect: Effect.ALLOW,
          actions: [
            "dynamodb:ListTables"
          ],
          resources: [
            "arn:aws:dynamodb:*:*:*/*"
          ]
        })
      ]
    });

    // The code that defines your stack goes here
    const javaFunction = new lambda.Function(this, "JavaLambda", {
      handler: "com.orbitalsoftware.lambda.TestLambda",
      runtime: Runtime.JAVA_21,
      code: Code.fromAsset("java/target/lambda-0.1.jar"),
      functionName: "JavaLambda",
      architecture: Architecture.ARM_64,
      description: "A simple Lambda function implemented in Java.",
      timeout: Duration.seconds(10),
    });
    javaFunction.role?.attachInlinePolicy(policy);

    // TODO: Add Java Lambda function with SnapStart
    // https://docs.aws.amazon.com/lambda/latest/dg/snapstart.html

    // TODO: Add Rust Lambda function

    // example resource
    // const queue = new sqs.Queue(this, 'CdkQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
