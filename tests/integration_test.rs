// Basic integration tests for PeRust

#[test]
fn test_server_properties_default() {
    let props = perust_config::ServerProperties::default();
    assert_eq!(props.server_port, 19132);
    assert_eq!(props.max_players, 20);
    assert_eq!(props.view_distance, 10);
    assert_eq!(props.gamemode, 1);
    assert_eq!(props.difficulty, 2);
}

#[test]
fn test_chunk_creation() {
    let chunk = perust_world::chunk::Chunk::new(0, 0);
    assert_eq!(chunk.x, 0);
    assert_eq!(chunk.z, 0);
}

#[test]
fn test_flat_generator() {
    let gen = perust_world::generator::FlatGenerator::default();
    let chunk = gen.generate_chunk(0, 0);
    // Bedrock at y=0
    let (block_id, _) = chunk.get_block(0, 0, 0);
    assert_eq!(block_id, 7); // BEDROCK
}

#[test]
fn test_player_creation() {
    let player = perust_player::Player::new(1, "127.0.0.1:19132".parse().unwrap());
    assert_eq!(player.runtime_id, 1);
}

#[test]
fn test_command_dispatcher() {
    use perust_command::{Command, CommandDispatcher, CommandSender, CommandExecutor, CommandResult};

    struct TestCommand;
    impl CommandExecutor for TestCommand {
        fn execute(&self, _sender: &CommandSender, _command: &Command, _args: &[String]) -> CommandResult {
            Ok(())
        }
    }

    let mut dispatcher = CommandDispatcher::new();
    let cmd = Command::new("test").with_description("A test command");
    dispatcher.register(cmd, Box::new(TestCommand));

    let sender = CommandSender::Console;
    assert!(dispatcher.dispatch(&sender, "test").is_ok());
}

#[test]
fn test_event_dispatcher() {
    use perust_event::{EventDispatcher, EventPriority, Event};
    use std::sync::{Arc, AtomicI32, Ordering};

    #[derive(Default)]
    struct TestEvent { pub value: i32 }
    impl Event for TestEvent { fn event_name(&self) -> &str { "TestEvent" } }

    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = counter.clone();

    let mut dispatcher = EventDispatcher::new();
    dispatcher.register::<TestEvent>(EventPriority::Normal, Box::new(move |event| {
        let e = event.downcast_mut::<TestEvent>().unwrap();
        e.value += 1;
        counter_clone.fetch_add(1, Ordering::SeqCst);
    }));

    let mut event = TestEvent::default();
    dispatcher.dispatch(&mut event);
    assert_eq!(event.value, 1);
}

#[test]
fn test_nbt_roundtrip() {
    use perust_nbt::{Tag, NbtReader, NbtWriter, Endianness};

    let mut compound = Tag::Compound(indexmap::IndexMap::new());
    if let Tag::Compound(map) = &mut compound {
        map.insert("name".to_string(), Tag::String("PeRust".to_string()));
        map.insert("version".to_string(), Tag::Int(1));
    }

    let mut writer = NbtWriter::new(Endianness::BigEndian);
    writer.write_compound("root", &compound);
    let bytes = writer.into_bytes();

    let mut reader = NbtReader::new(&bytes, Endianness::BigEndian);
    let result = reader.read_compound().unwrap();
    assert_eq!(result.name, "root");
}

#[test]
fn test_var_int() {
    use perust_utils::binary::{read_var_int, write_var_int};

    let values = [0, 1, 127, 128, 255, 30000, 2097151, i32::MAX];
    for &val in &values {
        let mut buf = Vec::new();
        write_var_int(&mut buf, val);
        let result = read_var_int(&mut &buf[..]).unwrap();
        assert_eq!(result, val);
    }
}

#[test]
fn test_block_registry() {
    let registry = perust_blocks::block::BLOCK_REGISTRY.read();
    let air = registry.get_by_id(0);
    assert!(air.is_some());
    let air = air.unwrap();
    assert_eq!(air.id, 0);
    assert!(!air.is_solid);
}

#[test]
fn test_item_registry() {
    let registry = perust_items::item_registry::ITEM_REGISTRY.read();
    let diamond = registry.get_by_id(264); // DIAMOND
    assert!(diamond.is_some());
}

#[test]
fn test_inventory() {
    use perust_inventory::{BaseInventory, Inventory, InventoryType, ItemStack};

    let mut inv = BaseInventory::new(InventoryType::Player, 36);
    assert_eq!(inv.size(), 36);

    let item = ItemStack::new(264, 0, 64); // 64 diamonds
    inv.set_item(0, Some(item));
    assert!(inv.get_item(0).is_some());
}

#[test]
fn test_scheduler() {
    use perust_scheduler::{Scheduler, FnTask};
    use std::sync::{Arc, AtomicI32, Ordering};

    let counter = Arc::new(AtomicI32::new(0));
    let counter_clone = counter.clone();

    let mut scheduler = Scheduler::new();
    let task = FnTask::new(move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });
    scheduler.schedule_task(Box::new(task));

    let tasks = scheduler.tick();
    for mut task in tasks {
        task.run();
    }

    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
