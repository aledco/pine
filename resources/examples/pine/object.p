
obj Example begin
    count: int,
    cost: float,
end

fun main() begin
    let o = new Example begin
        count = 0,
        cost = 0.0,
    end

    # let c = o.count
    # set o.count = x + 1
end
