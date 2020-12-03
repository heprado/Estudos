import yaml

dicionario = {"Roteadores":{
    "DC":{
        "IPADDR":"192.168.0.1",
        "NETMASK":"255.255.255.0",
        "DEFAULTROUTE":"10.0.0.1"},
    "OFFICE1":{"IPADDR":"192.168.0.2",
               "NETMASK":"255.255.255.0",
               "DEFAULTROUTE":"192.168.0.1"}
    
    }
              }
##Pegamos o dicionario e serializamos em um arquivo yaml.
with open("yaml_serialization.yml", "w") as file:
    yaml.dump(dicionario,file,default_flow_style=False)
    
    