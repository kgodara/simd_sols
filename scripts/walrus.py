# Rust
'''
let rounded_median: u16 =
    if d.len() % 2 != 0 {
        d[d.len() / 2]
    } else {
        ( d[d.len() / 2] + d[(d.len() / 2) - 1] ) / 2 +
        ( d[d.len() / 2] + d[(d.len() / 2) - 1] ) % 2
    }
;
'''


# Python
#d = [random.randint(0,9999) for _ in range(0,random.randint(10_000,11_000))]
#d.sort()

d = [0 for _ in range(0,random.randint(10_000,11_000))]

#x = (d[len(d)//2] 
#    if len(d) % 2 != 0
#    else
#        (d[len(d)//2] + d[len(d)//2 - 1]) // 2 + 
#        (d[len(d)//2] + d[len(d)//2 - 1]) % 2
#)

#x = (d[len(d)//2] if len(d) % 2 != 0 else (d[len(d)//2] + d[len(d)//2 - 1]) // 2 + (d[len(d)//2] + d[len(d)//2 - 1]) % 2)

#z = (d[y//2] if (y:=len(d)) % 2 != 0 else (d[y//2] + d[y//2 - 1]) // 2 + (d[y//2] + d[y//2 - 1]) % 2)
#z2 = (d[y//2] if (y:=len(d)) % 2 != 0 else (d[w:=y//2] + d[w - 1]) // 2 + (d[w] + d[w - 1]) % 2)
#e=(e:=d[(a:=len(d))//2-1+a%2]+d[a//2])//2+e%2

import random
import timeit
for _ in range(0,4):
    d = [0 for _ in range(0,random.randint(10_000,11_000))]

    print(timeit.timeit(lambda: len(d)%4 + len(d)*4 + len(d), number=10_000_000))
    print(timeit.timeit(lambda: (l:=len(d))%4 + l*4 + l, number=10_000_000))
    print()

    #print(timeit.timeit(lambda: (d[len(d)//2] if len(d) % 2 != 0 else (d[len(d)//2] + d[len(d)//2 - 1]) // 2 + (d[len(d)//2] + d[len(d)//2 - 1]) % 2), number=10_000_000))
    #print(timeit.timeit(lambda: (d[y//2] if (y:=len(d)) % 2 != 0 else (d[y//2] + d[y//2 - 1]) // 2 + (d[y//2] + d[y//2 - 1]) % 2), number=10_000_000))
    #print(timeit.timeit(lambda: (d[y//2] if (y:=len(d)) % 2 != 0 else (d[w:=y//2] + d[w - 1]) // 2 + (d[w] + d[w - 1]) % 2), number=10_000_000))
    #print(timeit.timeit(lambda: (e:=d[(a:=len(d))//2-1+a%2]+d[a//2])//2+e%2, number=10_000_000))


#print(f"x: {x}")
#print(f"z: {z}")
#print(f"z2: {z2}")
#print(f"e: {e}")

#assert e == x