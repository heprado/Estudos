using System;

namespace Classes
{
    public class Person
    {/*Criarmos um field que são os atributos da classe, no nosso caso utilizamos
         a classe Person e demos um nome para ela que é Henrique.*/
        /*o public diz que essa string é publica e pode ser utilizada em outros classes ou namespaces do programa, já o static
        diz que não precisamos criar um objeto novo toda vez que formos acessar esse field.*/
        public static string name = "Henrique";


        /*Aqui criarmos um metodo, esses metodos são funcões que a classe pode fazer como por exemplo,
         se introduzir Introduce().*/
        /*o public diz que esse metodo pode ser acessado em todo o programa, isso é utilizado para segurança já o static quer dizer
         que não precisamos criar um objeto novo da classe toda vez que formos executar.*/
        public static void Introduce()
        {
            Console.WriteLine("Meu nome é " + name);
        }
    }
}