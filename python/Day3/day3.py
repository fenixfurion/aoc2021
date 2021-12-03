#!/usr/bin/env python

def day3_part1(data):
    return [int(gamma,2) * (int(gamma,2)^(int('1'*len(data[0]),2))) for gamma in [''.join(['0' if counts.count('0') > len(data)/2 else '1' for counts in [list(i) for i in zip(*data)]])]][0]

# sort of annotated
def day3_part2_annotated(data):
    # print(data)
    # temp = [list(i) for i in zip(*data)]
    # print(temp)
    # #temp = list(filter(lambda a: [True if counts.count('0') >= len(data)/2 else False for counts in [[elem[0] for elem in [[character for character in bitstring] for bitstring in data]]]], data))
    # # temp = ['0' if counts.count('0') >= len(data)/2 else '1' for counts in [[elem[0] for elem in [[character for character in bitstring] for bitstring in data]]]][0]
    # temp = ['0' if counts.count('0') > len(data)/2 else '1' for counts in [[elem[bit_index] for elem in [list(i) for i in zip(*data)]] for bit_index in range(len(data[0]))]]
    # print(temp)
    # temp = list(filter(lambda a: a[0] == ['0' if counts.count('0') > len(data)/2 else '1' for counts in [[elem[bit_index] for elem in [list(i) for i in zip(*data)]] for bit_index in range(len(data[0]))]][0], data))
    # # keep ones
    # index = 0
    # # temp = elem[i] == ['0' if counts.count('0') > len(data)/2 else '1' for counts in [list(i) for i in zip(*data)]][i]
    # print("Counts: ", temp)
    # # clone data into two parts for CO2 and O2 scrubber
    # # temp = [[elem for elem in data], [elem for elem in data]]
    # # print(temp)
    # # self-referential list comprehensions and we also need to keep track of index somehow
    # #temp = [[data.remove(elem) for i in range(len(data[0])) for elem in data if (elem[i] == ['0' if counts.count('0') > len(data)/2 else '1' for counts in [list(i) for i in zip(*data)]][i]) and len(elem)>1], [elem for elem in data]]
    # # [data.remove(elem) for i in range(1) for elem in data if (elem[i] == ['0' if counts.count('0') > len(data)/2 else '1' for counts in [list(i) for i in zip(*data)]][i]) and len(data)>1]
    # # this is technically a two-liner since i have to return something
    # # doubles data into data[0] and data[1]
    # temp = [iter if type(data[0]) == list else data.pop(0) if type(data[-1]) == list else data.extend([[e for e in data], [e for e in data]]) for iter in range(len(data)+1)]
    # print("temp: ", temp)
    # print("Cloned:", data)
    # use dummy side to do garbage to the 0th index
    return [int(data[-2][0], 2) * int(data[-1][0], 2) for iter in [(side, data_index, bit_index, [elem for elem in data[side-2]], data[side-2].pop(data_index)) if type(data[-1]) == list and side !=2 else data.extend([[e for e in data], [e for e in data]]) if type(data[-1]) != list else (side, data_index, bit_index, [elem for elem in data[side-2]], 'waiting') for side in [2, 0, 1] for bit_index in range(len(data[side-2][0])) for data_index in [index for index, elem in enumerate(data[side-2]) if len(data[side-2])>1 and elem[bit_index] != ['0' if (side == 0) and (counts.count('0') > (len(data[side-2])/2)) else '1' if (side==0) else '0' if counts.count('0') <= counts.count('1') else '1' for counts in [list(i) for i in zip(*data[side-2])]][bit_index]][::-1]][1]][0]

def day3_part2(data):
    return [int(data[-2][0], 2) * int(data[-1][0], 2) for iter in [data[side-2].pop(data_index) if type(data[-1]) == list and side !=2 else data.extend([[e for e in data], [e for e in data]]) if type(data[-1]) != list else [0] for side in [2, 0, 1] for bit_index in range(len(data[side-2][0])) for data_index in [index for index, elem in enumerate(data[side-2]) if len(data[side-2])>1 and elem[bit_index] != ['0' if (side == 0) and (counts.count('0') > (len(data[side-2])/2)) else '1' if (side==0) else '0' if counts.count('0') <= (len(data[side-2])/2) else '1' for counts in [list(i) for i in zip(*data[side-2])]][bit_index]][::-1]][1]][0]

if __name__ == '__main__':
    #              0        1        2        3        4        5        6        7        8        9        10       11
    sample_data = ['00100', '11110', '10110', '10111', '10101', '01111', '00111', '11100', '10000', '11001', '00010', '01010']
    # O2 - remove less
    # bit 0, more 1 (7) than 0 (5)
    # remove: index 11, 10, 6, 5, 0
    #               0        1        2        3        4        5        6        7        8        9        10       11
    #sample_data = ['11110', '10110', '10111', '10101', '11100', '10000', '11001']
    # bit 1, more 0 (4) than 1 (3)
    # remove: index 6, 4, 0
    #               0        1        2        3        4        5        6        7        8        9        10       11
    #sample_data = ['10110', '10111', '10101', '10000']
    # bit 2, more 1 (3) than 0 (1)
    # remove: index 3
    #               0        1        2        3        4        5        6        7        8        9        10       11
    #sample_data = ['10110', '10111', '10101']
    # kill logic -> 
    #O2: side = 0, kill smaller group (keep 1s)
    # elem[bit_index] != ['0' if counts.count('0') >= ((len(data[side-2])/2)) else '1' for counts in [list(i) for i in zip(*data[side-2])]]
    # zero count > half

    #CO2: side = 1, kill larger group (keep 0s)
    # more 0s
    

    data = [line.strip() for line in open('input.txt','r').readlines()]
    print("AOC 2021 Day 3 python mode: welcome to hell")
    print(f"Part 1 - Sample: {day3_part1(sample_data)}")
    print(f"Part 1 - Puzzle: {day3_part1(data)}")
    print(f"Part 1 - Sample: {day3_part2(sample_data)}")
    print(f"Part 1 - Sample: {day3_part2(data)}")
    # out = day3_part2(sample_data)
    # print(sample_data)
    # last_elem = out[1][2]
    # for elem in out:
    #     if elem and len(elem)>=3:
    #         if elem[2] != last_elem:
    #             print()
    #         last_elem = elem[2]
    #     print(elem)