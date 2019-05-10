def _map function array index
    eq index 0
    if
        at array index
        call function
        > tmp
        arr tmp
        return
    endif
    sub index 1
    _map function array
    > res
    at array index
    call function
    > tmp
    arr tmp
    > tmp2
    concat res tmp2
    return
end

def map function array
    len array
    > v
    sub v 1
    _map function array
    return
end

def test v
    add v 10
    return
end

arr 1 2 3
print
map test
print