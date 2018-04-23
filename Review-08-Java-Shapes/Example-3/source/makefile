MAINPROG=RunShapes

.SUFFIXES: .java .class

SOURCES:=$(wildcard *.java)
#SOURCES+=$(wildcard utilities/*.java)
SOURCES+=$(wildcard */*.java)
OBJECTS=$(SOURCES:.java=.class)
FLAGS=

all: jar

jar: $(OBJECTS)
	jar cvfe $(MAINPROG).jar $(MAINPROG) *.class */*.class

$(MAINPROG): $(OBJECTS)
	javac $(FLAGS) $(OBJECTS) $@

.java.class:shape.class
	javac $(FLAGS) $<

clean:
	rm *.class */*.class $(MAINPROG).jar

run: 
	java -jar $(MAINPROG).jar 
