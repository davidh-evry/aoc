function count_wins(t, dist)
    i = findfirst(v -> v * (t - v) > dist, 1:t-1)
    return t - 2 * i + 1
end

lines = readlines("res/day6.txt")
times, dists = (parse.(Int, split(l[10:end])) for l in lines)
println(prod(count_wins.(times, dists)))

t2, d2 = (parse(Int, join(string.(x))) for x in (times, dists))
println(count_wins(t2, d2))
