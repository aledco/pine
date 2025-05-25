import util
import sub

fun main() -> int begin
    util::func()
    sub::func()
    sub::util::func()
    return 0
end
