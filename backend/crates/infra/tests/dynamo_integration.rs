use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    types::{AttributeDefinition, KeySchemaElement, KeyType, ScalarAttributeType},
    Client,
};
use tenk_domain::{Goal, GoalId, UserId};
use tenk_infra::DynamoRepositories;

fn localstack_endpoint() -> Option<String> {
    std::env::var("LOCALSTACK_ENDPOINT").ok()
}

#[tokio::test]
async fn saves_and_reads_goal_if_localstack_available() {
    let Some(endpoint) = localstack_endpoint() else {
        eprintln!("LOCALSTACK_ENDPOINT not set; skipping integration test");
        return;
    };

    let shared_config = aws_config::from_env()
        .region("us-east-1")
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url(endpoint)
        .load()
        .await;
    let client = Client::new(&shared_config);
    let table_name = format!("tenk-test-{}", uuid::Uuid::new_v4());

    client
        .create_table()
        .table_name(&table_name)
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("pk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("sk")
                .attribute_type(ScalarAttributeType::S)
                .build(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("pk")
                .key_type(KeyType::Hash)
                .build(),
        )
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("sk")
                .key_type(KeyType::Range)
                .build(),
        )
        .billing_mode("PAY_PER_REQUEST")
        .send()
        .await
        .unwrap();

    let repo = DynamoRepositories::new(&table_name, client.clone());
    let goal = Goal {
        id: GoalId::default(),
        user_id: UserId::default(),
        title: "Localstack goal".into(),
        target_hours: 10000,
        completed_minutes: 0,
        milestones: vec![],
        achievements: vec![],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    repo.create(&goal).await.unwrap();

    let fetched = repo
        .find_by_id(&goal.id, &goal.user_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.title, goal.title);
}
