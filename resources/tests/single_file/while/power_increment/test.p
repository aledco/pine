fun main() begin
    let x = 0
    while x < 1000 do
        set x = x ** x
    end
    return x
end
