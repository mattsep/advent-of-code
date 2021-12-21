#![allow(dead_code, unused_assignments)]

use std::fs;
use std::io;
use std::path::Path;

enum Packet {
    Literal(u8, u8, u64),
    Operator(u8, u8, Vec<Packet>),
}

fn hex_to_bits(hex: u8) -> [bool; 4] {
    [
        (hex & (1 << 3)) != 0,
        (hex & (1 << 2)) != 0,
        (hex & (1 << 1)) != 0,
        (hex & (1 << 0)) != 0,
    ]
}

fn bits_to_value(bits: &[bool]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, b)| (*b as u64) << (i as u64))
        .fold(0, |a, b| a + b)
}

fn parse<P: AsRef<Path>>(path: P) -> io::Result<Vec<bool>> {
    let line = fs::read_to_string(path)?;
    let line = line.trim();
    let result = line
        .chars()
        .map(|c| hex_to_bits(c.to_digit(16).unwrap() as u8))
        .flatten()
        .collect::<Vec<_>>();
    Ok(result)
}

fn parse_literal(bits: &[bool]) -> (u64, &[bool]) {
    let mut value: u64 = 0;
    let mut skip: usize = 0;
    for group in bits.chunks(5) {
        skip += 5;
        value = (value << 4) | bits_to_value(&group[1..]);
        if !group[0] {
            break;
        }
    }

    while (6 + skip) % 4 != 0 {
        skip += 1
    }

    (value, &bits[skip..])
}

fn parse_packet(bits: &mut [bool]) -> Packet {
    let version = bits_to_value(&bits[0..3]) as u8;
    let typeid = bits_to_value(&bits[3..6]) as u8;

    let mut bits = &mut bits[6..];
    if typeid == 4 {
        // Literal
        Packet::Literal(version, typeid, parse_literal(&mut bits))
    } else {
        // Some operation + operands
        let mut packets = Vec::new();
        if bits[0] {
            // Total number of packets in next 11 bits
            let num_packets = bits_to_value(&bits[1..12]);
            bits = &mut bits[12..];
            for _ in 0..num_packets {}
            packets.push(parse_packet(&mut bits));
            Packet::Operator(version, typeid, packets)
        } else {
            // Total bit-length of packets in next 15 bits
            Packet::Literal(0, 0, 0)
        }
    }
}

pub fn solve_it() {
    let mut bits = parse("input/test.txt").unwrap();
}
