def ist_palindrom(x: int) -> bool:
    if x < 0:
        return False
    
    str_x = str(x)
    return str_x == str_x[::-1]

print(ist_palindrom(121))
print(ist_palindrom(-121))
print(ist_palindrom(10))

