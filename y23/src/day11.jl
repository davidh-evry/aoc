function find_dists(positions, empty_rows, empty_cols, expansion)
    total_dist = 0
    for i in 1:length(positions)-1, j in i+1:length(positions)
        (x1, y1) = positions[i]
        (x2, y2) = positions[j]
        rx = min(x1, x2):max(x1, x2)
        ry = min(y1, y2):max(y1, y2)
        dx = length(rx) - 1 + count(c -> c in rx, empty_cols) * (expansion - 1)
        dy = length(ry) - 1 + count(r -> r in ry, empty_rows) * (expansion - 1)
        total_dist += dx + dy
    end
    println(total_dist)
end

lines = readlines("res/day11.txt")
positions = Tuple{Int,Int}[]
empty_cols = trues(length(lines[1]))
empty_rows = Int[]
for (y, line) in enumerate(lines)
    empty_row = true
    for (x, c) in enumerate(line)
        if c == '#'
            push!(positions, (x, y))
            empty_row = false
            empty_cols[x] = false
        end
    end
    if empty_row
        push!(empty_rows, y)
    end
end
empty_cols = findall(empty_cols)
find_dists(positions, empty_rows, empty_cols, 2)
find_dists(positions, empty_rows, empty_cols, 1000000)