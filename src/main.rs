//use systemstat::{System, Platform, saturating_sub_bytes};
use sysinfo::{System, SystemExt, Networks, NetworkExt};

//investigate stats that can be used here.
// std::io::net;

const CPU_STRESS_CODE: i64 = 10; 
const CPU_STRESS_WEIGHT: f64 = 2_f64;
const NETWORK_STRESS_CODE: i64 = 20;
const NETWORK_STRESS_WEIGHT: f64 = 0.001_f64;

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

    let mut qm = 0_f64;
    for metric in qm_matrix {
        qm = qm + (metric.value * metric.weight);
        println!("Processing Code: {} with Quality Value: {} and Weight: {}", code_to_string(metric.code), metric.value, metric.weight);
    }
    qm
}

fn get_cpu_stress(sys: System, mut quality_matrix: Vec::<MetricElement>) -> (System, Vec<MetricElement>) {
    let loadav =  sys.load_average();

    let cpu_stress = loadav.fifteen; //using the average over the last 15 minutes
    let metric_entry = MetricElement {
            code: CPU_STRESS_CODE,
            value: cpu_stress,
            weight: CPU_STRESS_WEIGHT
    };
    quality_matrix.push(metric_entry);
    (sys, quality_matrix)
}

//received and transmitted elements return current transmission rates
fn get_network_stress(mut sys: System, mut quality_matrix: Vec::<MetricElement>) -> (System, Vec<MetricElement>) {

    sys.refresh_networks_list();
    let networks = sys.networks();

    let mut packets_in = 0;
    let mut packets_out = 0;
    for (interface_name, network) in networks {
        packets_in = packets_in + network.received();
        packets_out = packets_out + network.transmitted();
    }
    
    let metric_entry = MetricElement{
        code: NETWORK_STRESS_CODE,
        value: (packets_in as f64) + (packets_out as f64),
        weight: NETWORK_STRESS_WEIGHT
    };

    quality_matrix.push(metric_entry);
    (sys, quality_matrix)
}

fn code_to_string(code: i64) -> String {
    let string_code = match code {
        CPU_STRESS_CODE => {String::from("CPU_STRESS_CODE")},
        NETWORK_STRESS_CODE => {String::from("NETWORK_STRESS_CODE")},
        _ => {String::from("UNKNOWN CODE")}
    };
    string_code
}