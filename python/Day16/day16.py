#!/usr/bin/env python3

from enum import Enum, auto

def hexstr_to_bits(path):
    try:
        data = open(path, 'r').read().strip()
    except Exception as e:
        # hack to let me load in string literals directly
        data = path
    if len(data) > 100:
        print("Hexstr: (puzzle)")
    else:
        print(f"Hexstr: {data}")
    bitstr = ''.join(['{:04b}'.format(int(elem,16)) for elem in data])
    return bitstr

class PState(Enum):
    STATE_HEADER = auto()
    STATE_LITERAL_VALUE = auto()
    STATE_OPERATOR = auto()

DEBUG = False

def dprint(*args):
    if DEBUG:
        print(*args)


def parse_packet(data, depth=0):
    dprint("-"*10 + " Parse Packet Start " +  "-"*10)
    result = 0
    state = PState.STATE_HEADER
    next_state = PState.STATE_HEADER
    cur = 0
    packets = []
    current_packet = {}
    dstr = depth*"----" + "D{:03}".format(depth) 
    while cur < len(data):
        dprint(30*"-")
        dprint(f"{dstr} State: {state}, Cursor: {cur}")
        if state == PState.STATE_HEADER: 
            dprint(f"{dstr} Data: {data} (len {len(data)})")
            if cur+4 >= len(data):
                break
            current_packet = {}
            current_packet['version'] = int(data[cur:cur+3],2)
            cur += 3
            dprint(f"{dstr} Packet version: {current_packet['version']}")
            current_packet['id'] = int(data[cur:cur+3],2)
            cur += 3
            dprint(f"{dstr} Packet id: {current_packet['id']}")
            if current_packet['id'] == 4:
                next_state = PState.STATE_LITERAL_VALUE
                literal_data = ''
            else:
                next_state = PState.STATE_OPERATOR
        elif state == PState.STATE_LITERAL_VALUE:
            keep_read = data[cur]
            cur += 1
            val = data[cur:cur+4]
            literal_data += val
            dprint(f"{dstr} Literal: Data: {val}")
            cur += 4
            if keep_read == '1':
                # keep reading, next state is LITERAL_VALUE
                next_state = PState.STATE_LITERAL_VALUE
            elif keep_read == '0':
                # done with this literal packet, parse it
                final_literal = int(literal_data,2)
                dprint(f"{dstr} Literal: End: {final_literal}")
                current_packet['data'] = final_literal
                packets.append(current_packet)
                dprint(packets)
                return cur, packets
        elif state == PState.STATE_OPERATOR:
            if cur+1+11 >= len(data):
                break
            length_type_bit = data[cur]
            cur += 1
            if length_type_bit == '0':
                # next 15 bits are the total length in
                # bits of the sub-packets contained here
                sub_packet_length = int(data[cur:cur+15],2)
                cur += 15
                dprint(f"{dstr} Parsing {sub_packet_length} bits of packets")
                start_cur = cur
                current_packet['packets'] = []
                while cur < start_cur+sub_packet_length:
                    sub_packet_data = data[cur:cur+sub_packet_length]
                    bits_read, subpacket = parse_packet(sub_packet_data, depth=depth+1)
                    dprint(f"{dstr} Read {bits_read} bits")
                    current_packet['packets'] += subpacket.copy()
                    cur += bits_read
                packets.append(current_packet)
                return cur, packets
            if length_type_bit == '1':
                # number of sub-packets immediately contained by this packet
                sub_packet_count = int(data[cur:cur+11],2)
                cur += 11
                current_packet['packets'] = []
                dprint(f"{dstr} Parsing {sub_packet_count} packets")
                for i in range(sub_packet_count):
                    sub_packet_data = data[cur:]
                    bits_read, subpacket = parse_packet(sub_packet_data, depth=depth+1)
                    dprint(f"{dstr} Read {bits_read} bits")
                    current_packet['packets'] += subpacket.copy()
                    cur += bits_read
                packets.append(current_packet)
                return cur, packets
        state = next_state
    return cur, packets

def sum_versions(packets):
    sum_val = 0
    # dprint(f"Called sum versions on {packets}")
    for packet in packets:
        # dprint(f"Packet: {packet}")
        sum_val += packet['version']
        if 'packets' in packet.keys():
            sum_val += sum_versions(packet['packets'])
    return sum_val

def part1(filename):
    data = hexstr_to_bits(filename)
    cur, packets = parse_packet(data)
    sum_result = sum_versions(packets)
    return sum_result

def eval_packet(packet, depth=0):
    # dstr = depth*"----" + "D{:03} Eval: ".format(depth) 
    if packet['id'] == 4:
        # literal
        # print(f"{dstr} Literal with value {packet['data']}")
        return packet['data']
    if packet['id'] == 0:
        # sum
        # print(f"{dstr} SUM on packets")
        result = 0
        for subpacket in packet['packets']:
            result += eval_packet(subpacket, depth=depth+1)
        return result
    if packet['id'] == 1:
        # product
        result = 1
        for subpacket in packet['packets']:
            result *= eval_packet(subpacket, depth=depth+1)
        return result
    if packet['id'] == 2:
        # min
        minimum = 99999999
        for subpacket in packet['packets']:
            result = eval_packet(subpacket, depth=depth+1)
            if result < minimum:
                minimum = result
        return minimum
    if packet['id'] == 3:
        # maximum
        maximum = 0
        for subpacket in packet['packets']:
            result = eval_packet(subpacket, depth=depth+1)
            if result > maximum:
                maximum = result
        return maximum
    if packet['id'] == 5:
        # greater
        assert(len(packet['packets']) == 2)
        lhs = eval_packet(packet['packets'][0], depth=depth+1)
        rhs = eval_packet(packet['packets'][1], depth=depth+1)
        return 1 if lhs > rhs else 0
    if packet['id'] == 6:
        # less
        assert(len(packet['packets']) == 2)
        lhs = eval_packet(packet['packets'][0], depth=depth+1)
        rhs = eval_packet(packet['packets'][1], depth=depth+1)
        return 1 if lhs < rhs else 0
    if packet['id'] == 7:
        # eq
        assert(len(packet['packets']) == 2)
        lhs = eval_packet(packet['packets'][0], depth=depth+1)
        rhs = eval_packet(packet['packets'][1], depth=depth+1)
        return 1 if lhs == rhs else 0


def part2(filename):
    data = hexstr_to_bits(filename)
    cur, packets = parse_packet(data)
    # print(packets)
    result = eval_packet(packets[0])
    return result

if __name__ == '__main__':
    # god this is miserable i should have just used string literals instead of values
    print(f"Part 1 on Sample 1:   {part1('input_1.txt')}")
    print(f"Part 1 on Sample 2:   {part1('input_2.txt')}")
    print(f"Part 1 on Sample 3:   {part1('input_3.txt')}")
    print(f"Part 1 on Sample 4:   {part1('input_4.txt')}")
    print(f"Part 1 on Sample 5:   {part1('input_5.txt')}")
    print(f"Part 1 on Sample 6:   {part1('input_6.txt')}")
    print(f"Part 1 on Sample 7:   {part1('input_7.txt')}")
    print(f"Part 1 on Puzzle:     {part1('input.txt')}")
    print(f"Part 2 on Sample 1:   {part2('C200B40A82')}")
    print(f"Part 2 on Sample 2:   {part2('04005AC33890')}")
    print(f"Part 2 on Sample 3:   {part2('880086C3E88112')}")
    print(f"Part 2 on Sample 4:   {part2('CE00C43D881120')}")
    print(f"Part 2 on Sample 5:   {part2('D8005AC2A8F0')}")
    print(f"Part 2 on Sample 6:   {part2('F600BC2D8F')}")
    print(f"Part 2 on Sample 7:   {part2('9C005AC2F8F0')}")
    print(f"Part 2 on Sample 8:   {part2('9C0141080250320F1802104A08')}")
    print(f"Part 2 on Puzzle:     {part2('input.txt')}")
    # print(f"Part 2 on Puzzle:   {part2('input.txt')}")
