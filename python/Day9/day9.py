#!/usr/bin/env python3

def is_lowpoint(data, row, col):
    # skip check up
    is_lowpoint = True
    val = data[row][col]
    if row != 0:
        if data[row-1][col] <= val:
            is_lowpoint = False
    if row != len(data)-1:
        if data[row+1][col] <= val:
            is_lowpoint = False
    if col != 0:
        if data[row][col-1] <= val:
            is_lowpoint = False
    if col != len(data[0])-1:
        if data[row][col+1] <= val:
            is_lowpoint = False
    return is_lowpoint
        

def part1():
    risk = 0
    data = [[int(e) for e in line.strip()] for line in open("input.txt", 'r').readlines()]
    for row in range(len(data)):
        for col in range(len(data[row])):
            if is_lowpoint(data, row, col):
                risk += 1 + data[row][col]
    print(risk)

def get_basin(data, row, col):
    # check each side
    if row != 0:
        if data[row-1][col] <= val:
            is_lowpoint = False
    if row != len(data)-1:
        if data[row+1][col] <= val:
            is_lowpoint = False
    if col != 0:
        if data[row][col-1] <= val:
            is_lowpoint = False
    if col != len(data[0])-1:
        if data[row][col+1] <= val:
            is_lowpoint = False

# assume all rows are equal length
def get_higher_cells(data, row, col):
    higher_cells = set()
    val = data[row][col]
    if row != 0:
        if data[row-1][col] > val and data[row-1][col] < 9:
            higher_cells.add(((row-1,col)))
    if row != len(data)-1:
        if data[row+1][col] > val and data[row+1][col] < 9:
            higher_cells.add(((row+1,col)))
    if col != 0:
        if data[row][col-1] > val and data[row][col-1] < 9:
            higher_cells.add(((row,col-1)))
    if col != len(data[0])-1:
        if data[row][col+1] > val and data[row][col+1] < 9:
            higher_cells.add(((row,col+1)))
    return higher_cells

import time
def printbasin(data, basin):
    # print(basin)
    print("\033[%d;%dH" % (0, 0))
    #for row in range(len(data)):
    for row in range(35,len(data)-40):
        #for col in range(len(data[0])):
        for col in range(0, len(data[0])-60):
            if (row,col) in basin:
                print('\x1b[6;30;42m', end='')
                print(data[row][col], end='')
                print('\x1b[0m', end='')
            else:
                print(data[row][col], end='')
        print('')
    time.sleep(0.25)

# def printdata_heat(data):
#     print(basin)
#     val = data[row][col]
#     color = ''
#     match val:
#         case 0: color = 
#         case 1:
#         case 2:
#         case 3:
#         case 4:
#         case 5:
#         case 6:
#         case 7:
#         case 8:
#         case 9:
#         case _:
#             raise ValueError()
#     for row in range(len(data)):
#         for col in range(len(data[0])):
#             print('\x1b[6;30;42m', end='')
#             print(data[row][col], end='')
#             print('\x1b[0m', end='')
#         print('')

def find_basin(data, row, col):
    last_size = 0
    current_size = 1
    current_basin = set()
    # printbasin(data, current_basin)
    if data[row][col] < 9:
        current_basin = set([(row,col)])
        # print(f"Checking new basin centered on {(row,col)}")
        while last_size < current_size:
            last_size = current_size
            to_add = set()
            for coords in current_basin:
                # print(coords)
                to_add |= get_higher_cells(data, coords[0], coords[1])
            current_basin |= to_add
            # if current_size < len(current_basin):
            #     printbasin(data, current_basin)
            current_size = len(current_basin)
    return current_basin

def part2():
    data = [[int(e) for e in line.strip()] for line in open("input.txt", 'r').readlines()]
    basins = []
    for row in range(len(data)):
        for col in range(len(data[row])):
            new_basin = frozenset(find_basin(data,row,col))
            if len(new_basin) == 98:
                print(f"Biggest basin is at {row}, {col}")
            basins.append(new_basin)
    # for elem in set(basins):
    #     print(f"Basin size: {len(elem)}, basin: {elem}")
    sizes = [(len(elem), elem) for elem in set(basins)]
    sizes.sort(key = lambda x: x[0])
    # print([elem[0] for elem in sizes])
    print(sizes[-1][0] * sizes[-2][0] * sizes[-3][0])
    # for elem in sizes[-3:]:
    #     printbasin(data, elem[1])

def pp():
    data = [[int(e) for e in line.strip()] for line in open("input.txt", 'r').readlines()]
    basin = find_basin(data,43,4)
    print(len(basin))
    

if __name__ == '__main__':
    part1()
    part2()
    # pp()

