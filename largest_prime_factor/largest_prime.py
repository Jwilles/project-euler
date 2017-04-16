import math 

num = 600851475143
i_max = 0

def is_prime(a):
  if a == 2:
    return True
  if a % 2 == 0 or a <= 1:
    return False

  sqr = int(math.sqrt(a)) + 1
  
  for div in range(3, sqr, 2):
    if a % div == 0:
      return False
  return True


# Naive approach
for i in xrange(3, num, 2):
  if num%i == 0 and is_prime(i) and i > i_max:
    i_max = i
print i_max  

#Implement with factor division
