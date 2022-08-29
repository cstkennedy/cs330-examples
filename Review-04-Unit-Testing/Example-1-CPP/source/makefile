MAINPROG=playTicTacToe

SOURCES:=$(wildcard *.cpp)
OBJECTS=$(SOURCES:.cpp=.o)
FLAGS=-std=c++11 -fsanitize=address -Wall -fuse-ld=gold

all: $(SOURCES) $(MAINPROG) tests

$(MAINPROG): Player.o Board.o Game.o playTicTacToe.o Strategy.o
	g++ $(FLAGS) Strategy.o Player.o Board.o Game.o playTicTacToe.o -o $@

tests: Player.o TestPlayer.o Board.o TestBoard.o Strategy.o
	g++ $(FLAGS) Strategy.o Player.o TestPlayer.o -o testPlayer
	g++ $(FLAGS) Board.o TestBoard.o -o testBoard

.cpp.o:
	g++ $(FLAGS) -c $< -o $@

clean:
	rm *.o $(MAINPROG)
