#include <iostream>
#include <fstream>
#include <cstdlib>

#include "Inventory.h"
#include "ItemStack.h"

#include "ItemFactory.h"

using namespace std;

/**
 * Read an input stream and generate an Inventory
 *
 * @param size desired number of Inventory slots
 */
Inventory createInventory( std::istream &ins, int size );

/**
 * Print the final Inventory summary
 * 
 * @param inv Inventory to print
 */
void printInventorySummary( std::ostream &outs, const Inventory &inv );

/**
 * Assignment 1: Item Storage
 *
 * @param argv[1] items filename
 * @param argv[2] inventories filename
 */
int main( int argc, char** argv ){      
    ifstream infile;  

    int inv_size = 10; // Inventory Size

    // Check Command Line Arguments
    if( argc < 2 ){
        cerr << "Usage: " << argv[0] << " item_file " << "\n";
        return 1;
    }

    // Open list_file
    infile.open( argv[1] );
    if( !infile ){
        cerr << "Error: " << argv[1] << "could not be opened" << "\n";
        return 2;
    }

    // If an inventory size was specified, parse it.
    if( argc == 3 ){
        inv_size = atoi( argv[2] );
    }

    // Default to 10 if inv_size is invalid--i.e., <= 0
    if( inv_size < 1 ){
        inv_size = 10;
    }

    // Read the Items file and create an Inventory
    Inventory inv = createInventory( infile, inv_size );
    infile.close();

    // Print the Inventory
    printInventorySummary( cout, inv );

    return 0;
}

/**
 *
 */
Inventory createInventory( std::istream &ins, int size ){
    Inventory inventory( size );

    cout << "Processing Log:" << "\n";;

    while( ins ){
        Item *item = nullptr;
        ins >> item;

        if( item != nullptr ){            
            bool success = inventory.addItems( ItemStack( item ) );

            cout << " (" << ( success ? "S" : "D" ) << ") "
                 << item->getName() 
                 << "\n";           
        }
    }
    cout << "\n";

    return inventory;
}

/**
 *
 */
void printInventorySummary( std::ostream &outs, const Inventory &inv ){
    cout << "Player Storage Summary:" << "\n"
         << inv;
}