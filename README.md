this is an ultra biased benchmark that tests how many loop cycles fit into
a single time slice on linux.

there's a couple reasons why this is a __TERRIBLE__ benchmark and should be only
used as a very very rough idea about what is going on:
1. it's linux
2. i'm currently listening to nirvana
3. there's a browser(!!!) running on the system somewhere
4. no winsor (given the extreme behavior of threads, this is quite important)

## theorem for the dumb

#### increased contention for cpu resources

when you increase the number of threads, you create more contention for the cpu resources. this can lead to the operating system's scheduler giving each thread less frequent but potentially longer time slices to run, as it tries to manage the higher number of competing threads. as a result, each thread might perform more iterations of the `while` loop before being preempted.

#### overhead of context switching

context switching involves overhead, including saving and restoring the state of each thread. with more threads, the scheduler has to perform more context switches. if the system's load increases, the operating system may optimize by reducing the frequency of context switches to improve overall performance, leading to longer time slices for each thread. this results in a higher number of iterations between yields.

#### yield behavior with high thread counts

when you call `thread::yield_now()`, you are hinting to the scheduler that the current thread is willing to give up its time slice. however, with more threads, the chance that the scheduler immediately picks another thread increases. if the scheduler is heavily loaded, it might handle the yield calls differently to maintain system responsiveness, potentially leading to fewer immediate context switches and more iterations of the loop.

#### scheduler heuristics and prioritization

the operating system's scheduler uses heuristics to balance thread execution. with more threads, the scheduler might prioritize certain threads differently based on their activity and the overall system load. this could result in some threads getting longer uninterrupted runs, especially if the system interprets yield calls as low-priority signals under high load.

#### system load and background processes

with a higher number of threads, the overall system load increases. background processes and system tasks might affect the behavior of thread scheduling. if the system is under heavy load, it might give more cpu time to the threads running your benchmark to maintain performance, leading to more iterations before the `stop_flag` is checked again.
