#include <iostream>
#include <iomanip>
#include <fstream>
#include <vector>

#include "Polyhedron.h"
#include "PolyhedronFactory.h"

using namespace std;

/**
 * Construct a list of polyhedra from
 * an input stream
 */
void readPolyhedra(istream& inf, vector<Polyhedron*>& polyhedra);

/**
 * Print a collection of polyhedra
 */
void printPolyhedra(ostream& outs, const vector<Polyhedron*>& polyhedra);

/**
 * Copy each polyhedron and apply
 * the scaling factor to each copy
 *
 * @param polyhedra orignal polyhedron collection 
 * @param scaledCopies collection in whcih to store the scled copies
 * @param s scaling factor
 */
void duplicateAndScale(const vector<Polyhedron*>& polyhedra, 
                       vector<Polyhedron*>& scaledCopies,
                       double s);

/**
 * Main function 
 *
 * @param argv[1] polyhedron input file
 * @param argv[2] scaling factor
 */
int main(int argc, char** argv)
{    
    // Validate number of command line arguments
    if (argc < 3) {
        cerr << "Usage " << argv[0] << " inputFile scalingFactor" << "\n";
        return 1;
    }

    // Validate Scaling Factor
    int scalingFactor = atoi(argv[2]);
    if (scalingFactor < 1) {
        cerr << "Scaling Factor must be >= 1" << "\n";
        return 2;
    }

    // Validate Input File
    ifstream polyIn(argv[1]);
    if (!polyIn) {
        cerr << "Could not open " << argv[1] << "\n";
        return 3;
    }

    // Original polyhedra
    vector<Polyhedron*> polyhedra;
    readPolyhedra(polyIn, polyhedra);

    // Scaled copies of all polyhedra
    vector<Polyhedron*> scaledCopies;
    duplicateAndScale(polyhedra, scaledCopies, scalingFactor);

    cout << "Original Polyhedra" << "\n"
         << string(54, '-')      << "\n";
    printPolyhedra(cout, polyhedra);

    cout << "\n"
         << "\n";
    cout << "Scaled Polyhedra (Clones)" << "\n"
         << string(54, '-')             << "\n";
    printPolyhedra(cout, scaledCopies);    

    return 0;
}

/**
 *
 */
void readPolyhedra(istream& inf, vector<Polyhedron*>& polyhedra)
{
    Polyhedron* p = nullptr;

    while (inf >> p) {
        if (p != nullptr) {
            polyhedra.push_back(p);
        }        
    }
}

/**
 *
 */
void printPolyhedra(ostream& outs, const vector<Polyhedron*>& polyhedra)
{
    for (const Polyhedron* p : polyhedra) {
        cout << (*p) << "\n";
    }
}

/**
 *
 */
void duplicateAndScale(const vector<Polyhedron*>& polyhedra, 
                       vector<Polyhedron*>& scaledCopies,
                       double s)
{
    for (Polyhedron* original : polyhedra) {
        Polyhedron* copy = original->clone();
        copy->scale(s);

        scaledCopies.push_back(copy);
    }
}