use crate::config::Config;

use anyhow::Result;
use neo4rs::{ConfigBuilder, Graph, Error, query, Query};
use serde_json::Value;

pub async fn new_client(config: &Config) -> Result<Graph> {
    let neo4j_config = ConfigBuilder::default()
        .uri(&config.db_url)
        .user(&config.db_user)
        .password(&config.db_pass)
        .build()?;

    let client: Graph = Graph::connect(neo4j_config).await?;

    Ok(client)
}

pub async fn ensure_ip_uniqueness_constraint(graph: &Graph) -> Result<(), Error> {
    graph.run(
        query(
            r#"
            CREATE CONSTRAINT ip_unique IF NOT EXISTS
            FOR (n:IP)
            REQUIRE n.ip IS UNIQUE
            "#
        )
    ).await?;

    Ok(())
}

pub fn model_query(event: Value) -> Query {
    query(
        r#"
        MERGE (src:IP {ip: $src_ip_zeek})
        MERGE (dest:IP {ip: $dest_ip_zeek})
        CREATE (src)-[:EVENT {
            created_at: datetime(),
            conn_state: $conn_state,
            duration: $duration,
            src_port_zeek: $src_port_zeek,
            dest_port_zeek: $dest_port_zeek,
            local_orig: $local_orig,
            local_resp: $local_resp,
            missed_bytes: $missed_bytes,
            orig_bytes: $orig_bytes,
            orig_pkts: $orig_pkts,
            proto: $proto,
            resp_bytes: $resp_bytes,
            resp_pkts: $resp_pkts,
            service: $service,
            ts: $ts,
            uid: $uid,,
            label_binary: $label_binary
            label_tactic: $label_tactic,
            confidence: $confidence,
        }]->(dest)
        "#,
    )
    .param("src_ip_zeek", event["src_ip_zeek"].as_str().unwrap_or_default())
    .param("dest_ip_zeek", event["dest_ip_zeek"].as_str().unwrap_or_default())
    .param("conn_state", event["conn_state"].as_str().unwrap_or_default())
    .param("duration", event["duration"].as_f64().unwrap_or_default())
    .param("src_port_zeek", event["src_port_zeek"].as_i64().unwrap_or_default())
    .param("dest_port_zeek", event["dest_port_zeek"].as_i64().unwrap_or_default())
    .param("local_orig", event["local_orig"].as_bool().unwrap_or_default())
    .param("local_resp", event["local_resp"].as_bool().unwrap_or_default())
    .param("missed_bytes", event["missed_bytes"].as_i64().unwrap_or_default())
    .param("orig_bytes", event["orig_bytes"].as_i64().unwrap_or_default())
    .param("orig_pkts", event["orig_pkts"].as_i64().unwrap_or_default())
    .param("proto", event["proto"].as_str().unwrap_or_default())
    .param("resp_bytes", event["resp_bytes"].as_i64().unwrap_or_default())
    .param("resp_pkts", event["resp_pkts"].as_i64().unwrap_or_default())
    .param("service", event["service"].as_str().unwrap_or_default())
    .param("ts", event["ts"].as_f64().unwrap_or_default())
    .param("uid", event["uid"].as_str().unwrap_or_default())
    .param("label_binary", event["label_binary"].as_bool().unwrap_or_default())
    .param("label_tactic", event["label_tactic"].as_str().unwrap_or_default())
    .param("confidence", event["confidence"].as_i64().unwrap_or_default())
}