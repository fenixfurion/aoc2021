#!/usr/bin/env python3
import math 
def read_input(filename):
    with open(filename, 'r') as fd:
        data = {}
        line = fd.read().strip().split(': ')[1]
        line = line.split(', ')
        xvals = line[0].split('=')[1].split('..')
        yvals = line[1].split('=')[1].split('..')
        # print(xvals, yvals)
        # print(line)
        data['xmin'] = int(xvals[0])
        data['xmax'] = int(xvals[1])
        data['ymin'] = int(yvals[0])
        data['ymax'] = int(yvals[1])
    return data

def day17_part1(filename):
    bounds = read_input(filename)
    x_vel = 0
    y_vel = 0
    max_height = 0
    successful_shots = []
    for x_vel in range(0, bounds['xmax']+10):
        for y_vel in range(bounds['ymin'], -bounds['ymin']+10):
            success, shot_height = shoot(x_vel, y_vel, bounds)
            if success:
                if shot_height > max_height:
                    max_height = shot_height
                successful_shots.append((x_vel, y_vel))
    print(f"There were {len(successful_shots)} total successful shots")
    return max_height

def shoot(x_vel, y_vel, bounds):
    x = 0
    y = 0
    init_x_vel = x_vel
    init_y_vel = y_vel
    y_max = y
    # print(10*"-" + "NEW SHOT" + 10*"-")
    # print(f"Shot initial condition: x=0, y=0, x_vel={x_vel}, y_vel={y_vel}")
    step = 0
    while not overshot(x, y, bounds):
        y_max = y if y > y_max else y_max
        x = x+x_vel
        y = y+y_vel
        x_vel = 0 if x_vel == 0 else x_vel - 1 if x_vel > 0 else x_vel + 1
        y_vel = y_vel-1
        # print(f"After step {step}: x={x}, y={y}, x_vel={x_vel}, y_vel={y_vel}")
        if in_range(x, y, bounds):
            # print(f"Shot {init_x_vel},{init_y_vel} SUCCEEDED")
            return True, y_max
        step += 1
    # print(f"Shot {init_x_vel},{init_y_vel} FAILED")
    return False, 0

def in_range(x, y, bounds):
    in_range = True
    if x not in range(bounds['xmin'], bounds['xmax']+1):
        in_range = False
    if y not in range(bounds['ymin'], bounds['ymax']+1):
        in_range = False
    return in_range

def approx_distance(x, y, bounds):
    return math.sqrt(pow(x-bounds['xmin'],2)+pow(y-bounds['ymin'],2))

def overshot(x, y, bounds):
    if y < bounds['ymin']:
        return True 
    else:
        return False

if __name__ == '__main__':
    result = day17_part1('input_sample.txt')
    print(f"Part 1 Sample: {result}")
    result = day17_part1('input.txt')
    print(f"Part 1 Puzzle: {result}")
