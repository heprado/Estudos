using System;

namespace Operators
{
    class Program
    {
        static void Main(string[] args)
        {  

            //no caso abaixo o B ficará como 1 pois ele adiciona o valor//
            //de A e só depois adiciona A+A, então A ficara 2 e B ficara 1//
            
            int a = 1;
            int b = a++;
            
            Console.WriteLine("a={0} b={1}",a,b);
            
            //no caso abaixo os dois ficarão como 2 pois C é adicionado a D//
            //somente depois que C é adicionado a C(C+C=2)
             int c = 1;
             int d = ++c;
            
            Console.WriteLine("c={0} d={1}",c,d);
            
            int h = 2;
            h *= 2;
            Console.WriteLine(h);

            int z = 2;
            int x = 2;
            //Se Z é menor que X , não eles são iguais então False
            Console.WriteLine(z < x);
            //Se Z é menor que X , não eles são iguais então False mas como termos o ! ele inverte para True
            Console.WriteLine(!(z < x));
            //Se Z é igual a X e Z e Z é maior que X, sim Z é igual a X mas z não é maior que X então false
            Console.WriteLine(z == x && z > x);
            //Se Z é igual a X e Z OU Z é maior que X, sim Z é igual a X mas z não é maior que X então True
            Console.WriteLine(z == x || z > x);
            
        }
    }
}
