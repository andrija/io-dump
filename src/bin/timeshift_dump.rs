extern crate io_dump;

use io_dump::write_packet;
use io_dump::Packets;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let packets = Packets::new(std::io::stdin());
    let dest = std::io::stdout();
    let mut shift_time: Option<Duration> = None;

    for packet in packets {
        if shift_time == None {
            shift_time = Some(packet.elapsed());
        }

        let _ = write_packet(
            &dest,
            packet.direction(),
            packet.data(),
            packet.elapsed() - shift_time.unwrap(),
        );
    }

    Ok(())
}
