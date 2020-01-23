MAINPROG=updateRoom

SOURCES:=$(wildcard *.cpp)
OBJECTS=$(SOURCES:.cpp=.o)
FLAGS=-std=c++11 -fsanitize=leak,address -Wall -Wpedantic -fuse-ld=gold

all: $(SOURCES) $(MAINPROG)

$(MAINPROG): $(OBJECTS)
	g++ $(FLAGS) $(OBJECTS) -o $@

.cpp.o:
	g++ $(FLAGS) -c $< -o $@

clean:
	rm *.o $(MAINPROG)
