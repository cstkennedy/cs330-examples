// Thomas Kennedy
// CS 330 Fall 2014

#include "shapeFactory.h"

ShapeFactory::ShapePair ShapeFactory::_known_shapes[] = {
    ShapePair( "Triangle" ,             new Triangle()            ),
    ShapePair( "Right Triangle" ,       new RightTriangle()       ),
    ShapePair( "Equilateral Triangle" , new EquilateralTriangle() ),
    ShapePair( "Square",                new Square()              ),
    ShapePair( "Circle",                new Circle()              ),
    ShapePair()
};

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
    for( int i = 0; _known_shapes[i]._model != nullptr; i++ ){
        if( _known_shapes[i]._name == name ){
            return _known_shapes[i]._model->clone();
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
    for( int i = 0; _known_shapes[i]._model != nullptr; i++ ){
        if( _known_shapes[i]._name == name ){
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
    for( int i = 0; _known_shapes[i]._model != nullptr; i++ ){
        outs << " " << _known_shapes[i]._name << "\n"; 
    }
}

/**
 *
 */
int ShapeFactory::numberKnown()
{
    int count = 0;

    while( _known_shapes[ count ]._model != nullptr ){
        count++;
    }

    return count;
}