class Ride:
    def __init__(self, startpos, endpos, start_time, end_time, ride_id):
        self.startpos = startpos
        self.endpos = endpos
        self.start_time = start_time
        self.end_time = end_time
        self.distance = abs(startpos[0] - endpos[0]) + abs(startpos[1] - endpos[1])
        self.score = self.distance
        self.ride_id = ride_id

    def __str__(self):
        return "startpos: {}, endpos: {}, start: {}, end:{}".format(self.startpos, self.endpos, self.start_time, self.end_time)

def get_distance(startpos, endpos):
    return abs(startpos[0] - endpos[0]) + abs(startpos[1] - endpos[1])

class Car:
    def __init__(self, pos, car_id):
        self.pos = pos
        self.ride = None
        self.car_id = car_id
        self.memory = []
        self.busy_time = 0

    def move(self):
        if self.ride:
            self.busy_time = max(0, self.busy_time - 1)
            if self.busy_time == 0:
                self.memory.append(self.ride.ride_id)
                self.pos = self.ride.endpos
                self.ride = None

    def set_ride(self, ride, current_time):
        self.ride = ride
        self.busy_time = max(get_distance(self.pos, self.ride.startpos), self.ride.start_time - current_time) + self.ride.distance

    def __str__(self):
        return "{} ".format(len(self.memory)) + " ".join([str(i) for i in self.memory])

# class Map:
#
#     def __init__(self,cars, rides):
#         self.cars = cars
#         self.rides = rides
#         self.current_time = 0
#
#     def simulate(self):
#         self.current_time += 1
#         for car in self.cars:
#             car.move()
#
#
#     def __str__(self):
#         return "\n".join([str(car) for car in self.cars])
