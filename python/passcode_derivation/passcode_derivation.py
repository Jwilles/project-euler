
f = open('keylog.txt', 'r')
login_attempts = sorted([line for line in f.read().split('\n') if line])

#for attempt in login_attempts:
  
print login_attempts 
