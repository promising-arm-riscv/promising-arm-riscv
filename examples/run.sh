#!/usr/bin/env bash

if [ $# -lt 1 ]
then
  echo "usage : $0 litmus"
  exit 1
fi

ldir=`dirname $1`

bin/rmem -model promising -model promise_first -priority_reduction true -interactive false -shallow_embedding true -prune_late_writes true -shared_memory $ldir/shared_memory.txt -new_run -pp_hex true $1
