from ClassFile import *

# Reads file with name filename and outputs dictionary
def parse(filename):
    with open(filename, mode="r") as ifs:
        lines = ifs.readlines()
        dictionary = {"filename": filename[6:]}

        # Line 1 (VERCX)
        [R, C, F, N, B, T] = map(int, lines[0].split())
        dictionary['R'] = R
        dictionary['C'] = C
        dictionary['F'] = F
        dictionary['N'] = N
        dictionary['B'] = B
        dictionary['T'] = T

        line = 1
        rides = []
        for i in range(N):
            ride_info = list(map(int, lines[line + i].split()))
            rides.append(Ride( (ride_info[0], ride_info[1]), (ride_info[2], ride_info[3]), ride_info[4], ride_info[5], i ))

        cars = []
        for j in range(F):
            cars.append( Car((0,0), j) )

        # Return problem data
        return (cars, rides, B, T)

# Output the solution
def output(filename, solution):
    # Cut off 'input/'
    with open('output/{}.txt'.format(filename[6:-3]), mode="w") as ofs:
        print("Outputting solution for {}...".format(filename[6:]))
        for car in solution:
            ofs.write("{}\n".format(str(car)))