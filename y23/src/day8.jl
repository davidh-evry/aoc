struct Cycle
    start::Int
    exit::Int
    cycle_size::Int
end

function part1(map, instructions)
    node = "AAA"
    index = 1
    count = 0
    while node != "ZZZ"
        instruction = instructions[index]
        node = map[node][instruction]
        index = index % length(instructions) + 1
        count += 1
    end
    println(count)
end

function part2(node_map, instructions)
    cycles = find_cycles(node_map, instructions)
    lcm_goes_brr_lol = reduce(lcm, map(c -> c.cycle_size, cycles))
    println(lcm_goes_brr_lol)
end

function find_cycles(map, instructions)
    nodes = collect(filter(n -> n[3] == 'A', keys(map)))
    visited = [Dict{Tuple{Int,String},Int}() for _ in 1:length(nodes)]
    index = 1
    count = 0
    cycles = fill(Cycle(0, 0, 0), length(nodes))
    exits = zeros(Int, length(nodes))
    while any(c -> c.start == 0, cycles)
        count += 1
        instruction = instructions[index]
        for (i, node) in enumerate(nodes)
            if cycles[i].start != 0
                continue
            end
            if haskey(visited[i], (index, node))
                start = visited[i][(index, node)]
                cycles[i] = Cycle(start, exits[i] - start, count - start)
            else
                if node[3] == 'Z'
                    exits[i] = count
                end
                nodes[i] = map[node][instruction]
                visited[i][(index, node)] = count
            end
        end
        index = index % length(instructions) + 1
    end
    return cycles
end

function try_to_actually_solve_the_general_case()
    println("cycles: $cycles")
    dists_from_exit = [find_distance_from_exit(c, 0) for c in cycles]
    i = argmax(dists_from_exit)
    round_count = dists_from_exit[i]
    found = zeros(Bool, length(cycles))
    jump_size = 1
    while any(d -> d != 0, dists_from_exit)
        dists_from_exit = [found[i] ? 0 : find_distance_from_exit(c, round_count) for (i, c) in enumerate(cycles)]
        found_new = false
        for i in eachindex(cycles)
            if !found[i] && dists_from_exit[i] == 0
                found[i] = true
                found_new = true
            end
        end
        if found_new
            found_cycles = [c.cycle_size for (i, c) in enumerate(cycles) if dists_from_exit[i] == 0]
            jump_size = reduce(lcm, found_cycles)
        end
        round_count += jump_size
        println("round_count: $round_count, $dists_from_exit")
    end
    println(round_count)
end

function find_distance_from_exit(self::Cycle, count::Int)
    index_in_cycle = count - self.start % self.cycle_size
    dist_from_exit = self.exit - index_in_cycle
    while (dist_from_exit < 0)
        dist_from_exit += self.cycle_size
    end
    return dist_from_exit
end

function parse_input(file)
    lines = eachline(file)
    instructions = [c == 'R' ? 2 : 1 for c in iterate(lines)[1]]
    iterate(lines)
    map = Dict()
    for line in lines
        name, rest = eachsplit(line, " = ")
        left, right = eachsplit(rest[2:end-1], ", ")
        map[name] = (left, right)
    end
    return instructions, map
end

open("res/day8.txt") do file
    instructions, map = parse_input(file)
    # part1(map, instructions)
    part2(map, instructions)
end
