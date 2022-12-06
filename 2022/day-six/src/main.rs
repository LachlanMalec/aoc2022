use std::fs;

fn main() {
    let datastream_buffer = fs::read("data").expect("Unable to read file");

    println!("Part 1: {}", identify_start_of_packet_marker_position(&datastream_buffer));

    println!("Part 2: {}", identify_start_of_message_marker_position(&datastream_buffer));
}

fn identify_start_of_packet_marker_position(datastream_buffer: &[u8]) -> usize {
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

fn identify_start_of_message_marker_position(datastream_buffer: &[u8]) -> usize {
    let mut marker_position: Option<usize> = None;
    for (i, _) in datastream_buffer.iter().enumerate() {
        let mut unique_chars = 0;
        for j in 0..14 {
            let mut unique = true;
            for k in 0..14 {
                if j != k && datastream_buffer[i + j] == datastream_buffer[i + k] {
                    unique = false;
                    break;
                }
            }
            if unique {
                unique_chars += 1;
            }
        }
        if unique_chars == 14 {
            marker_position = Some(i + 14);
            break;
        }
    }
    marker_position.expect("Unable to find marker")
}
