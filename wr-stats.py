import csv
import re

level_names = ["Warm Up",
               "Flat Track",
               "Twin Peaks",
               "Over and Under",
               "Uphill Battle",
               "Long Haul",
               "Hi Flyer",
               "Tag",
               "Tunnel Terror",
               "The Steppes",
               "Gravity Ride",
               "Islands in the Sky",
               "Hill Legend",
               "Loop-de-Loop",
               "Serpents Tale",
               "New Wave",
               "Labyrinth",
               "Spiral",
               "Turnaround",
               "Upside Down",
               "Hangman",
               "Slalom",
               "Quick Round",
               "Ramp Frenzy",
               "Precarious",
               "Circuitous",
               "Shelf Life",
               "Bounce Back",
               "Headbanger",
               "Pipe",
               "Animal Farm",
               "Steep Corner",
               "Zig-Zag",
               "Bumpy Journey",
               "Labyrinth Pro",
               "Fruit in the Den",
               "Jaws",
               "Curvaceous",
               "Haircut",
               "Double Trouble",
               "Framework",
               "Enduro",
               "He He",
               "Freefall",
               "Sink",
               "Bowling",
               "Enigma",
               "Downhill",
               "What the Heck",
               "Expert System",
               "Tricks Abound",
               "Hang Tight",
               "Hooked",
               "Apple Harvest"]

wr_tables = []

with open("2018-04-19_elma_wrs.csv") as wrtcsv:
    r = csv.reader(wrtcsv, delimiter=',', quotechar='"')

    for row in r:
        t = re.split(':|,', row[3])
        M = 0
        S = 0
        MS = 0
        if len(t) == 3:
            M = int(t[0])
            S = int(t[1])
            MS = int(t[2])
        else:
            S = int(t[0])
            MS = int(t[1])

        wr_tables.append([int(row[0]), int(row[1]), (M, S, MS), row[4]])

time_table = []

with open("stats.txt") as times:
    level_counter = 0
    for line in times:
        data = line.split()

        if data != [] and data[0] == "Level":
            data = times.readline().strip().split(" ")[0].split(":")
            t = (int(data[0]), int(data[1]), int(data[2]))
            time_table.append(t)
            level_counter += 1

        if level_counter == 54:
            break


def compare_times(t1, t2):
    return t1[0] * 60 * 100 + t1[1] * 100 + t1[2] <= t2[0] * 60 * 100 + t2[1] * 100 + t2[2]


def time_difference(t1, t2):
    ms = t1[2]-t2[2]

    if ms < 0:
        s = t1[1]-t2[1]-1
        ms = 100 + ms
    else:
        s = t1[1]-t2[1]

    if s < 0:
        m = t1[0]-t2[0]-1
        s = 60 + s
    else:
        m = t1[0]-t2[0]

    return m, s, ms
    # return(m, s, ms)


def add_times(t1, t2):
    ms = t1[2]+t2[2]

    if ms > 99:
        s = t1[1]+t2[1]+1
        ms = ms-100
    else:
        s = t1[1]+t2[1]

    if s > 60:
        m = t1[0]+t2[0]+1
        s = s-60
    else:
        m = t1[0]+t2[0]

    return m, s, ms


def time_to_string(t):
    return "{min:02d}:{sec:02d},{ms:02d}".format(min=t[0], sec=t[1], ms=t[2])


headers = ["Lev", "Name", "PR", "Table", "Time", "Kuski", "Target", "Diff", "Kuski"]
lev_format = "{:<5}"
lev_name_format = "{:<19}"
time_format = "{:<10}"
diff_format = "{:<11}"
table_format = "{:<7}"
kuski_format = "{:<13}"
header_format = lev_format + lev_name_format + time_format + table_format + time_format + kuski_format + time_format\
                + diff_format +kuski_format + "\n"
row_format = header_format
# "{:<5}{:<19}{:<11}{:<7}{:<12}{:<14}{:<11}{:<11}{:<14}\n"

FIRST = 0
LAST = -1
TABLE = 0
TIME = 2
KUSKI = 3

with open("wrs_beat.txt", 'w') as file:
    file.write(header_format.format(*headers))
    for i in range(54):
        t = time_table[i]

        wrs_beat = list(filter(lambda x: (x[1] == i + 1) and compare_times(t, x[TIME]), wr_tables))
        wrs_not_beat = list(filter(lambda x: (x[1] == i + 1) and not compare_times(t, x[TIME]), wr_tables))

        lev_number = str(i + 1)
        lev_name = level_names[i]
        pr = time_to_string(t)

        if wrs_beat:
            last_table_beat = str(wrs_beat[LAST][TABLE])
            last_time_beat = time_to_string(wrs_beat[LAST][TIME])
            last_kuski_beat = wrs_beat[LAST][KUSKI]
        else:
            last_table_beat = "-"
            last_time_beat = "-"
            last_kuski_beat = "-"

        if wrs_not_beat:
            next_target = time_to_string(wrs_not_beat[FIRST][TIME])
            diff = "+" + time_to_string(time_difference(t, wrs_not_beat[FIRST][TIME]))
            next_kuski = wrs_not_beat[FIRST][KUSKI]
        else:
            next_target = "-"
            diff = "-"
            next_kuski = "-"

        # if wrs_beat != []:
        file.write(row_format.format(
            lev_number,
            lev_name,
            pr,
            last_table_beat,
            last_time_beat,
            last_kuski_beat,
            next_target,
            diff,
            next_kuski))

print("Script is finished running. Data saved in wrs_beat.txt.")
