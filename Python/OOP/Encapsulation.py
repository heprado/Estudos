class Pai():
    
    def __init__(self):
        ##Fazendo um parametro privado em python, utilizamos dois _ para isso
        self.__cor = "Azul"
        self.__tamanho = "3CM"
        self.public = "Vish ta publico."
        
class Filha(Pai):
    
    def  __init__(self):
        ##Herdando o metodo construtor da classe Pai
        super().__init__()
        ##Printando o parametro publico da classe construtor pq ele está publico, não consigo printar a cor.
        print(self.public)
        
               
filha_obj = Filha()