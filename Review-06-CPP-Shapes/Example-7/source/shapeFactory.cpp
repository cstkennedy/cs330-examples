// Thomas Kennedy
// CS 330 Fall 2014

#include "shapeFactory.h"

ShapeFactory::ShapePair ShapeFactory::_known_shapes[] = {
    ShapePair( "Triangle" ,             new Triangle()            ),
    ShapePair( "Right Triangle" ,       new RightTriangle()       ),
    ShapePair( "Equilateral Triangle" , new EquilateralTriangle() ),
    ShapePair( "Square",                new Square()              ),
    ShapePair( "Circle",                new Circle()              )
}; // No more empty ShapePair

/**
 *
 */
ShapeFactory::ShapePair::ShapePair()
    : _name(), _model( nullptr )
{
}

/**
 *
 */
ShapeFactory::ShapePair::ShapePair( std::string name, Shape *shape )
    : _name( name ), _model( shape )
{
}

ShapeFactory::ShapePair::~ShapePair()
{
    delete _model;
}

/**
 *
 */
Shape* ShapeFactory::createShape( std::string name )
{
    for(const ShapePair& pair : _known_shapes){
        if( pair._name == name ){
            return pair._model->clone();
        }
    }

    // A shape with the given name could not be found
    return nullptr;
}

/**
 *
 */
bool ShapeFactory::isKnown( std::string name )
{
    for(const ShapePair& pair : _known_shapes){
        if( pair._name == name ){
            return true;
        }
    }

    // The shape with the given name is unknown
    return false;
}

/**
 *
 */
void ShapeFactory::listKnown( std::ostream &outs )
{
    for(const ShapePair& pair : _known_shapes){
        outs << " " << pair._name << "\n"; 
    }
}

/**
 *
 */
int ShapeFactory::numberKnown()
{
    int count = 0;

    for(const ShapePair& pair : _known_shapes){
        count++;
    }

    return count;
}


/**
 *
 */
std::istream& operator>>(std::istream& ins, Shape*& rd)
{
    std::string     name;

    // Read name (key) and create the appropiate Shape
    getline(ins, name, ';');
    rd = ShapeFactory::createShape(name);

    if (rd != nullptr) {
        //do other stuff
        rd->read(ins);
    }
    else {
        // throw away the rest of the line
        getline(ins, name);
    }

    return ins;
}