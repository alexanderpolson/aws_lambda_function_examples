package com.orbitalsoftware.javalambda;

import com.amazonaws.services.lambda.runtime.Context;
import com.amazonaws.services.lambda.runtime.RequestHandler;
import com.google.common.collect.ImmutableMap;
import java.util.Map;
import software.amazon.awssdk.auth.credentials.AwsCredentialsProvider;
import software.amazon.awssdk.auth.credentials.EnvironmentVariableCredentialsProvider;
import software.amazon.awssdk.core.SdkSystemSetting;
import software.amazon.awssdk.http.urlconnection.UrlConnectionHttpClient;
import software.amazon.awssdk.regions.Region;
import software.amazon.awssdk.services.dynamodb.DynamoDbClient;
import software.amazon.awssdk.services.dynamodb.model.ListTablesResponse;

public class TestLambda implements RequestHandler<Void, Map<String, Object>> {

  private static final DynamoDbClient dynamoDbClient;

  static {
    AwsCredentialsProvider credentialsProvider = EnvironmentVariableCredentialsProvider.create();
    Region region = Region.of(System.getenv(SdkSystemSetting.AWS_REGION.environmentVariable()));
    dynamoDbClient =
        DynamoDbClient.builder()
            .credentialsProvider(credentialsProvider)
            .region(region)
            .httpClient(UrlConnectionHttpClient.create())
            .build();
    dynamoDbClient.listTables();
  }

  @Override
  public Map<String, Object> handleRequest(Void input, Context context) {
    ListTablesResponse response = dynamoDbClient.listTables();
    return ImmutableMap.of("message", "found tables", "tables", response.tableNames());
  }
}
