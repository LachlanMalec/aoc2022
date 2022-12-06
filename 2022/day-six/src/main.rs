use std::fs;

fn main() {
    let datastream_buffer = fs::read("data").expect("Unable to read file");

    println!("Part 1: {}", identify_marker_position(&datastream_buffer, Marker::StartOfPacket));

    println!("Part 2: {}", identify_marker_position(&datastream_buffer, Marker::StartOfMessage));
}

enum Marker {
    StartOfPacket,
    StartOfMessage,
}

fn identify_marker_position(datastream_buffer: &[u8], marker: Marker) -> usize {
    let marker_length = match marker {
        Marker::StartOfPacket => 4,
        Marker::StartOfMessage => 14,
    };
    let mut marker_position: Option<usize> = None;
    for (i, _) in datastream_buffer.iter().enumerate() {
        let mut unique_chars = 0;
        for j in 0..marker_length {
            let mut unique = true;
            for k in 0..marker_length {
                if j != k && datastream_buffer[i + j] == datastream_buffer[i + k] {
                    unique = false;
                    break;
                }
            }
            if unique {
                unique_chars += 1;
            }
        }
        if unique_chars == marker_length {
            marker_position = Some(i + marker_length);
            break;
        }
    }
    marker_position.expect("Unable to find marker")
}
