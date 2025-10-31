use chiplet_architecture_template::interconnect::communication::{Message, MessageBus};

#[test]
fn message_bus_initializes_channels() {
    let mut bus = MessageBus::default();
    bus.initialize_channels(2).expect("channel init");

    let message = Message {
        source: "core".into(),
        destination: "memory".into(),
        payload: vec![1, 2, 3],
        qos: 0,
    };

    bus.publish(0, message).expect("publish");
    let drained = bus.drain_channel(0).unwrap();
    assert_eq!(drained.len(), 1);
}
