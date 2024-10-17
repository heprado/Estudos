#include <stdio.h>

/*Temperatura em cada escala em que a agua congela*/
#define BASE_CELSIUS 0.0
#define BASE_KELVIN 273.15
#define BASE_FAHRENHEIT 32.0

/*Temperatura em cada escala em que a agua ferve*/
#define MAXIMO_CELSIUS 100.0 
#define MAXIMO_KELVIN 373.0 
#define MAXIMO_FAHRENHEIT 212.0



#define DEGRAU 10

/*Celsius para Kelvin*/
void celsius_kelvin() {

    double celsius,kelvin;
    printf("Celsius\tKelvin\n");
    for (celsius=BASE_CELSIUS; celsius <= MAXIMO_CELSIUS;celsius= celsius + DEGRAU) {

        kelvin = celsius + BASE_KELVIN ;

        printf("%.2f\t%.2f\n",celsius,kelvin);
    }
}

void celsius_fahrenheit() {

    double celsius,fahrenheit;

    printf("Celsius\tFahrenheit\n");

    for (celsius=BASE_CELSIUS; celsius <= MAXIMO_CELSIUS;celsius= celsius + DEGRAU) {

        fahrenheit = (1.8 * celsius) + BASE_FAHRENHEIT;

        printf("%.2f\t%.2f\n",celsius,fahrenheit);
    }
    

}

/*Kelvin para Celsius*/
void kelvin_celsius() {

    double celsius,kelvin;

    printf("Kelvin\tCelsius\n");
    for (kelvin=BASE_KELVIN; kelvin <= MAXIMO_KELVIN;kelvin= kelvin + DEGRAU) {

        celsius = kelvin - BASE_KELVIN ;

        printf("%.2f\t%.2f\n",kelvin,celsius);
    }
}


void kelvin_fahrenheit() {

    double fahrenheit,kelvin;

    printf("\nKelvin\tFahrenheit\n");
    for (kelvin=BASE_KELVIN; kelvin <= MAXIMO_KELVIN;kelvin= kelvin + DEGRAU) {

        fahrenheit = (kelvin - BASE_KELVIN) * 1.8 + BASE_FAHRENHEIT ;

        printf("%.2f\t%.2f\n",kelvin,fahrenheit);
    }
}


void fahrenheit_celsius() {

    double celsius,fahrenheit;

    printf("Fahrenheit\tCelsius\n");

    for (fahrenheit=BASE_FAHRENHEIT; fahrenheit <= MAXIMO_FAHRENHEIT;fahrenheit= fahrenheit + DEGRAU) {

        celsius = (fahrenheit - BASE_FAHRENHEIT) / 1.8 ;

        printf("%.2f\t%.2f\n",fahrenheit,celsius);
    }
}

void fahrenheit_kelvin() {

    double fahrenheit,kelvin;

    printf("Fahrenheit\tKelvin\n");

    for (fahrenheit=BASE_FAHRENHEIT; fahrenheit <= MAXIMO_FAHRENHEIT;fahrenheit= fahrenheit + DEGRAU) {

        kelvin = (fahrenheit - BASE_FAHRENHEIT) * 5/9 + BASE_KELVIN ;

        printf("%.2f\t%.2f\n",fahrenheit,kelvin);
    }
}

int main() {


    celsius_kelvin();
    printf("\v");
    celsius_fahrenheit();
    printf("\v");

    kelvin_celsius();
    printf("\v");
    kelvin_fahrenheit();
    printf("\v");

    fahrenheit_celsius();
    printf("\v");
    fahrenheit_kelvin();
    printf("\v");

    
    return 0;
}