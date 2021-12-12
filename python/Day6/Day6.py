#!/usr/bin/env python3

def main(fn, days):
    data = [[int(a) for a in open(fn).read().strip().split(',')].count(val) for val in range(0,9)]
    f = lambda data: [data[(index+1)%9] if (index%9) != 6 else data[7]+data[0] for index in range(0,9)]
    for i in range(days):
        data = f(data)
        print(data)
        if i % 100000 == 0:
            # print(f"Day {i}: {sum(data)}")
            print(f"Day {i}")
    return(sum(data))

def main_fast(fn, days):
    data = [[int(a) for a in open(fn).read().strip().split(',')].count(val) for val in range(0,9)]
    # f = lambda data: [data[(index+1)%9] if (index%9) != 6 else data[7]+data[0] for index in range(0,9)]
    print(data)
    for i in range(days):
        #data = f(data)
        next_zero = data[0]
        for index in range(0,9):
            if index == 6:
                data[index] = next_zero+data[index+1]
            elif index == 8:
                data[index] = next_zero
            else:
                data[index] = data[index+1]
        # print(data)
        if i % 100000 == 0:
            # print(f"Day {i}: {sum(data)}")
            print(f"Day {i}")
    return(sum(data))

if __name__ == '__main__':
    fn_sample = 'input_sample.txt'
    fn = 'input.txt'
    result = main_fast(fn,pow(2,23))
    print(f"Result: {result}")
    # result = main(fn,16)
    # print(f"Result: {result}")
    # print(main(fn_sample, 80))
    # print(main(fn_sample, 256))
    # print(main(fn, 80))
    #print(main(fn, 256))
    #for i in range(9999):
    #    print(f"Day {i}: {main(fn, i)}")

