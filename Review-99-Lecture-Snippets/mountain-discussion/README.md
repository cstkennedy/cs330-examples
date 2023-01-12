# Pseudocode

  1. Generate 4 triangles
  2. Place them in a queue
  3. while queue is not empty
    1. pop the next triangle from the queue
    2. if the triangle can be split
      1. Split it into 3 smaller triangles using the centroid
      2. Add three new triangles to the queue
    3. If the triangle can not be split, place it in the completed list

 # First Pass at Python-izing the Logic

 ```python
from queue import Queue

class Triangle:
    pass

 
work_queue = Queue()

# Create 4 starting triangles
work_queue.put(Triangle(...)) 
work_queue.put(Triangle(...)) 
work_queue.put(Triangle(...)) 
work_queue.put(Triangle(...)) 

while not work_queue.empty():
    tri = work_queue.get()

    if canBeSplit(tri()):
        # split
    else:
        # PLace in finished list


 ```

