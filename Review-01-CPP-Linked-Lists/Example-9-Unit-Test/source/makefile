MAINPROG=testLinkedList

SOURCES:=$(wildcard *.cpp)
OBJECTS=$(SOURCES:.cpp=.o)
FLAGS=-std=c++17 -fsanitize=leak,address -Wall -Wpedantic -fuse-ld=gold #-DPOOLDEBUG \
	  #-fprofile-arcs -ftest-coverage -g -O0 # This line is for gcovr/profiling

all: $(SOURCES) $(MAINPROG)

$(MAINPROG): $(OBJECTS)
	g++ $(FLAGS) $(OBJECTS) -o $@

.cpp.o:
	g++ $(FLAGS) -c $< -o $@

clean:
	rm *.o $(MAINPROG) #*.gcov *.gcda *.gcno

coverage: $(MAINPROG)
	./$(MAINPROG)
	gcov -r $(MAINPROG)
