use sysinfo::{System, SystemExt, ProcessExt, Networks, NetworkExt};

//investigate stats that can be used here.
// std::io::net;

const CPU_STRESS_CODE: i64 = 10; 
const CPU_STRESS_WEIGHT: f64 = 2_f64;
const NETWORK_STRESS_CODE: i64 = 20;
const NETWORK_STRESS_WEIGHT: f64 = 0.001_f64;
const DISK_STRESS_CODE: i64 = 30;
const DISK_STRESS_WEIGHT: f64 = 0.001_f64;

#[derive(Debug, Clone, Copy)]
struct MetricElement {
    code: i64,
    value: f64,
    weight: f64
}

fn main() {
    println!("Quality Metric = {}", get_quality_metric());
}

fn get_quality_metric() -> f64 {
    let qm_matrix = Vec::<MetricElement>::new();
    let sys = System::new_all();

    let (sys, qm_matrix) = get_cpu_stress(sys, qm_matrix);
    let (sys, qm_matrix)  = get_network_stress(sys, qm_matrix);
    let (sys, qm_matrix)  = get_disk_stress(sys, qm_matrix);

    let mut qm = 0_f64;
    for metric in qm_matrix {
        qm = qm + (metric.value * metric.weight);
        println!("Processing Code: {} with Quality Value: {} and Weight: {} Total: {}", code_to_string(metric.code), metric.value, metric.weight, metric.value*metric.weight);
    }
    qm
}

// This function gets the current CPU load on the system.
fn get_cpu_stress(sys: System, mut quality_matrix: Vec::<MetricElement>) -> (System, Vec<MetricElement>) {
    let loadav =  sys.load_average();

    let cpu_stress = loadav.one; //using the average over the last 15 minutes
    let metric_entry = MetricElement {
            code: CPU_STRESS_CODE,
            value: cpu_stress,
            weight: CPU_STRESS_WEIGHT
    };
    quality_matrix.push(metric_entry);
    (sys, quality_matrix)
}

//This function gets the current network load on the system
fn get_network_stress(mut sys: System, mut quality_matrix: Vec::<MetricElement>) -> (System, Vec<MetricElement>) {

    sys.refresh_networks_list();
    let networks = sys.networks();

    let mut packets_in = 0;
    let mut packets_out = 0;
    for (interface_name, network) in networks {
        packets_in = packets_in + network.received();
        packets_out = packets_out + network.transmitted();
    }

    //TODO: add network card capabilities to the metric. cards with > network capacity should get a lower stress number.

    let metric_entry = MetricElement{
        code: NETWORK_STRESS_CODE,
        value: (packets_in as f64) + (packets_out as f64),
        weight: NETWORK_STRESS_WEIGHT
    };

    quality_matrix.push(metric_entry);
    (sys, quality_matrix)
}


fn get_disk_stress(mut sys: System, mut quality_matrix: Vec::<MetricElement>) -> (System, Vec<MetricElement>) {
    //use systemstat::{System, Platform, saturating_sub_bytes};
    //let mut sys = System::new();

    /*let stats = sys.block_device_statistics();
    for blkstats in stats { 
        println!("{:?}", blkstats);
    }*/

    sys.refresh_all();
    // We display all disks' information:
    /*println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
    println!("\n");*/

    /*match sys.block_device_statistics() {
        Ok(stats) => {
            for blkstats in stats.values() {
                println!("{}: {:?}", blkstats.name, blkstats);
            }
        }
        Err(x) => println!("\nBlock statistics error: {}", x.to_string())
    }*/

    // Sum up the disk usage measured as read and writes per process:
    let mut total_usage = 0_u64;
    for (pid, process) in sys.processes() {
        //println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        let usage = process.disk_usage();
        total_usage = total_usage + usage.written_bytes + usage.read_bytes;
    }

    let metric_entry = MetricElement{
        code: DISK_STRESS_CODE,
        value: total_usage as f64,
        weight: DISK_STRESS_WEIGHT
    };

    quality_matrix.push(metric_entry);
    (sys, quality_matrix)
}

//convert a quality code to a string
fn code_to_string(code: i64) -> String {
    let string_code = match code {
        CPU_STRESS_CODE => {String::from("CPU_STRESS_CODE")},
        NETWORK_STRESS_CODE => {String::from("NETWORK_STRESS_CODE")},
        DISK_STRESS_CODE => {String::from("DISK_STRESS_CODE")},
        _ => {String::from("UNKNOWN CODE")}
    };
    string_code
}


#[macro_use]
extern crate more_asserts;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn cpu_load_test(){
        use sysinfo::{System, SystemExt, Networks, NetworkExt};
        use std::thread;
        use std::time::Duration;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let qm_matrix = Vec::<MetricElement>::new();
        let mut sys = System::new_all();
        sys.refresh_all();
        let loading = Arc::new(AtomicBool::new(true));
        let loading_test = loading.clone();

        let (sys, qm_matrix) = get_cpu_stress(sys, qm_matrix);
        let mut qm = 0_f64;
        for metric in qm_matrix {
            qm = qm + (metric.value*metric.weight);
        }
        assert_ne!(0_f64,qm); //zero should never be returned here

        let handle = thread::spawn(move|| {
            let mut cpu_fire = 0;
            while loading_test.load(Ordering::Relaxed) {
                cpu_fire = cpu_fire + 1;
            }
        });

        thread::sleep(Duration::from_millis(2000)); //let cpu spin up
        let qm2_matrix = Vec::<MetricElement>::new();
        let mut sys2 = System::new_all();
        sys2.refresh_all();
        let (sys2, qm2_matrix) = get_cpu_stress(sys2, qm2_matrix);
        let mut qm2 = 0_f64;
        for metric in qm2_matrix {
            qm2 = qm2 + (metric.value*metric.weight);
        }

        assert_gt!(qm2, qm);
        loading.store(false, Ordering::Relaxed);  //kill thread
        handle.join().unwrap();                   //wait for thread
    }
}