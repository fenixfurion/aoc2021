%% oh god
% load data
%A = importdata('input.txt');
fid = fopen('input.txt');
tline = fgetl(fid);
data = [];
while ischar(tline)
    disp(tline)
    line = [];
    for i=1:length(tline)
       line = [line str2num(tline(i))];
    end
    data = [data; line];
    tline = fgetl(fid);
end
x = 0:99;
y = 0:99;
surf(x,y,data)
zlim([-5 15]);