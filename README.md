# Quality Metrics for Pyrsia
This is a prototype to flesh out a peer quality metric that can be used in pyrsia to help determine the "best" peer to choose and artifact from. The goal is to design a metric that can be used by peers to advertise the stress level on the hosting computer. The metric is a summation of weighted values where the higher the number the more stress is being applied to the host. Upon integration to Pyrsia, clients will download the artifact from the host with the lowest stress quality index. When complete, this prototype will sum the weighted attributes in the following table.

|Peer Attribute| Definition | Weight |
|---|---|---|
| CPU Load | The percentage of CPU load on the peer in question | 2 |
| Network Load| The current number of packets in + current number of packets out | 0.001 |
 
TODO: Need to mix in MTU size and network card bandwidth to the metric.