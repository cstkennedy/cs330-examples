// Thomas Kennedy
// CS 330 Fall 2019

#include "shapeFactory.h"
#include <algorithm>

ShapeFactory::ShapePair ShapeFactory::_KNOWN_SHAPES[] = {
    {"Triangle" ,            std::unique_ptr<Shape>(new Triangle())           },
    {"Right Triangle" ,      std::unique_ptr<Shape>(new RightTriangle())      },
    {"Equilateral Triangle", std::unique_ptr<Shape>(new EquilateralTriangle())},
    {"Square",               std::unique_ptr<Shape>(new Square())             },
    {"Circle",               std::unique_ptr<Shape>(new Circle())             }
};  // No more empty ShapePair

//------------------------------------------------------------------------------
Shape* ShapeFactory::createShape(std::string_view name)
{
    const auto pair_it = std::find_if(begin(ShapeFactory::_KNOWN_SHAPES),
                                      end(ShapeFactory::_KNOWN_SHAPES),
                                      [name](const ShapePair& pair) -> bool {
                                          return (pair.first == name);
                                      });

    if (pair_it != end(ShapeFactory::_KNOWN_SHAPES)) {
        const auto& [name, model_shape] = *pair_it;
        return model_shape->clone();
        // return (*pair_it).second->clone();
    }

    // A shape with the given name could not be found
    return nullptr;
}

//------------------------------------------------------------------------------
bool ShapeFactory::isKnown(std::string_view name)
{
    return std::find_if(begin(ShapeFactory::_KNOWN_SHAPES),
                        end(ShapeFactory::_KNOWN_SHAPES),
                        [&name](const ShapePair& pair) -> bool {
                            return (pair.first == name);
                        }) != std::end(_KNOWN_SHAPES);
}

//------------------------------------------------------------------------------
void ShapeFactory::listKnown(std::ostream &outs)
{
    for (const auto& [name, model_shape] : ShapeFactory::_KNOWN_SHAPES) {
        outs << " " << name << "\n";
    }
}

//------------------------------------------------------------------------------
int ShapeFactory::numberKnown()
{
    return (end(_KNOWN_SHAPES) - begin(_KNOWN_SHAPES));
}

//------------------------------------------------------------------------------
std::istream& operator>>(std::istream& ins, Shape*& rd)
{
    std::string     name;

    // Read name (key) and create the appropriate Shape
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
