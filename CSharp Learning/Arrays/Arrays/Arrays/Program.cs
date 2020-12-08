using System;

namespace Arrays
{
    class Program
    {
        static void Main(string[] args)
        {/*Definimos um array de int falando new int[], oq isso faz é criar uma nova instancia da classe array do 
         CSharp*/
            var numeros = new int[3] { 1, 2, 3 };
            Console.WriteLine(numeros[1]);
         /*Definimos um array de bool falando new bool[], interessante ressaltar que o valor padrão de um bool é false*/
            var flags = new bool[3];
            flags[0] = true;
            Console.WriteLine(flags[0]);
            Console.WriteLine(flags[1]);
            Console.WriteLine(flags[2]);

         /*Array de strings normal*/   
            var nomes = new string[3] { "Henrique", "Lara", "Vitão" };
            Console.WriteLine("{0},{1},{2}", nomes[0],nomes[1],nomes[2] +" estão jogando aramzin");

            
        }
    }
}
