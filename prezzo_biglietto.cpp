/*
Un impianto di risalita ha tariffa base di 7€; in estate (e) viene applicata
una maggiorazione del 15%, in autunno (a) uno sconto del 50%; in inverno (i)
viene raddoppiata, in primavera (p) il costo rimane pari alla tariffa base.
Data in ingresso la stagione, calcolare e visualizzare il costo dell'impianto
*/
#include <iostream>
using namespace std;

int main() {
    float prezzo = 7;
    int stagione;
    cout << "1. Primavera; 2. Estate; 3. Autunno; 4. Inverno\n";
    cout << "Numero Stagione: ";
    cin >> stagione;
    switch(stagione) {
        case 1:
            cout << "In primavera il prezzo è rimane la tariffa base.\n";
            break;
        case 2:
            cout << "In estate c'è un incremento del 15%.\n";
            prezzo = prezzo * 115.0 / 100.0;
            break;
        case 3:
            cout << "In autunno c'è uno sconto del 50%.\n";
            prezzo /= 2.0;
            break;
        case 4:
            cout << "In inverno il prezzo raddoppia.\n";
            prezzo *= 2.0;
            break;
        default:
            cout << "Nessuna stagione corrisponde al numero inserito.\n";
            break;
    }
}
