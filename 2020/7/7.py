def parse_line(line):
    line = line.strip()
    from_color, to_color = line.split(" bags contain ")
    if "no other bags" in to_color:
        return from_color, {}

    to_color_text = to_color.split(", ")
    to_colors = {}
    for to_color in to_color_text:
        num, brightness, color, _ = to_color.split(" ")
        to_colors[f"{brightness} {color}"] = int(num)

    return from_color, to_colors


with open("7.txt") as f:
    graph = dict(parse_line(line) for line in f.readlines())

    def dfs(node):
        return sum(
            (1 + dfs(color)) * count
            for color, count
            in graph[node].items()
        )

    print(dfs("shiny gold"))
