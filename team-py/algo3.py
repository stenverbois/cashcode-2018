def get_distance(startpos, endpos):
    return abs(startpos[0] - endpos[0]) + abs(startpos[1] - endpos[1])

def sort_func(t, B):

    def func(x):
        if x.start_time - t < 0:
            return (x.distance + x.start_time - t)

        # Still time left to do the task
        return x.distance - B + x.start_time - t

    return func

def ret_inf_if_neg(t, car_pos):
    def f(x):
        if (x.start_time - get_distance(car_pos, x.startpos) - t < 0):
            return x.end_time - x.distance - t

        return x.start_time - get_distance(car_pos, x.startpos) - t
    return f

def exec_alg(data):
    cars = data[0]
    # Sort on start time
    rides = data[1]#sorted(data[1], key=lambda x: (x.end_time - x.distance - x.start_time))

    cars_occupied = []
    B = data[2]
    T = data[3]

    for t in range(1,T+1):

        while len(cars) > 0:
            car = cars.pop(0)
            rides = sorted(rides, key=ret_inf_if_neg(t, car.pos))
            if len(rides) == 0:
                break

            #rides = sorted(rides, key=sort_func(t, B))
            ride = rides.pop(0)

            # How do you choose the ride
            #cars = sorted(cars, key= lambda x: get_distance(x.pos, ride.startpos) - ride.start_time - t )
            #car = cars.pop(0)

            car.set_ride(ride, t)
            cars_occupied.append(car)

        new_cars_occupied = []
        for car in cars_occupied:
            car.move()
            if car.busy_time == 0:
                cars.append(car)
            else:
                new_cars_occupied.append(car)

        cars_occupied = new_cars_occupied


    return cars + cars_occupied

