income = float(input("Enter the annual income: "))


tax_relief = 85528.0

if income <= tax_relief:
    tax = (income / 100) * 18 - 556.2   
    if tax <= 0:
        tax = 0   
elif income > tax_relief:
    surplus = income - tax_relief
    tax = (surplus / 100) * 32 + 14839.2


tax = round(tax, 0)
print("The tax is:", tax, "thalers")