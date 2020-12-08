using System;

namespace Classes 
{
/*Declaramos a classe carro*/
public class Carro
{/*Criamos um field para ele somente que dessa vez ele não é static, então teremos que criar instancias de objetos fora da classe.*/
    public string modelo;
    /*Criamos um metodo que pode ser acessado em outra classe por conta do public e não definimos ele como static, então podemos utilizar
     outras instacias de objetos dessa classe para acessar esse metodo.*/
    public void QualModelo()
    {
        Console.WriteLine("O Modelo do carro é: " + modelo);
    }

}
}
