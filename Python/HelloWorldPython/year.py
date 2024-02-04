year = int(input("Enter a year: "))

if year <= 1582:
    print("Not within the Gregorian Calendar period")
elif year%4 != 0:
    print("It's a common year")
elif year%100 !=0:
    print("It's a leap year")
elif year%400 != 0:
    print("It's a common year")
else:
    print("It's a leap year")