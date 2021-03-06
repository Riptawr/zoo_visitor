## Synopsis

zoo_visitor is a simple tool to lookup a kafka broker by id, via a visit to the zookeeper

Since kafka ships with no healthchecks of its own and using docker with jmx you cannot start additional java applications inside the container (you get a port bind error unless you unset/reset the JMX) to do the healtcheck, this simple binary should do the trick.

## How to use
0. Write a oneliner to parse your broker id from kafka server.properties e.g. 
```bash
cat $KAFKA_HOME/config/server.properties | awk 'BEGIN{FS="="}/^broker.id=/{print $2}'
```
1. pass it to zoo_visitor e.g. 
```rust
./zoo_visitor <myid>
``` 
2. receive a line with some useless info or an exception / exit 1 if that broker id is not in the list of brokers (i.e. not up)

Note: the zk server:ip will be read from the environment `ZOOKEEPER_SERVERS` otherwise it defaults to localhost:2181 - which works ok for --net=host or bundled zk/kafka containers