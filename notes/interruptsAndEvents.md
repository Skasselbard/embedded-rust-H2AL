# Heading

Interrupts should only produce events (that will be handled later).
Device crates have to implement the enable/disable_interrupts function for a critical section around event enqueuing.

Only interrupts should enqueue events.
Only threads should dequeue events.
Enqueuing cannot be preempted and will not be preempted due to critical section.
Dequeueing should be safe due to Heapless::queue lock free access.
Multi core scenarios are probably safe as well due to the lock free access.
