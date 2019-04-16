# Running rmem

To run the executable model for one of the examples in `examples`,
run:

  `rmem/rmem -model promising -model promise_first -priority_reduction true -interactive false -shallow_embedding true -prune_late_writes true -shared_memory {location to shared_memory.txt} -pp_hex true {location to a litmus file}`

with `{location to shared_memory.txt}` and `{location to a litmus
file}` filled in to run a test from the example directory and compute
the set of allowed outcomes. E.g.

 `rmem/rmem -model promising -model promise_first
-priority_reduction true -interactive false -shallow_embedding true
-prune_late_writes true -shared_memory examples/DQ/shared_memory.txt
-new_run -pp_hex true examples/DQ/deque-100-1-0.litmus`

executes a small instance of the DQ example. For convenience, the
example directory contains a script for doing this: running (in the
example directory) `./run.sh DQ/deque-100-1-0.litmus` will do the same
as the above command.

Running the command or the script, once the tool is finished, it will
produce an output showing the possible final outcomes of the program:
each of the outcomes shows an execution's final state for certain
registers and memory. Which registers and memory are shown in the
output of the tool is determined by the postcondition of the *.litmus
file. In the case of the DQ examples this postcondition says

`exists (0:X30=0 /\ 1:X30=0 /\ 2:X30=0 /\ t0_res1 = 0 /\ t0_res2 = 0 /\ t0_res3 = 0 /\ t1_res = 0 /\ t2_res = 0 /\ queue_struct = 0 )`

but we are using this only to instruct the tool to print the state for
these registers and memory; the values can be ignored.
 
(The output of the tool for the tests will usually report "Deadlock
states ..". This is expected behaviour also for tests without ARMv8
load/store exclusive instructions and can be ignored -- it is then due
to an optimised implementation of certification used by the model in
non-promise mode, no actual model deadlocks.)

For each possible final state the tool also shows a trace: "via ..".
Appending `-interactive true -cmds 'set always_print true; set follow_list {TRACE}` to the above command allows interactively stepping through a trace
leading to the outcome, where "{TRACE}" should be replaced with the
string (excluding the quotation marks) after the "via" for that outcome,
for example:

`/rmem/rmem -model promising -model promise_first -priority_reduction true -interactive false -shallow_embedding true -prune_late_writes true -shared_memory examples/DQ/shared_memory.txt -new_run -pp_hex true examples/DQ/deque-100-1-0.litmus -interactive true -cmds 'set always_print true; set follow_list 0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0'`

Then repeatedly press "Enter" to follow this trace in the
model. (Also, type "?" followed by "Enter" for printing information
about other available functions of the user interface, which are not
detailed in the paper). Typing "u" (for "undo") followed by "Enter"
steps one step back.


The `examples` directory also contains the script
`runfast.sh`, which does the same as `run.sh`, but which addtionally
applies an optimisation that in non-promise-mode computes the possible
final thread states more efficiently, but which currently does not
support returning an interactive trace with as much detail as
`run.sh`. We are planning to improve this.
