use dj_engine::data::campaign::{CampaignData, CampaignNodeType, CampaignNodeData};

#[test]
fn test_campaign_data_serialization() {
    let mut campaign = CampaignData::new("Test Campaign");
    
    let mut node1 = CampaignNodeData::default();
    node1.id = "node_1".to_string();
    node1.name = "Intro Scene".to_string();
    node1.node_type = CampaignNodeType::Scene;
    node1.content_path = Some("scenes/intro.json".to_string());
    
    campaign.add_node(node1);
    
    let serialized = serde_json::to_string_pretty(&campaign).unwrap();
    println!("{}", serialized);
    
    let deserialized: CampaignData = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(deserialized.title, "Test Campaign");
    assert_eq!(deserialized.nodes.len(), 1);
    assert_eq!(deserialized.nodes[0].name, "Intro Scene");
    assert_eq!(deserialized.nodes[0].node_type, CampaignNodeType::Scene);
}

#[test]
fn test_campaign_defaults() {
    let node = CampaignNodeData::default();
    assert_eq!(node.node_type, CampaignNodeType::Start);
    assert!(node.next_node_ids.is_empty());
}
