use cortex_m::asm::delay as busy_delay;

pub fn busy_delay_ms(cpu_hz: u32, ms: u32) {
    let cycles = (cpu_hz as u64 * ms as u64) / 1000;
    let cycles = cycles.max(1) as u32;
    busy_delay(cycles);
}
