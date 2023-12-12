#include <iostream>
#include <fstream>
#include <stack>
#include <unordered_map>
using namespace std;

//#define SOLUCIONPART1
#ifdef SOLUCIONPART1
int main(int argc, char* argv[]){
    int suma = 0;
    // Abrimos el archivo input.txt en modo lectura
    ifstream archivo("input.txt");
    // Leemos el archivo
    while(!archivo.eof()){
        // Leemos linea por linea
        string linea;
        getline(archivo, linea);

        int digit = 0;
        bool dosDigitos = false; // 0 = false, 1 = true
        for ( char c : linea){
            if (isdigit(c)){
                // Convertimos el caracter a entero
                int numero = c - '0';
                if ( digit == 0){
                    digit = numero*10;
                    suma += digit;
                }
                else{
                    dosDigitos = true;
                    digit = numero;
                }
            }
        }
        dosDigitos ? suma += digit : suma += (digit/10);
    }
    cout << "La suma de los numeros es: " << suma << endl;
    
    return 0;
}
#endif // SOLUCIONPART1
#ifndef SOLUCIONPART1
// Función para convertir una palabra a número
static std::unordered_map<std::string, int> palabraToNumero = {
        {"zero", 0},
        {"one", 1},
        {"two", 2},
        {"three", 3},
        {"four", 4},
        {"five", 5},
        {"six", 6},
        {"seven", 7},
        {"eight", 8},
        {"nine", 9}
};

int main(int argc, char* argv[]){
    int suma = 0;
    // Abrimos el archivo input.txt en modo lectura
    ifstream archivo("input.txt");
    // Leemos el archivo
    while(!archivo.eof()){
        // Leemos linea por linea
        string linea;
        getline(archivo, linea);

        for ( auto it : palabraToNumero){
            size_t pos = linea.find(it.first);
            // Una vez que se lea una palabra, se reemplaza por su valor numérico
            // Hay que tener en cuenta que una misma palabra puede contener dos numeros
            // Ejemplo: oneight -> 18
            while ( pos != string::npos){
                linea.replace(pos+1, it.first.length()-2, to_string(it.second));
                pos = linea.find(it.first);
            }
        }
        int digit = 0;
        bool dosDigitos = false; // 0 = false, 1 = true
        for ( char c : linea){
            if (isdigit(c)){
                // Convertimos el caracter a entero
                int numero = c - '0';
                if ( digit == 0){
                    digit = numero*10;
                    suma += digit;
                }
                else{
                    dosDigitos = true;
                    digit = numero;
                }
            }
        }

        dosDigitos ? suma += digit : suma += (digit/10);

    }
    cout << "La suma de los numeros es: " << suma << endl;

    return 0;
}



#endif // SOLUCIONPART1