# Shadow Configuration Specification

Shadow uses the standard YAML format to accept configuration options from users.
The following describes Shadow's YAML format and all of the options that Shadow
supports that can be used to customize a simulation.

Example:

```yaml
general:
  stop_time: 2 min
network:
  graph:
    type: gml
    inline: |
      graph [
        node [
          id 0
          host_bandwidth_down "140 Mbit"
          host_bandwidth_up "18 Mbit"
        ]
        edge [
          source 0
          target 0
          latency "50 ms"
          packet_loss 0.01
        ]
      ]
hosts:
  server:
    network_node_id: 0
    processes:
    - path: /usr/sbin/nginx
      args: -c ../../../nginx.conf -p .
      start_time: 1
  client:
    network_node_id: 0
    quantity: 20
    options:
      log_level: debug
    processes:
    - path: /usr/bin/curl
      args: server --silent
      start_time: 5
```

## Contents:

- [`general`](#general)
- [`general.bootstrap_end_time`](#generalbootstrap_end_time)
- [`general.data_directory`](#generaldata_directory)
- [`general.heartbeat_interval`](#generalheartbeat_interval)
- [`general.log_level`](#generallog_level)
- [`general.parallelism`](#generalparallelism)
- [`general.progress`](#generalprogress)
- [`general.seed`](#generalseed)
- [`general.stop_time`](#generalstop_time)
- [`general.template_directory`](#generaltemplate_directory)
- [`network`](#network)
- [`network.graph`](#networkgraph)
- [`network.graph.type`](#networkgraphtype)
- [`network.graph.<file|inline>`](#networkgraphfileinline)
- [`network.graph.file.path`](#networkgraphfilepath)
- [`network.graph.file.compression`](#networkgraphfilecompression)
- [`network.use_shortest_path`](#networkuse_shortest_path)
- [`experimental`](#experimental)
- [`experimental.host_heartbeat_interval`](#experimentalhost_heartbeat_interval)
- [`experimental.host_heartbeat_log_info`](#experimentalhost_heartbeat_log_info)
- [`experimental.host_heartbeat_log_level`](#experimentalhost_heartbeat_log_level)
- [`experimental.interface_buffer`](#experimentalinterface_buffer)
- [`experimental.interface_qdisc`](#experimentalinterface_qdisc)
- [`experimental.interpose_method`](#experimentalinterpose_method)
- [`experimental.preload_spin_max`](#experimentalpreload_spin_max)
- [`experimental.runahead`](#experimentalrunahead)
- [`experimental.scheduler_policy`](#experimentalscheduler_policy)
- [`experimental.socket_recv_autotune`](#experimentalsocket_recv_autotune)
- [`experimental.socket_recv_buffer`](#experimentalsocket_recv_buffer)
- [`experimental.socket_send_autotune`](#experimentalsocket_send_autotune)
- [`experimental.socket_send_buffer`](#experimentalsocket_send_buffer)
- [`experimental.strace_logging_mode`](#experimentalstrace_logging_mode)
- [`experimental.unblocked_syscall_latency`](#experimentalunblocked_syscall_latency)
- [`experimental.unblocked_syscall_limit`](#experimentalunblocked_syscall_limit)
- [`experimental.use_cpu_pinning`](#experimentaluse_cpu_pinning)
- [`experimental.use_dynamic_runahead`](#experimentaluse_dynamic_runahead)
- [`experimental.use_explicit_block_message`](#experimentaluse_explicit_block_message)
- [`experimental.use_legacy_working_dir`](#experimentaluse_legacy_working_dir)
- [`experimental.use_libc_preload`](#experimentaluse_libc_preload)
- [`experimental.use_memory_manager`](#experimentaluse_memory_manager)
- [`experimental.use_o_n_waitpid_workarounds`](#experimentaluse_o_n_waitpid_workarounds)
- [`experimental.use_object_counters`](#experimentaluse_object_counters)
- [`experimental.use_preload_openssl_crypto`](#experimentaluse_preload_openssl_crypto)
- [`experimental.use_preload_openssl_rng`](#experimentaluse_preload_openssl_rng)
- [`experimental.use_sched_fifo`](#experimentaluse_sched_fifo)
- [`experimental.use_shim_syscall_handler`](#experimentaluse_shim_syscall_handler)
- [`experimental.use_seccomp`](#experimentaluse_seccomp)
- [`experimental.use_syscall_counters`](#experimentaluse_syscall_counters)
- [`experimental.worker_threads`](#experimentalworker_threads)
- [`host_defaults`](#host_defaults)
- [`host_defaults.log_level`](#host_defaultslog_level)
- [`host_defaults.pcap_capture_size`](#host_defaultspcap_capture_size)
- [`host_defaults.pcap_directory`](#host_defaultspcap_directory)
- [`hosts`](#hosts)
- [`hosts.<hostname>.bandwidth_down`](#hostshostnamebandwidth_down)
- [`hosts.<hostname>.bandwidth_up`](#hostshostnamebandwidth_up)
- [`hosts.<hostname>.ip_addr`](#hostshostnameip_addr)
- [`hosts.<hostname>.network_node_id`](#hostshostnamenetwork_node_id)
- [`hosts.<hostname>.options`](#hostshostnameoptions)
- [`hosts.<hostname>.quantity`](#hostshostnamequantity)
- [`hosts.<hostname>.processes`](#hostshostnameprocesses)
- [`hosts.<hostname>.processes[*].args`](#hostshostnameprocessesargs)
- [`hosts.<hostname>.processes[*].environment`](#hostshostnameprocessesenvironment)
- [`hosts.<hostname>.processes[*].path`](#hostshostnameprocessespath)
- [`hosts.<hostname>.processes[*].quantity`](#hostshostnameprocessesquantity)
- [`hosts.<hostname>.processes[*].start_time`](#hostshostnameprocessesstart_time)
- [`hosts.<hostname>.processes[*].stop_time`](#hostshostnameprocessesstop_time)

#### `general`

*Required*

General experiment settings.

#### `general.bootstrap_end_time`

Default: "0 sec"  
Type: String OR Integer

The simulated time that ends Shadow's high network bandwidth/reliability
bootstrap period.

If the bootstrap end time is greater than 0, Shadow uses a simulation
bootstrapping period where hosts have unrestricted network bandwidth and no
packet drop. This can help to bootstrap large networks quickly when the network
hosts have low network bandwidth or low network reliability.

#### `general.data_directory`

Default: "shadow.data"  
Type: String

Path to store simulation output.

#### `general.heartbeat_interval`

Default: "1 sec"  
Type: String OR Integer OR null

Interval at which to print heartbeat messages.

#### `general.log_level`

Default: "info"  
Type: "error" OR "warning" OR "info" OR "debug" OR "trace"

Log level of output written on stdout. If Shadow was built in release mode, then
messages at level 'trace' will always be dropped.

#### `general.parallelism`

Default: 1  
Type: Integer

How many parallel threads to use to run the simulation. Optimal performance is
usually obtained with `nproc`, or sometimes `nproc/2` with hyperthreading.

Virtual hosts depend on network packets that can potentially arrive from other
virtual hosts, so each worker can only advance according to the propagation
delay to avoid dependency violations. Therefore, not all threads will have 100%
CPU utilization.

#### `general.progress`

Default: false  
Type: Bool

Show the simulation progress on stderr.

When running in a tty, the progress will be updated every second and shown at
the bottom of the terminal. Otherwise the progress will be printed without ANSI
escape codes at intervals which increase as the simulation progresses.

#### `general.seed`

Default: 1  
Type: Integer

Initialize randomness using seed N.

#### `general.stop_time`

*Required*  
Type: String OR Integer

The simulated time at which simulated processes are sent a SIGKILL signal.

#### `general.template_directory`

Default: null  
Type: String OR null

Path to recursively copy during startup and use as the data-directory.

#### `network`

*Required*

Network settings.

#### `network.graph`

*Required*

The network topology graph.

A network topology represented by a connected graph with certain attributes
specified on the network nodes and edges. For more information on how to
structure this data, see the [Network Graph Overview](network_graph_overview.md).

Example:

```yaml
network:
  graph:
    type: gml
    inline: |
      graph [
        ...
      ]
```

#### `network.graph.type`

*Required*  
Type: "gml" OR "1\_gbit\_switch"

The network graph can be specified in the GML format, or a built-in
"1\_gbit\_switch" graph with a single network node can be used instead.

The built-in "1\_gbit\_switch" graph contains the following:

```text
graph [
  directed 0
  node [
    id 0
    host_bandwidth_up "1 Gbit"
    host_bandwidth_down "1 Gbit"
  ]
  edge [
    source 0
    target 0
    latency "1 ms"
    packet_loss 0.0
  ]
]
```

#### `network.graph.<file|inline>`

*Required if `network.graph.type` is "gml"*  
Type: Object OR String

If the network graph type is not a built-in network graph, the graph data can be
specified as a path to an external file, or as an inline string.

#### `network.graph.file.path`

*Required*  
Type: String

The path to the file.

If the path begins with `~/`, it will be considered relative to the current
user's home directory.

#### `network.graph.file.compression`

Default: null  
Type: "xz" OR null

The file's compression format.

#### `network.use_shortest_path`

Default: true  
Type: Bool

When routing packets, follow the shortest path rather than following a direct
edge between network nodes. If false, the network graph is required to be
complete (including self-loops) and to have exactly one edge between any two
nodes.

#### `experimental`

Experimental experiment settings. Unstable and may change or be removed at any
time, regardless of Shadow version.

#### `experimental.host_heartbeat_interval`

Default: null  
Type: String OR Integer OR null

Amount of time between heartbeat messages for this host.

#### `experimental.host_heartbeat_log_info`

Default: ["node"]  
Type: Array of ("node" OR "socket" OR "ram")

List of information to show in the host's heartbeat message.

#### `experimental.host_heartbeat_log_level`

Default: "info"  
Type: "error" OR "warning" OR "info" OR "debug" OR "trace"

Log level at which to print host statistics.

#### `experimental.interface_buffer`

Default: "1024000 B"  
Type: String OR Integer

Size of the interface receive buffer that accepts incoming packets.

#### `experimental.interface_qdisc`

Default: "fifo"  
Type: "fifo" OR "roundrobin"

The queueing discipline to use at the network interface.

#### `experimental.interpose_method`

Default: "preload"  
Type: "ptrace" OR "preload"

Which interposition method to use.

#### `experimental.preload_spin_max`

Default: 0  
Type: Integer

Max number of iterations to busy-wait on IPC semaphore before blocking.

#### `experimental.runahead`

Default: "1 ms"  
Type: String OR null

If set, overrides the automatically calculated minimum time workers may run
ahead when sending events between virtual hosts.

#### `experimental.scheduler_policy`

Default: "host"  
Type: "host" OR "steal" OR "thread" OR "threadxthread" OR "threadxhost"

The event scheduler's policy for thread synchronization.

#### `experimental.socket_recv_autotune`

Default: true  
Type: Bool

Enable receive window autotuning.

#### `experimental.socket_recv_buffer`

Default: "174760 B"  
Type: String OR Integer

Initial size of the socket's receive buffer.

#### `experimental.socket_send_autotune`

Default: true  
Type: Bool

Enable send window autotuning.

#### `experimental.socket_send_buffer`

Default: "131072 B"  
Type: String OR Integer

Initial size of the socket's send buffer.

#### `experimental.strace_logging_mode`

Default: "off"  
Type: "off" OR "standard" OR "deterministic"

Log the syscalls for each process to individual "strace" files.

The mode determines the format that the syscalls are logged in. For example,
the "deterministic" mode will avoid logging memory addresses or potentially
uninitialized memory.

The logs will be stored at
`shadow.data/hosts/<hostname>/<hostname>.<procname>.<pid>.strace`.

Limitations:

- Syscalls handled within Shadow's preloaded "shim" library will not be logged
  (for example `SYS_gettimeofday`).
- Syscalls run natively will not log the syscall arguments or return value (for
  example `SYS_getcwd`).
- Syscalls processed within Shadow's C code will not log the syscall arguments.
- Syscalls that are interrupted by a signal may not be logged (for example
  `SYS_read`).
- Syscalls that are interrupted by a signal may be logged inaccurately. For
  example, the log may show `syscall(...) = -1 (EINTR)`, but the managed
  process may not actually see this return value. Instead the syscall may be
  restarted.

#### `experimental.unblocked_syscall_latency`

Default: "2 microseconds"  
Type: String

The simulated latency of an unblocked syscall. For simulation efficiency, this
latency is only added when `unblocked_syscall_limit` is reached.

#### `experimental.unblocked_syscall_limit`

Default: 0
Type: Integer

If non-zero, the number of consecutive unblocked syscalls that Shadow will
allow a thread to execute before `unblocked_syscall_latency` is applied.
Setting this to a non-zero value allows time to eventually move forward in cases
where the managed thread is in a "busy loop" that executes forever until a
timeout or some other condition is satisfied.

#### `experimental.use_cpu_pinning`

Default: true  
Type: Bool

Pin each thread and any processes it executes to the same logical CPU Core to
improve cache affinity.

#### `experimental.use_dynamic_runahead`

Default: false  
Type: Bool

Update the minimum runahead dynamically throughout the simulation.

#### `experimental.use_explicit_block_message`

Default: false  
Type: Bool

Send message to managed process telling it to stop spinning when a syscall
blocks.

#### `experimental.use_legacy_working_dir`

Default: false  
Type: Bool

Don't adjust the working directories of the virtual processes.

#### `experimental.use_libc_preload`

Default: true  
Type: Bool

Preload our libc library for all managed processes for fast syscall
interposition when possible.

#### `experimental.use_memory_manager`

Default: true  
Type: Bool

Use the MemoryManager. It can be useful to disable for debugging, but will hurt
performance in most cases.

#### `experimental.use_o_n_waitpid_workarounds`

Default: false  
Type: Bool

Use performance workarounds for waitpid being O(n). Beneficial to disable if
waitpid is patched to be O(1), if using one logical processor per host, or in
some cases where it'd otherwise result in excessive detaching and reattaching.

#### `experimental.use_object_counters`

Default: true  
Type: Bool

Count object allocations and deallocations. If disabled, we will not be able to
detect object memory leaks.

#### `experimental.use_preload_openssl_crypto`

Default: false  
Type: Bool

Preload our OpenSSL crypto library for all managed processes to skip some AES
crypto operations, which may speed up simulation if your CPU lacks AES-NI
support. However, it changes the behavior of your application and can cause bugs
in OpenSSL that are hard to notice. You should probably not use this option
unless you really know what you're doing.

#### `experimental.use_preload_openssl_rng`

Default: true  
Type: Bool

Preload our OpenSSL RNG library for all managed processes to mitigate
non-deterministic use of OpenSSL.

#### `experimental.use_sched_fifo`

Default: false  
Type: Bool

Use the `SCHED_FIFO` scheduler. Requires `CAP_SYS_NICE`. See sched(7),
capabilities(7).

#### `experimental.use_shim_syscall_handler`

Default: true  
Type: Bool

Use shim-side syscall handler to force hot-path syscalls to be handled via an
inter-process syscall with Shadow.

#### `experimental.use_seccomp`

Default: true iff `experimental.interpose_method == preload`.
Type: Bool

Use seccomp to trap syscalls.

#### `experimental.use_syscall_counters`

Default: false  
Type: Bool

Count the number of occurrences for individual syscalls.

#### `experimental.worker_threads`

Default: # of hosts in the simulation  
Type: Integer

Create N worker threads. Note though, that `general.parallelism` of them will be
allowed to run simultaneously. If unset, will create a thread for each simulated
Host. This is to work around limitations in ptrace, and may change in the
future.

#### `host_defaults`

Default options for all hosts. These options can also be overridden for each
host individually in the host's [`hosts.<hostname>.options`](#hostshostnameoptions)
section.

#### `host_defaults.log_level`

Default: null  
Type: "error" OR "warning" OR "info" OR "debug" OR "trace" OR null

Log level at which to print host log messages.

#### `host_defaults.pcap_capture_size`

Default: "65535 B"  
Type: String OR Integer

How much data to capture per packet (header and payload) if pcap logging is
enabled.

The default of 65535 bytes is the maximum length of an IP packet.

#### `host_defaults.pcap_directory`

Default: null  
Type: String OR null

Where to save the pcap files (relative to the host directory).

Logs all network input and output for this host in PCAP format (for viewing in
e.g. wireshark). The directory must already exist and be relative to the host
directory, although absolute paths are also allowed. Example:
`pcap_directory: '.'` will generate pcap files such as
`shadow.data/hosts/myhost/myhost-11.0.0.1.pcap`.

#### `hosts`

*Required*  
Type: Object

The simulated hosts which execute processes. Each field corresponds to a host
configuration, with the field name being used as the network hostname.

Shadow assigns each host to a network node in the [network graph](network_graph_overview.md).

#### `hosts.<hostname>.bandwidth_down`

Default: null  
Type: String OR Integer OR null

Downstream bandwidth capacity of the host.

Overrides any default bandwidth values set in the assigned network graph
node.

#### `hosts.<hostname>.bandwidth_up`

Default: null  
Type: String OR Integer OR null

Upstream bandwidth capacity of the host.

Overrides any default bandwidth values set in the assigned network graph
node.

#### `hosts.<hostname>.ip_addr`

Default: null  
Type: String OR null

IP address to assign to the host.

This IP address must not conflict with the address of any other host (two hosts
must not have the same IP address). If this option is set,
[`hosts.<hostname>.quantity`](#hostshostnamequantity) must be set to 1.

#### `hosts.<hostname>.network_node_id`

*Required*  
Type: Integer

Network graph node ID to assign the host to.

#### `hosts.<hostname>.options`

See [`host_defaults`](#host_defaults) for supported fields.

Example:

```yaml
hosts:
  client:
    ...
    options:
      log_level: debug
```

#### `hosts.<hostname>.quantity`

Default: 1  
Type: Integer

Number of hosts to start.

If quantity is greater than 1, each host's hostname will be suffixed with a
counter. For example, a host with an id of `host` and quantity of 2 would
produce hosts with hostnames `host1` and `host2`.

#### `hosts.<hostname>.processes`

*Required*  
Type: Array

Virtual software processes that the host will run.

#### `hosts.<hostname>.processes[*].args`

Default: ""  
Type: String OR Array of String

Process arguments.

The arguments can be specified as a string in a shell command-line format:

```yaml
args: "--user-agent 'Mozilla/5.0 (compatible; ...)' http://myserver:8080"
```

Or as an array of strings:

```yaml
args: ['--user-agent', 'Mozilla/5.0 (compatible; ...)', 'http://myserver:8080']
```

#### `hosts.<hostname>.processes[*].environment`

Default: ""  
Type: String

Environment variables passed when executing this process. Multiple variables can
be specified by using a semicolon separator (ex: `ENV_A=1;ENV_B=2`).

#### `hosts.<hostname>.processes[*].path`

*Required*  
Type: String

If the path begins with `~/`, it will be considered relative to the current
user's home directory.

#### `hosts.<hostname>.processes[*].quantity`

Default: 1  
Type: Integer

The number of replicas of this process to execute.

#### `hosts.<hostname>.processes[*].start_time`

Default: "0 sec"  
Type: String OR Integer

The simulated time at which to execute the process.

#### `hosts.<hostname>.processes[*].stop_time`

Default: null  
Type: String OR Integer OR null

The simulated time at which to send a SIGKILL signal to the process.
