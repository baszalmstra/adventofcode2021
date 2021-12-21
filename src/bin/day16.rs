use bitstream_io::{BigEndian, BitRead, BitReader, Endianness};
use hex::FromHex;
use std::io::Cursor;

#[derive(Debug)]
struct Packet {
    version: u8,
    kind: PacketKind,
}

impl Packet {
    fn version_sum(&self) -> usize {
        if let PacketKind::Operator((_, sub_packets)) = &self.kind {
            self.version as usize + sub_packets.iter().map(Packet::version_sum).sum::<usize>()
        } else {
            self.version as usize
        }
    }

    fn value(&self) -> usize {
        match &self.kind {
            PacketKind::Literal(value) => *value,
            PacketKind::Operator((0, sub_packets)) => sub_packets.iter().map(Packet::value).sum(),
            PacketKind::Operator((1, sub_packets)) => {
                sub_packets.iter().map(Packet::value).product()
            }
            PacketKind::Operator((2, sub_packets)) => {
                sub_packets.iter().map(Packet::value).min().unwrap()
            }
            PacketKind::Operator((3, sub_packets)) => {
                sub_packets.iter().map(Packet::value).max().unwrap()
            }
            PacketKind::Operator((5, sub_packets)) => {
                if sub_packets[0].value() > sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketKind::Operator((6, sub_packets)) => {
                if sub_packets[0].value() < sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            PacketKind::Operator((7, sub_packets)) => {
                if sub_packets[0].value() == sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PacketKind {
    Literal(usize),
    Operator((u8, Vec<Packet>)),
}

fn read_operator<R: std::io::Read, E: Endianness>(
    reader: &mut BitReader<R, E>,
) -> anyhow::Result<(Vec<Packet>, usize)> {
    let length_type_id = reader.read_bit()?;
    let mut sub_packets = Vec::new();
    let mut bits_read = 1;
    if length_type_id {
        let num_packets: u16 = reader.read(11)?;
        bits_read += 11;
        sub_packets.reserve(num_packets as usize);
        for _ in 0..num_packets {
            let (packet, packet_bits_read) = read_packet(reader)?;
            sub_packets.push(packet);
            bits_read += packet_bits_read;
        }
    } else {
        let num_bits: u16 = reader.read(15)?;
        bits_read += 15;
        let mut packet_bits_read = 0;
        while packet_bits_read < num_bits as usize {
            let (packet, sub_packet_bits_read) = read_packet(reader)?;
            sub_packets.push(packet);
            packet_bits_read += sub_packet_bits_read;
        }
        assert_eq!(packet_bits_read, num_bits as usize);
        bits_read += packet_bits_read;
    }

    Ok((sub_packets, bits_read))
}

fn read_literal<R: std::io::Read, E: Endianness>(
    reader: &mut BitReader<R, E>,
) -> anyhow::Result<(usize, usize)> {
    let mut value = 0;
    let mut bits_read = 0;
    loop {
        let has_more = reader.read_bit()?;
        let part: u8 = reader.read(4)?;
        value = value << 4 | part as usize;
        bits_read += 5;
        if !has_more {
            return Ok((value, bits_read));
        }
    }
}

fn read_packet<R: std::io::Read, E: Endianness>(
    reader: &mut BitReader<R, E>,
) -> anyhow::Result<(Packet, usize)> {
    let version: u8 = reader.read(3)?;
    let type_id: u8 = reader.read(3)?;

    let (kind, bits_read) = match type_id {
        4 => {
            let (literal, bits_read) = read_literal(reader)?;
            (PacketKind::Literal(literal), bits_read)
        }
        _ => {
            let (sub_packets, bits_read) = read_operator(reader)?;
            (PacketKind::Operator((type_id, sub_packets)), bits_read)
        }
    };

    Ok((Packet { version, kind }, bits_read + 6))
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("inputs/day16/input")?;
    let bytes = Vec::from_hex(input)?;
    let mut reader = BitReader::endian(Cursor::new(&bytes), BigEndian);

    let (packet, _) = read_packet(&mut reader)?;

    println!("Solution 1: {}", packet.version_sum());
    println!("Solution 1: {}", packet.value());

    Ok(())
}
