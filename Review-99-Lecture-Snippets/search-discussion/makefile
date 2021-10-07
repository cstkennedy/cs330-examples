ifneq (,$(findstring MinGW,$(PATH)))
DISTR=MinGW
LFLAGS=
else
DISTR=Unix
EXE=
LFLAGS=
endif

MAINPROG=search_demo

SOURCES:=$(wildcard *.cpp)
OBJECTS=$(SOURCES:.cpp=.o)
FLAGS=-std=c++17 -fsanitize=leak,address -Wall -Wpedantic -fuse-ld=gold -DPOOLDEBUG

all: $(SOURCES) $(MAINPROG)

$(MAINPROG): $(OBJECTS)
	g++ $(FLAGS) $(OBJECTS) -o $@ $(LFLAGS)

.cpp.o:
	g++ $(FLAGS) -c $< -o $@

clean:
	rm *.o $(MAINPROG)
