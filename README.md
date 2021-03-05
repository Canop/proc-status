[![MIT][s2]][l2] [![Latest Version][s1]][l0] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/proc-status.svg
[l1]: https://crates.io/crates/proc-status

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/proc-status/badge.svg
[l3]: https://docs.rs/proc-status/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

# proc-status

basic process information

The data comes from `/proc/<pid>/process` and is only
available on unix-like systems.

This crate aims at keeping very simple.
If it doesn't cover your needs, you should probably have a look
at the much more complete [procfs](https://crates.io/crates/procfs).

# Examples:

## Dump memory info about the current process:

```
let mem = proc_status::mem_usage().unwrap();
println!("Mem usage in bytes: current={}, peak={}", mem.current, mem.peak);
```
This prints something like

```stdout
Mem usage in bytes: current=1232896, peak=141430784
```


## Print all the fields of the current process' status:

```
use proc_status::ProcStatus;

let ps = ProcStatus::read().unwrap();
for entry in ps.entries() {
    let entry = entry.unwrap();
    println!("{} = {:?}", entry.key, entry.value);
}
```

## Get the raw value of specific entries

```
use proc_status::ProcStatus;

let ps = ProcStatus::read().unwrap();
println!("State: {:?}", ps.value("State").unwrap());
println!("VmPeak in bytes: {:?}", ps.value_KiB("VmPeak").unwrap() * 1024);
```
