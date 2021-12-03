with open('input.txt') as fd:
    data = [int(line) for line in fd.readlines()]
    previous_val = 99999999
    increases = 0
    increases_window = 0
    for line in data:
        if line > previous_val:
            increases+=1
        previous_val = line
    # sliding window
    previous_val = 9999999
    for index in range(len(data)-2):
        current_val = data[index]+data[index+1]+data[index+2]
        if current_val > previous_val:
            increases_window +=1
        previous_val = current_val
print(increases)
print(increases_window)