import xml.etree.ElementTree as ET
##Transformando o XML de um arquivo para um objeto
xml = ET.parse("xml_deserialization.xml")
##Pegando o root desse objeto
root = xml.getroot()
##Adicionando um subelemento que é o DC3
dc3 = ET.SubElement(root,"dc3")

##Adicionando um subelemento no DC3
ipaddr = ET.SubElement(dc3,"ipaddr")
##O Texto dentro desse subelemento
ipaddr.text = "192.168.0.3"
netmask = ET.SubElement(dc3,"netmask")
netmask.text = "255.255.255.0"
defaultroute = ET.SubElement(dc3,"defaultroute")
defaultroute.text = "192.168.0.1"

##Escrevendo para um arquivo.    
xml.write("xml_serialization.xml")
    



