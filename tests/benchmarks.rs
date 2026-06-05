// Benchmark tests for PeRust
// Run with: cargo test --benches or cargo bench

#[cfg(test)]
mod benches {
    use perust_utils::binary::{read_var_int, write_var_int};
    use perust_world::chunk::Chunk;
    use perust_world::generator::FlatGenerator;
    use perust_nbt::{Tag, NbtWriter, Endianness};

    #[test]
    fn bench_var_int_encoding() {
        let mut buf = Vec::with_capacity(1024 * 1024);
        for _ in 0..100_000 {
            buf.clear();
            write_var_int(&mut buf, 1234567);
        }
    }

    #[test]
    fn bench_chunk_serialization() {
        let gen = FlatGenerator::default();
        let chunk = gen.generate_chunk(0, 0);
        for _ in 0..100 {
            let _data = chunk.serialize_network();
        }
    }

    #[test]
    fn bench_nbt_write() {
        let mut compound = Tag::Compound(indexmap::IndexMap::new());
        if let Tag::Compound(map) = &mut compound {
            for i in 0..100 {
                map.insert(format!("key_{}", i), Tag::Int(i));
            }
        }
        for _ in 0..1000 {
            let mut writer = NbtWriter::new(Endianness::BigEndian);
            writer.write_compound("bench", &compound);
            let _ = writer.into_bytes();
        }
    }

    #[test]
    fn bench_flat_generation() {
        let gen = FlatGenerator::default();
        for i in 0..100 {
            let _chunk = gen.generate_chunk(i, 0);
        }
    }
}
