// Programmer : Thomas Kennedy

#include "utilities.h"

namespace utilities {
    //--------------------------------------------------------------------------
    bool areEqual(double lhs, double rhs, double threshold)
    {
        return std::abs(rhs - lhs) < threshold;
    }

}
