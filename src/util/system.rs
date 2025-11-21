use std::fs;

use crate::lotto::lotto::{LOTTO_PRICE, Lotto};
use std::cmp::min;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

const PADDING: u64 = 60;
pub const MAX_PURCHASE_AMOUNT: u64 = 100_000_000_000;
const LINUX_MEMORY_CURRENT_PATH: &str = "/sys/fs/cgroup/memory.current";
const LINUX_MEMORY_MAX_PATH: &str = "/sys/fs/cgroup/memory.max";

pub fn get_memories_by_os() -> (String, u64, u64, u64) {
    let (os, total_memory, available_memory) = match std::env::consts::OS {
        "linux" => get_linux_sys_info(),
        "macos" => get_mac_sys_info(),
        "windows" => get_win_sys_info(),
        _ => ("Unknown".to_string(), 0, 0),
    };

    let max_purchase_amount = get_max_purchase_amount(available_memory);
    (os, total_memory, available_memory, max_purchase_amount)
}

fn get_linux_sys_info() -> (String, u64, u64) {
    let mem_used = read_value(LINUX_MEMORY_CURRENT_PATH);
    let mem_limit = read_value(LINUX_MEMORY_MAX_PATH);

    ("Linux".to_string(), mem_limit, mem_limit - mem_used)
}

fn get_mac_sys_info() -> (String, u64, u64) {
    let sys =
        System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));

    (
        "MAC".to_string(),
        sys.total_memory(),
        sys.total_memory() - sys.used_memory(),
    )
}

fn get_win_sys_info() -> (String, u64, u64) {
    let sys =
        System::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()));

    (
        "Windows".to_string(),
        sys.total_memory(),
        sys.available_memory(),
    )
}

fn read_value(path: &str) -> u64 {
    let data = fs::read_to_string(path)
        .expect("memory 파일 읽기에 실패했습니다")
        .trim()
        .to_string();

    if data == "max" {
        return u64::MAX;
    }

    data.parse::<u64>().expect("메모리 값 변환에 실패했습니다")
}

fn get_max_purchase_amount(free_memory: u64) -> u64 {
    min(
        free_memory / Lotto::size_in_bytes() * PADDING / 100 * LOTTO_PRICE,
        MAX_PURCHASE_AMOUNT,
    )
}
