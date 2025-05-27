
obj Example begin
    count: int,
    cost: float,
end

fun main() begin
    let o = new Example begin
        count = 10,
        cost = 1.4,
    end

    # let c = o.count
    # set o.count = x + 1
end
