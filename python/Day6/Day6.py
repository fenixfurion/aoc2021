#!/usr/bin/env python3

def main(fn, days):
    data = [[int(a) for a in open(fn).read().strip().split(',')].count(val) for val in range(0,9)]
    f = lambda data: [data[(index+1)%9] if (index%9) != 6 else data[7]+data[0] for index in range(0,9)]
    for i in range(days):
        data = f(data)
    return(sum(data))

if __name__ == '__main__':
    fn_sample = 'input_sample.txt'
    fn = 'input.txt'
    print(main(fn_sample, 80))
    print(main(fn_sample, 256))
    print(main(fn, 80))
    print(main(fn, 256))

