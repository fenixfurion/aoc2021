#!/usr/bin/env python3
import sys

def day7(fn):
    result = sys.maxsize
    data = [int(line) for line in open(fn,'r').read().strip().split(',')]
    for i in data:
        total_sum_to_i = 0
        for k in data:
            total_sum_to_i += abs(i-k)
        if total_sum_to_i < result:
            result = total_sum_to_i
    return result

def day7_part2(fn):
    result = sys.maxsize
    data = [int(line) for line in open(fn,'r').read().strip().split(',')]
    for i in range(max(data)):
        total_sum_to_i = 0
        for k in data:
            delta = abs(i-k)
            tri = (delta*(delta+1))/2
            # print(tri)
            total_sum_to_i += tri
        if total_sum_to_i < result:
            result = total_sum_to_i
    return int(result)

# i-k = delta
# delta[k] = 0, 1, 3, 6, 10, 15

if __name__ == '__main__':
    print("AOC Day 7 in python: gimme stars first")
    filename = './input_sample.txt'
    result = day7(filename)
    print(result)
    result = day7_part2(filename)
    print(result)
    filename = './input.txt'
    result = day7(filename)
    print(result)
    result = day7_part2(filename)
    print(result)
