use std::fs;

pub fn is_in_process_tree(child_pid: u32, parent_pid: u32) -> bool {
    if child_pid == parent_pid {
        return true;
    }

    match get_parent_pid(child_pid) {
        0 => false,
        pid => is_in_process_tree(pid, parent_pid),
    }
}

fn get_parent_pid(pid: u32) -> u32 {
    let stat_path = format!("/proc/{}/stat", pid);
    let stat_content = fs::read_to_string(stat_path).expect("Failed to read /proc/<pid>/stat");
    let fields: Vec<&str> = stat_content.split_whitespace().collect();
    fields[3]
        .parse::<u32>()
        .expect("Failed to parse pid from /proc/<pid>/stat")
}
