use anyhow::{Context, Result};
use indexmap::IndexMap;
use nvml_wrapper::enums::device::UsedGpuMemory::Used;
use nvml_wrapper::NVML;
use sysinfo::{Pid, PidExt, System, SystemExt};

fn main() -> Result<()> {
    let nvml = NVML::init().context("unable to initialize NVIDIA Management Library (NVML)")?;
    let gpu_count = nvml.device_count()?;
    let mut s = System::new();
    for idx in 0..gpu_count {
        let device = nvml.device_by_index(idx)?;
        let meminfo = device.memory_info()?;
        println!(
            "{}: {}\t mem usage: {:.2}/{:.2}GiB ({:.2}%)",
            idx,
            device.name()?,
            meminfo.used as f64 / (1u32 << 30) as f64,
            meminfo.total as f64 / (1u32 << 30) as f64,
            100. * meminfo.used as f64 / meminfo.total as f64
        );
        let mut usage = IndexMap::new();
        let mut processes = device.running_compute_processes()?;
        processes.extend(device.running_graphics_processes()?);
        for process in processes {
            if let Used(used_mem) = process.used_gpu_memory {
                let pid = Pid::from_u32(process.pid);
                s.refresh_process(pid);
                if let Some(process) = s.process(pid) {
                    *usage.entry((process.uid, process.gid)).or_insert(0) += used_mem;
                }
            }
        }
        for ((uid, gid), memuse) in usage.sorted_unstable_by(|_, v1, _, v2| v2.cmp(v1)) {
            let user = users::get_user_by_uid(uid).unwrap();
            let group = users::get_group_by_gid(gid).unwrap();
            println!(
                "\t{}:{}: {:.2}%",
                user.name().to_str().unwrap(),
                group.name().to_str().unwrap(),
                100. * memuse as f64 / meminfo.total as f64
            );
        }
    }
    Ok(())
}
