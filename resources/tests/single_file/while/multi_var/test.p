fun main() -> int begin
    let i = 0
    let x = 0
    let y = 0
    while x + y < 10 do
        if i % 2 == 0 then
            set x = x + 1
        else
            set y = y + 1
        end

        set i = i + 1
    end
    return x + y
end
