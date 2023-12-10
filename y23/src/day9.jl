function calculate_num(nums)
    l = length(nums) + 1
    while any(i -> nums[i] != 0, 1:l)
        l -= 1
        for i in 1:l-1
            nums[i] = nums[i+1] - nums[i]
        end
    end
    return sum(nums[l:end])
end

lines = readlines("res/day9.txt")
nums_it = (parse.(Int, eachsplit(line)) for line in lines)
sum_1 = sum(map(nums -> calculate_num(copy(nums)), nums_it))
sum_2 = sum(map(nums -> calculate_num(reverse(nums)), nums_it))
println(sum_1)
println(sum_2)
