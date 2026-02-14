How can a system upgrade itself automatically as new features come online?
Not new nodes in the network, as the system should be designed around computers coming online and offline, and joining the network. I mean: 
- How can an update be propagated to every node? 
- How can updates be generated in the first place?
The idea of a network receiving a request to extend itself in features, deploy them along the network, debug itself, and then continue is fascinating and only possible recently.
# The naive approach
Simply push to a repository the changes, and send an update signal to every node. When a node receives the update signal, it spins up another script that will pull, build the app, and make it start running again. But this clearly has issues: what if the script can't pull from the repository? What if it can't build the app? What if the changes drop every node on the network at the same time? Then how would it recover?

From this the idea of a slow rollout seems obvious. Make the change on one machine, see how it goes, then continue. That way, failures don't kill the network.

# The monitor system
A single node is chosen as the monitor, it will connect directly to another node, then order it to update itself. If, at any point, something fails, there should be a logging system (in what machine is not decided, but it makes sense that the updatee keeps them locally in case the connection drops, and the updater somehow can retrieve them) so the monitor can debug what is happening. 

# Open questions:
The network will keep working while this happens, what happens if one node is in version 3 and another in 2? What if it receives a request for something it can't do? Or an older version of that request?
No idea