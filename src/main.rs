const VERSION: &str = env!("CARGO_PKG_VERSION");
const SCRATCHPAD_LENGTH: usize = 65536;
const NUM_RUNS: usize = 1024;

fn main() {
    println!("Number Benchmark v{VERSION}");
    println!("Scratchpad length:{SCRATCHPAD_LENGTH} Rounds:{NUM_RUNS}");

    let inital_time = std::time::Instant::now();
    
    perform_integer_addition_benchmarks();
    perform_float_addition_benchmarks();

    let total_elapsed = inital_time.elapsed();
    println!("Total runtime: {:.2?}",total_elapsed)
}

/// Takes average on a array of u128 values.
fn average_u128(data: &[u128]) -> f64 {
    data.iter().sum::<u128>() as f64 / data.len() as f64
}

/// Computes standard deviation from a list u128 values.
fn std_deviation_u128(data: &[u128]) -> f64 {
    let data_mean = average_u128(data);
    let variance = data.iter().map(|value| {
        let diff = data_mean - (*value as f64);
        diff * diff    
        }).sum::<f64>() / data.len() as f64;
    variance.sqrt()
}

// Runs all wrapping integer addition benchmarks
fn perform_integer_addition_benchmarks(){
    println!("Performing wrapping integer addition benchmark.");
    all_rounds_u8_addition_benchmark();
    all_rounds_u16_addition_benchmark();
    all_rounds_u32_addition_benchmark();
    all_rounds_u64_addition_benchmark();
    all_rounds_u128_addition_benchmark();
}

// Runs all float addition benchmarks
fn perform_float_addition_benchmarks(){
    println!("Performing float addition benchmark on values between 0 and 1.");
    all_rounds_f32_short_range_addition_benchmark();
    all_rounds_f64_short_range_addition_benchmark();

    println!("Performing float addition benchmark on any float value.");
    all_rounds_f32_long_range_addition_benchmark();
    all_rounds_f64_long_range_addition_benchmark();
}

/// Performs the u8 addition benchmark NUM_RUNS times and prints results.
fn all_rounds_u8_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_u8_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("u8   Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the u8 addition benchmark.
fn perform_u8_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0u8; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_u8(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_u8_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the u16 addition benchmark NUM_RUNS times and prints results.
fn all_rounds_u16_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_u16_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("u16  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the u16 addition benchmark.
fn perform_u16_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0u16; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_u16(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_u16_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the u32 addition benchmark NUM_RUNS times and prints results.
fn all_rounds_u32_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_u32_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("u32  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the u32 addition benchmark.
fn perform_u32_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0u32; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_u32(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_u32_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the u64 addition benchmark NUM_RUNS times and prints results.
fn all_rounds_u64_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_u64_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("u64  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the u64 addition benchmark.
fn perform_u64_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0u64; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_u64(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_u64_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the u128 addition benchmark NUM_RUNS times and prints results.
fn all_rounds_u128_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_u128_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("u128 Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the u128 addition benchmark.
fn perform_u128_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0u128; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_u128(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_u128_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the f32 addition benchmark NUM_RUNS times on values 0..1 and prints results.
fn all_rounds_f32_short_range_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_f32_short_range_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("f32  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the f32 addition benchmark on values 0..1.
fn perform_f32_short_range_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0f32; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_f32(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_f32_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the f32 addition benchmark NUM_RUNS times on any float and prints results.
fn all_rounds_f32_long_range_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_f32_long_range_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("f32  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the f32 addition benchmark on any float.
fn perform_f32_long_range_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0f32; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_any_f32(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_f32_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the f64 addition benchmark NUM_RUNS times on values 0..1 and prints results.
fn all_rounds_f64_short_range_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_f64_short_range_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("f64  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the f64 addition benchmark on values 0..1.
fn perform_f64_short_range_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0f64; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_f64(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_f64_addition(&scratchpad);
    }
    now.elapsed()
}

/// Performs the f64 addition benchmark NUM_RUNS times on any float and prints results.
fn all_rounds_f64_long_range_addition_benchmark(){
    let mut result_nanosec_times = [0u128; NUM_RUNS];
    
    for indx in 0..(NUM_RUNS){
        result_nanosec_times[indx] = perform_f64_long_range_addition_benchmark().as_nanos();
    }

    let avg_time = std::time::Duration::from_nanos(average_u128(&result_nanosec_times) as u64);
    let std_dev_time = std::time::Duration::from_nanos(std_deviation_u128(&result_nanosec_times) as u64);

    println!("f64  Average: {:.2?} Std.dev: {:.2?}", avg_time, std_dev_time);
}

/// Performs the f64 addition benchmark on any float.
fn perform_f64_long_range_addition_benchmark() -> std::time::Duration {
    let mut scratchpad = [0f64; SCRATCHPAD_LENGTH*2];
    randomly_fill_scratchpad_any_f64(&mut scratchpad);

    let now = std::time::Instant::now();
    {
        compute_f64_addition(&scratchpad);
    }
    now.elapsed()
}

/// Fills a scratchpad with random u128 data.
fn randomly_fill_scratchpad_u128(pad: &mut [u128; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::u128(..);
    }
}

/// Fills a scratchpad with random u64 data.
fn randomly_fill_scratchpad_u64(pad: &mut [u64; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::u64(..);
    }
}

/// Fills a scratchpad with random u32 data.
fn randomly_fill_scratchpad_u32(pad: &mut [u32; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::u32(..);
    }
}

/// Fills a scratchpad with random u16 data.
fn randomly_fill_scratchpad_u16(pad: &mut [u16; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::u16(..);
    }
}

/// Fills a scratchpad with random u8 data.
fn randomly_fill_scratchpad_u8(pad: &mut [u8; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::u8(..);
    }
}

/// Fills a scratchpad with random f64 values between 0 and 1.
fn randomly_fill_scratchpad_f64(pad: &mut [f64; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::f64();
    }
}

/// Fills a scratchpad with totally random f64 values. (Including inf, nan etc.)
fn randomly_fill_scratchpad_any_f64(pad: &mut [f64; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = f64::from_bits(fastrand::u64(..));
    }
}

/// Fills a scratchpad with random f32 values between 0 and 1.
fn randomly_fill_scratchpad_f32(pad: &mut [f32; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = fastrand::f32();
    }
}

/// Fills a scratchpad with totally random f32 values. (Including inf, nan etc.)
fn randomly_fill_scratchpad_any_f32(pad: &mut [f32; SCRATCHPAD_LENGTH*2]) {
    for value in pad {
        *value = f32::from_bits(fastrand::u32(..));
    }
}

/// Runs through every even value of the scratchpad and performs wrapping addition with the one that follows it.
fn compute_u128_addition(pad: &[u128; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value.wrapping_add(pad[ind+1]));
    }
}

/// Runs through every even value of the scratchpad and performs wrapping addition with the one that follows it.
fn compute_u64_addition(pad: &[u64; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value.wrapping_add(pad[ind+1]));
    }
}

/// Runs through every even value of the scratchpad and performs wrapping addition with the one that follows it.
fn compute_u32_addition(pad: &[u32; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value.wrapping_add(pad[ind+1]));
    }
}

/// Runs through every even value of the scratchpad and performs wrapping addition with the one that follows it.
fn compute_u16_addition(pad: &[u16; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value.wrapping_add(pad[ind+1]));
    }
}

/// Runs through every even value of the scratchpad and performs wrapping addition with the one that follows it.
fn compute_u8_addition(pad: &[u8; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value.wrapping_add(pad[ind+1]));
    }
}

/// Runs through every even value of the scratchpad and performs float addition with the one that follows it.
fn compute_f64_addition(pad: &[f64; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value + pad[ind+1]);
    }
}

/// Runs through every even value of the scratchpad and performs float addition with the one that follows it.
fn compute_f32_addition(pad: &[f32; SCRATCHPAD_LENGTH*2]) {
    for (ind, value) in pad.iter().enumerate().step_by(2) {
        std::hint::black_box(value + pad[ind+1]);
    }
}