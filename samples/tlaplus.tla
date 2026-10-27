--------------------------- MODULE Presence ---------------------------
EXTENDS Naturals, FiniteSets

CONSTANT Nodes
VARIABLE live, replicas

vars == <<live, replicas>>

Init == /\ live = {}
        /\ replicas = [n \in Nodes |-> {}]

Join(n) == /\ n \notin live
           /\ live' = live \cup {n}
           /\ UNCHANGED replicas

Replicate(n) == /\ n \in live
                /\ replicas' = [replicas EXCEPT ![n] = live]
                /\ UNCHANGED live

Next == \E n \in Nodes : Join(n) \/ Replicate(n)

TypeOK == /\ live \subseteq Nodes
          /\ replicas \in [Nodes -> SUBSET Nodes]

Spec == Init /\ [][Next]_vars
EventuallyReplicated == \A n \in Nodes : n \in live ~> n \in replicas[n]

\* A PlusCal algorithm embedded in a TLA+ module.
(* --algorithm Heartbeat
variables ticks = 0;
begin
  Tick:
    while ticks < 3 do
      ticks := ticks + 1;
    end while;
end algorithm; *)

=======================================================================
