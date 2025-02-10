
fn humanized_size(size: u64) -> (f64, &'static str) {
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
    
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    (size, units[unit_index])
}

#[test]
fn test_humanized_size() {
    let byte_size = 2157087073;
    let (size, unit) = humanized_size(byte_size);
    assert_eq!("Size :  2.0089 GiB", format!("Size :  {:0.4} {}", size, unit));
}
