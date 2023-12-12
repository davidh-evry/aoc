#RDLU
directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
roatations = Dict(
    '|' => d -> d,
    '-' => d -> d,
    'L' => d -> d == 2 ? 1 : 4,
    'J' => d -> d == 1 ? 4 : 3,
    '7' => d -> d == 1 ? 2 : 3,
    'F' => d -> d == 3 ? 2 : 1,
)

function part1(lines)
    start_pos = nothing
    for (y, line) in enumerate(lines), (x, c) in enumerate(line)
        if c == 'S'
            start_pos = (x, y)
            break
        end
    end

    direction = 0
    pos = start_pos
    for (i, dir) in enumerate(directions)
        (x, y) = pos .+ dir
        if y < 1 || y > length(lines) || x < 1 || x > length(lines[y]) ||
           !haskey(roatations, lines[y][x]) ||
           # Ugly, but works
           !any(d -> (x, y) .+ directions[roatations[lines[y][x]](d)] == start_pos, eachindex(directions))
            continue
        end
        direction = i
        pos = (x, y)
        break
    end

    loop = Tuple{Int,Int}[start_pos]
    while pos != start_pos
        push!(loop, pos)
        direction = roatations[lines[pos[2]][pos[1]]](direction)
        pos = pos .+ directions[direction]
    end
    println(length(loop) ÷ 2)
    return loop
end

function part2(lines, loop)
    right_map = Dict(1 => (0, 1), 2 => (-1, 0), 3 => (0, -1), 4 => (1, 0))
    inside_loop = Set{Tuple{Int,Int}}()
    start_i = argmin(loop) # Leftmost, topmost corner
    dir = 1 # Move clockwise and check inside/right side of loop
    pos = loop[start_i] .+ directions[dir]
    while pos != loop[start_i]
        union!(inside_loop, visit(pos .+ right_map[dir], loop, inside_loop))
        if lines[pos[2]][pos[1]] == 'S'
            dir = findfirst(d -> pos .+ d in (loop[2], loop[end]) && pos .+ d != pos .- directions[dir], directions)
        else
            dir = roatations[lines[pos[2]][pos[1]]](dir)
        end
        union!(inside_loop, visit(pos .+ right_map[dir], loop, inside_loop))
        pos = pos .+ directions[dir]
    end
    println(length(inside_loop))
end

function visit(pos, loop, inside_loop)
    if pos in inside_loop || pos in loop
        return Set()
    end
    nodes = Set{Tuple{Int,Int}}()
    push!(nodes, pos)
    visited = Set{Tuple{Int,Int}}()
    while !isempty(nodes)
        node = pop!(nodes)
        push!(visited, node)
        for next in (node .+ d for d in directions)
            if next ∉ visited && next ∉ nodes && next ∉ loop
                push!(nodes, next)
            end
        end
    end
    return visited
end

lines = readlines("res/day10.txt")
loop = part1(lines)
part2(lines, loop)
