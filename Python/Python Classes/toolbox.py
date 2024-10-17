import json

class User:
    def create_user(self,name:str,age:str):
        
        with open("users.json","r") as file:
            dicionario = json.load(file)
            
        for i in dicionario["Users"]:
            ID = int(i) + 1
            
            
        dicionario[str(ID)]= {"Name":name,
                        "Age":age}
            
        with open("users.json","w") as file:
        
            json.dump(dicionario,file,indent=4)
    