#include <utility>

#include "Schedule.h"

using namespace std::rel_ops;

/**
 *
 */
Schedule::Schedule()
    :head(nullptr), 
     tail(nullptr),
     totalCredits(0)
{
}

/**
 *
 */
Schedule::Schedule(const Schedule& src)
    :head(nullptr), 
     tail(nullptr),
     totalCredits(0)
{
    copyList(src);
}

/**
 *
 */
Schedule::~Schedule()
{
    deAllocateList();
}

/**
 *
 */
bool Schedule::add(Course course)
{
    // Check if adding this course would put the student over
    // 12 credit hours
    if (totalCredits + course.getCredits() > CREDIT_LIMIT) {
        return false;
    }

    // Check if the student is registered
    // for a different section of the same course
    Node* it = head;

    while (it != nullptr) {
        if ((it->data) == course) {
            return false;
        }

        it = it->next;
    }

    // I am guaranteed that I can add the new Course 
    // _course_
    Node* newNode = new Node(course);

    if (head == nullptr) {
        head = newNode;
        tail = newNode;

        totalCredits += course.getCredits();

        return true;
    }

    // The course can be added without further checks
    tail->next = newNode;
    tail       = tail->next;

    totalCredits += course.getCredits();

    return true;
}

/**
 *
 */
void Schedule::display(std::ostream& outs) const
{
    Node* it = head;

    outs << "  (" << totalCredits << " Total Credits)" << "\n";

    while (it != nullptr) {
        outs << "  - " << (it->data) << "\n";

        it = it->next;
    }    
}

/**
 *
 */
Schedule& Schedule::operator=(const Schedule& rhs)
{
    if (this != &rhs) {
        deAllocateList();
        copyList(rhs);
    }

    return *this;
}

/**
 *
 */
void Schedule::deAllocateList()
{
    Node* it = head;

    while (it != nullptr) {
        Node* prev = it;
        it = it->next;

        delete prev;
    }

    it   = nullptr;
    head = nullptr;
    tail = nullptr;
}

/**
 *
 */
void Schedule::copyList(const Schedule& rhs)
{
    Node* it = rhs.head;

    while(it != nullptr) {
        this->add(it->data);

        it = it->next;
    }
}