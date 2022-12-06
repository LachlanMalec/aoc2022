use std::fs;

fn main() {
    let datastream_buffer = fs::read("data").expect("Unable to read file");

    println!("Marker position: {}", identify_marker_position(&datastream_buffer));
}

fn identify_marker_position(datastream_buffer: &[u8]) -> usize {
    let mut marker_position: Option<usize> = None;
    for (i, _) in datastream_buffer.iter().enumerate() {
        let mut unique_chars = 0;
        for j in 0..4 {
            let mut unique = true;
            for k in 0..4 {
                if j != k && datastream_buffer[i + j] == datastream_buffer[i + k] {
                    unique = false;
                    break;
                }
            }
            if unique {
                unique_chars += 1;
            }
        }
        if unique_chars == 4 {
            marker_position = Some(i + 4);
            break;
        }
    }
    marker_position.expect("Unable to find marker")
}
