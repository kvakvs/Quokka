Quokka Observer
===============

## The idea: APP

This is a Native GUI application which takes stream of tracing data from file, or a stream
of events from a living Erlang (BEAM VM) node or a cluster, and attempts to rebuild an actual
picture of "reality" and display it in somewhat useful way.

Idea is to provide search, grouping and introspection tools which allow the observing person
to make sense of what is happening in the running node (or what happened in the past). Also
to be able to replay, slow, pause and rewind the event stream and see what happened at any
specific moment in the past and which events have led to it.

NOTE: This does not exist yet. Work started on the data model and displaying trace data and
time-aggregated trace data (flame graph output).

[//]: <> (## The idea: AGENTS)

[//]: <> (NOTE: These do not exist yet.)

[//]: <> (The agent would be injectable code modules injected by Quokka into a running Erlang cluster or installed by the user, which are able to collect the vital data for visualisation. Example: node resource state, running processes and OTP apps, collect trace events, logs, etc.)
