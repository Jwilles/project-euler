
f = open('num.txt', 'r')
num_string = f.read().replace('\n', '')

max_product = 1
product_list = []

for index, num in enumerate(num_string):
 product = 1
 if index < (len(num_string)-13):
   for i in range(index, index+13):
     product = product * int(num_string[i]) 
   if product > max_product:
     max_product = product 
 else:
   break
print 'Largest Product: ', max_product




