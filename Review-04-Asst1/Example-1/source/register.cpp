#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <vector>
#include <algorithm>

#include "Course.h"
#include "Student.h"

using namespace std;

typedef vector<Student> StudentList;

/**
 * Trim leading and trailing whitespace from a string.
 * 
 * @param str string to prune
 * 
 * @pre str is nonempty
 */
void trim(string &str);

/**
 * Parse a list of student names
 *
 * @return list of Student objects
 */
StudentList readStudentList(std::istream& ins);

/**
 * Process an enrollment requests file. For each request,
 * lookup the student and attempt to add a selected course
 * to hist/her schedule.
 */
void processEnrollmentRequests(std::istream& ins, StudentList& students);

/**
 * Print the names of all students
 */
void printStudentNames(std::ostream& outs, const StudentList& students);

/**
 * Print each student along with his/her full schedule
 */
void printStudentSchedules(std::ostream& outs, const StudentList& students);

/**
 * Assignment 1: Course Registration
 *
 * @param argv[1] Student List filename
 * @param argv[2] Enrollment Requests filename
 */
int main( int argc, char** argv )
{      
    // Check Command Line Arguments
    if( argc != 3 ){
        cerr << "Usage: " << argv[0] << " student_file request_file" << "\n";
        return 1;
    }

    // Open student file
    ifstream studentFile(argv[1]);
    if(!studentFile){
        cerr << "Error: " << argv[1] << " could not be opened" << "\n";
        return 2;
    }

    // Open request file
    ifstream requestsFile(argv[2]);
    if(!requestsFile){
        cerr << "Error: " << argv[1] << " could not be opened" << "\n";
        return 3;
    }

    StudentList students = readStudentList(studentFile);
    printStudentNames(cout, students);

    processEnrollmentRequests(requestsFile, students);
    printStudentSchedules(cout, students);

    return 0;
}

/**
 *
 */
void trim(std::string &str)
{
    if( str.empty() ){
        return;
    }

    int first_nonspace = str.find_first_not_of(" \t");
    int last_non_space = str.find_last_not_of(" \t");

    str = str.substr( 
        first_nonspace,
        last_non_space + 1
    );
}

/**
 *
 */
vector<Student> readStudentList(std::istream& ins)
{
    vector<Student> students;
    std::string     nextName;

    while (getline(ins, nextName)) {
        trim(nextName);

        students.push_back(Student(nextName));
    }

    return students;
}

/**
 *
 */
void printStudentNames(std::ostream& outs, const StudentList& students)
{
    outs << "\n"
         << "Student List"  << "\n"
         << std::string(42, '-') << "\n";

    for (const Student& student : students) {
        outs << " " << student.getName() << "\n";
    }
}

/**
 *
 */
void processEnrollmentRequests(std::istream& ins, StudentList& students)
{
    std::string studentName;
    std::string courseNumber;
    int         crn;
    int         creditHours;

    cout << "\n"
         << "Enrollment Request Log"  << "\n"
         << std::string(42, '-') << "\n";

    ins >> ws;
    while (getline(ins, studentName, ';')) {
        ins >> ws;
        ins >> courseNumber >> crn >> creditHours;

        StudentList::iterator it = find(students.begin(), 
                                        students.end(), 
                                        Student(studentName));

        if (it != students.end()) {
            bool success;
            success = it->enrollIn(Course(courseNumber, crn, creditHours));

            if (success) {
                cout << it->getName() 
                     << " WAS enrolled in " 
                     << courseNumber << "\n";
            }
            else {
                cout << it->getName() 
                     << " WAS NOT enrolled in " 
                     << courseNumber << "\n";
            }
        }

        ins >> ws;
    }
}

/**
 *
 */
void printStudentSchedules(std::ostream& outs, const StudentList& students)
{
    outs << "\n"
         << "Student Schedule Report"  << "\n"
         << std::string(42, '-') << "\n";


    for (const Student& student : students) {
        outs << student << "\n";
    }
}