from collections import defaultdict


def solution(target_x_range: range, target_y_range: range):
    tx_range = range(target_x_range.start, target_x_range.stop + 1)
    ty_range = range(target_y_range.start, target_y_range.stop + 1)

    count = 0
    answer = 0
    for svy in range(ty_range.start, -ty_range.start):
        for svx in range(0, tx_range.stop):
            # print("testing", svx, svy)
            if svx * (svx + 1) // 2 < target_x_range.start:
                continue
            
            # print("simulating", vx, vy)
        
            x, y = 0, 0
            vx, vy = svx, svy
            min_x, max_x = 0, tx_range.stop
            min_y, max_y = ty_range.start, 0
            while x <= max_x and y >= min_y:
                x += vx
                y += vy

                vx -= 1 if vx > 0 else -1 if vx > 0 else 0
                vy -= 1

                min_x = min(min_x, x)
                max_y = max(max_y, y)
                
                if x in tx_range and y in ty_range:
                    count += 1
                    answer = max(answer, max_y)
                    # print(svx, svy)
                    break
    print(answer, count)

# visualize((6, 5), range(20, 30), range(-10, -5))
# target area: x=124..174, y=-123..-86
# visualize((16, 30), range(124, 174), range(-123, -86))

def main():
    # solution(range(20, 30), range(-10, -5))
    solution(range(124, 174), range(-123, -86))

if __name__ == "__main__":
    main()