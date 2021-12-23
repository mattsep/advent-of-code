#![allow(dead_code, unused_assignments)]

use core::num;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
struct Metadata {
    version: u8,
    type_id: u8,
}

#[derive(Debug)]
enum Packet {
    Literal(Metadata, u64),
    Operator(Metadata, Vec<Packet>),
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

    (value, &bits[skip..])
}

fn parse_packet(bits: &[bool]) -> (Packet, &[bool]) {
    println!("Bit stream length: {}", bits.len());
    let version = bits_to_value(&bits[0..3]) as u8;
    let type_id = bits_to_value(&bits[3..6]) as u8;
    let metadata = Metadata { version, type_id };
    let packet: Packet;
    let mut bits = &bits[6..];
    if type_id == 4 {
        // Literal
        let (value, slice) = parse_literal(bits);
        packet = Packet::Literal(metadata, value);
        bits = slice;
    } else {
        // Some operation + operands
        let mut packets = Vec::new();
        if bits[0] {
            // Total number of packets in next 11 bits
            let num_packets = bits_to_value(&bits[1..12]);
            println!("Expecting {} packets", num_packets);
            bits = &bits[12..];
            for _ in 0..num_packets {
                let (packet, slice) = parse_packet(&bits);
                packets.push(packet);
                bits = slice;
            }
        } else {
            // Total bit-length of packets in next 15 bits
            let num_bits = bits_to_value(&bits[1..16]) as usize;
            println!("Expecting payload of {} bits", num_bits);
            bits = &bits[16..];
            loop {
                let (packet, slice) = parse_packet(&bits);
                packets.push(packet);
                bits = slice;
            }
        }

        packet = Packet::Operator(metadata, packets);
    }

    println!("Found packet: {:?}", packet);
    (packet, bits)
}

fn get_version_sum(packet: &Packet) -> i32 {
    match packet {
        Packet::Literal(metadata, _) => metadata.version as i32,
        Packet::Operator(metadata, packets) => {
            let mut sum = metadata.version as i32;
            for packet in packets {
                sum += get_version_sum(packet);
            }
            sum
        }
    }
}

pub fn solve_it() {
    let bits = parse("input/test.txt").unwrap();
    bits.iter().map(|b| *b as i32).for_each(|c| print!("{}", c)); println!("");
    
    let (packet, _) = parse_packet(&bits[..]);    

    println!("{:?}", packet);
    println!("{}", get_version_sum(&packet));
}
