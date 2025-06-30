use criterion::{criterion_group, criterion_main, Criterion};
use zion_core::network::p2p::MyBehaviour;

fn bench_my_behaviour(c: &mut Criterion) {
    c.bench_function("my_behaviour_poll", |b| {
        b.iter(|| {
            let mut behaviour = MyBehaviour::default();
            let mut cx = std::task::Context::from_waker(futures::task::noop_waker_ref());
            let mut params = libp2p::swarm::dummy::PollParameters::new();
            let _ = behaviour.poll(&mut cx, &mut params);
        })
    });
}

criterion_group!(benches, bench_my_behaviour);
criterion_main!(benches);
