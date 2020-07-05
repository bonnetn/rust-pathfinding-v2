# Pathfinding

Yet another pathfinding written in Rust for the [french robotic cup](https://www.coupederobotique.fr/).

This just applies Dijkstra on a visiblity map.

## Run it

### FFI
This library has a FFI interface. 
You can use it from other languages.
Check out `main.py` for an example.

```
cargo +nightly build --release
pip install cffi
python3 main.py
```

### Unit test
```
cargo +nightly test
```

### Benchmark
```
cargo +nightly bench
```
