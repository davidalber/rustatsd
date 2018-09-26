# rustatsd
This is currently only a toy project I am working on occasionally to do some Rust programming. This is the beginning of a StatsD server written in Rust.

Currently, I have implemented ingestion of UDP packets and sharding of the metrics out to worker threads. Aggregation code will come next.

# Sending a Metric
You can send metrics in a variety of ways. A simple command-line way is to use `socat`. For example, do the following.

```
echo -n "test.blah.blah" | socat -t 0 - UDP:127.0.0.1:8125
```
