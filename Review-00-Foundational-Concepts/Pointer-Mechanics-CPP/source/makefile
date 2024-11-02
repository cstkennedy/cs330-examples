MAINPROG=pointerReview

SOURCES:=$(wildcard *.cpp)
OBJECTS=$(SOURCES:.cpp=.o)
FLAGS=-std=c++17 -Wall -Wextra -Wpedantic -Weffc++ -fuse-ld=gold

all: $(SOURCES) $(MAINPROG)

$(MAINPROG): $(OBJECTS)
	g++ $(FLAGS) $(OBJECTS) -o $@

.cpp.o:
	g++ $(FLAGS) -c $< -o $@

clean:
	rm *.o $(MAINPROG)
