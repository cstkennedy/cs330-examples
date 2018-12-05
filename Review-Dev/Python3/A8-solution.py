#! /usr/bin/env python

# Thomas Kennedy
# March 2014
# Assignment 8 Solution

import os
import sys
import random
import math

# Constants
DEBUG = True # Set to true to generate a log file with debug output

#
# Print a centered and styled heading
#
def printHeading(title, width=40, outs=sys.stdout):
    print("=" * width, file=outs)
    print(title.center(width), file=outs)
    print("=" * width, file=outs)

#
# Create the deck of cards - initialize the array
#
def createCardDeck( deck ):
    for suit in ["Hearts", "Diamonds", "Clubs", "Spades"]:
        #create the cards in each suit
        for value in ( ["A"] + list(range( 2,11)) + ["J", "Q", "K"] ):
            deck.append( {"suit":suit, "value":value} )

#
# Shuffle the deck of cards
#
def shuffleCardDeck( deck ):
    #i is the index of the card to swap
    #swap_i is the index of the card with which it will be swapped

    for i in range(0, len(deck) ):
        swap_pos = random.randint(0, len( deck )-1 )

        #If the 2 positions are the same do not swap the card
        if i != swap_pos:
            temp = deck[i]
            deck[i] = deck[swap_pos]
            deck[swap_pos] = temp
#
# Print One Card
#
def printCard( outs, card ):
    print("{:>3} of {:<8}".format( card["value"], card["suit"] ), file=outs)

#Main Function
def main():
    #Variable Declarations
    deck        = []  #Deck of Playing Cards - inmplemented as a list of dictionaries
    num_players = 4   #Number of hands to be dealt

    if DEBUG:
        out_file = open( "A8-debug-output.txt", "w")

    #Seed the random number generator
    #random.seed(42)

    createCardDeck( deck )

    # #print the deck in order
    if DEBUG:
        printHeading( "Initial Deck", 40, out_file )
        for card in deck:
            printCard( out_file, card )

    #Shuffle the deck
    shuffleCardDeck( deck )

    # #print the shuffled deck
    if DEBUG:
        printHeading( "Shuffled Deck", 40, out_file )
        for card in deck:
            printCard( out_file, card )

    #Deal 7 cards to each player
    for i in range( 0,num_players ):
        printHeading( "Player {}'s Hand".format(( i+1 )) )

        for card in deck[ (i*7): ( (i+1)*7 )]:
            printCard( sys.stdout, card )

        #Blank Line
        print()

    #print the remaining cards
    printHeading( "{} Cards Remaining".format( len(deck[ (num_players*7): ]) ) )
    for card in deck[ (num_players*7): ]:
        printCard( sys.stdout, card )

    if DEBUG:
        out_file.close()

if __name__ == "__main__":
     main()
