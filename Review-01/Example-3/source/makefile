ifneq (,$(findstring MinGW,$(PATH)))
DISTR=MinGW
LFLAGS=
else
DISTR=Unix
EXE=
LFLAGS=/home/zeil/src/LeakTracer/LeakTracer.o
endif

MAINPROG=ll2

SOURCES:=$(wildcard *.cpp)
OBJECTS=$(SOURCES:.cpp=.o)
FLAGS=-std=c++11

all: $(SOURCES) $(MAINPROG)

$(MAINPROG): $(OBJECTS)
	g++ $(FLAGS) $(OBJECTS) -o $@ $(LFLAGS)

.cpp.o:
	g++ $(FLAGS) -c $< -o $@

clean:
	rm *.o $(MAINPROG)
