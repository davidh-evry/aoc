function part1(seeds, maps)
    min_val = typemax(Int)
    for seed in seeds
        val = seed
        for map in maps
            for (range, dest) in map
                if val in range
                    val = val - first(range) + dest
                    break
                end
            end
        end
        min_val = min(min_val, val)
    end
    println(min_val)
end

function part2(seeds, maps)
    seed_ranges = [seeds[i]:seeds[i]+seeds[i+1]-1 for i in 1:2:length(seeds)-1]
    min_val = typemax(Int)
    for seed_range in seed_ranges
        ranges = [seed_range]
        for map in maps
            remapped = [remap_range(range, map) for range in ranges]
            ranges = Iterators.flatten(remapped)
        end
        min_val = min(min_val, first(minimum(ranges)))
    end
    println(min_val)
end

function remap_range(start_range, map)
    ranges = [start_range]
    for (map_range, dest) in map
        range = ranges[end]
        overlap = range ∩ map_range
        if isempty(overlap)
            continue
        end
        mapped_first = first(overlap) - first(map_range) + dest
        mapped_last = last(overlap) - first(map_range) + dest
        ranges[end] = mapped_first:mapped_last
        if first(range) < first(map_range)
            push!(ranges, first(range):first(map_range)-1)
        end
        if last(range) > last(map_range)
            push!(ranges, last(map_range)+1:last(range))
        else
            return ranges
        end
    end
    return ranges
end

function parse_file(file)
    lines = eachline(file)
    seeds = [parse(Int, x) for x in split(iterate(lines)[1])[2:end]]
    iterate(lines)
    maps = [getMap(lines) for _ in 1:7]
    return (seeds, maps)
end

function getMap(lines)
    iterate(lines) # ignore map name
    tuples = Tuple{UnitRange{Int},Int}[]
    while true
        line = iterate(lines, nothing)
        if line === nothing || isempty(line[1])
            break
        end
        s = [parse(Int, x) for x in eachsplit(line[1])]
        push!(tuples, (s[2]:s[2]+s[3]-1, s[1]))
    end
    return sort(tuples; by=x -> x[1])
end

seeds, maps = (nothing, nothing)
open("res/day5.txt") do file
    global seeds, maps = parse_file(file)
end
part1(seeds, maps)
part2(seeds, maps)