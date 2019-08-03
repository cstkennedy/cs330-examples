// Thomas Kennedy
// CS 330 Fall 2014

#include "shapeFactory.h"

ShapeFactory::ShapePair ShapeFactory::_known_shapes[] = {
    {"Triangle" ,            std::unique_ptr<Shape>(new Triangle())           },
    {"Right Triangle" ,      std::unique_ptr<Shape>(new RightTriangle())      },
    {"Equilateral Triangle", std::unique_ptr<Shape>(new EquilateralTriangle())},
    {"Square",               std::unique_ptr<Shape>(new Square())             },
    {"Circle",               std::unique_ptr<Shape>(new Circle())             }
};  // No more empty ShapePair

//------------------------------------------------------------------------------
Shape* ShapeFactory::createShape(std::string name)
{
    for (const ShapePair& pair : _known_shapes) {
        if (pair.first == name) {
            return pair.second->clone();
        }
    }

    // A shape with the given name could not be found
    return nullptr;
}

//------------------------------------------------------------------------------
bool ShapeFactory::isKnown(std::string name)
{
    for (const ShapePair& pair : _known_shapes) {
        if (pair.first == name) {
            return true;
        }
    }

    // The shape with the given name is unknown
    return false;
}

//------------------------------------------------------------------------------
void ShapeFactory::listKnown(std::ostream &outs)
{
    for (const ShapePair& pair : _known_shapes) {
        outs << " " << pair.first << "\n";
    }
}

//------------------------------------------------------------------------------
int ShapeFactory::numberKnown()
{
    /*
    int count = 0;

    for (const ShapePair& pair : _known_shapes) {
        count++;
    }
    */
    return (end(_known_shapes) - begin(_known_shapes));
}

//------------------------------------------------------------------------------
std::istream& operator>>(std::istream& ins, Shape*& rd)
{
    std::string     name;

    // Read name (key) and create the appropiate Shape
    getline(ins, name, ';');
    rd = ShapeFactory::createShape(name);

    if (rd != nullptr) {
        // Do other stuff
        rd->read(ins);
    }
    else {
        // throw away the rest of the line
        // std::cerr << name << "\n"; // Debug output
        getline(ins, name);
    }
    ins >> std::ws;

    return ins;
}
