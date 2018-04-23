MAINPROG=Fibonacci

.SUFFIXES: .java .class

SOURCES:=$(wildcard *.java)
#SOURCES+=$(wildcard utilities/*.java)
#SOURCES+=$(wildcard */*.java)
OBJECTS=$(SOURCES:.java=.class)
FLAGS=

all: jar

jar: $(OBJECTS)
	jar cvfe $(MAINPROG).jar $(MAINPROG) $(OBJECTS)

$(MAINPROG): $(OBJECTS)
	javac $(FLAGS) $(OBJECTS) $@

.java.class:
	javac $(FLAGS) $<

clean:
	rm *.class $(MAINPROG).jar

run:
	java -jar $(MAINPROG).jar 
