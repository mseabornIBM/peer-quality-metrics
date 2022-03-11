use systemstat::{System, Platform, saturating_sub_bytes};

//investigate stats that can be used here.
// std::io::net;

const CPU_STRESS_CODE: i32 = 10; 
const CPU_STRESS_WEIGHT:f32 = 2_f32;

#[derive(Debug, Clone, Copy)]
struct MetricElement {
    code: i32,
    value: f32,
    weight: f32
}

fn main() {
    println!("Quality Metric = {}", get_quality_metric());
}

fn get_quality_metric() -> f32 {
    let mut qm_matrix = Vec::<MetricElement>::new();
    qm_matrix = get_cpu_stress(qm_matrix);

    let mut qm = 0_f32;
    for metric in qm_matrix {
        println!("Processing Code: {}", code_to_string(metric.code));
        qm = qm + (metric.value * metric.weight);
    }
    qm
}

fn get_cpu_stress(mut quality_matrix: Vec::<MetricElement>) -> Vec<MetricElement> {
    let sys = System::new();
    let loadav =  sys.load_average().unwrap();

    let cpu_stress = (loadav.one + loadav.five + loadav.fifteen) / 3_f32;
    let metric_entry = MetricElement {
            code: CPU_STRESS_CODE,
            value: cpu_stress,
            weight: CPU_STRESS_WEIGHT
    };
    quality_matrix.push(metric_entry);
    quality_matrix
}

fn code_to_string(code: i32) -> String {
    let string_code = match code {
        CPU_STRESS_CODE => {String::from("CPU_STRESS_CODE")},
        _ => {String::from("UNKNOWN CODE")}
    };
    string_code
}