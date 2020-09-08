#ifndef NAIVE_POOL_H
#define NAIVE_POOL_H

#ifdef POOLDEBUG
#include <iostream>
#endif

#include <utility>
#include <algorithm>

/**
 * This is a simple (i.e., naive) memorypool implementation. We will stick with
 * `new[]` and a fixed size array of pointers for simplicity and familiarity.
 * The type, `T`, represents the DataType to store.
 */
template <typename T>
class NaivePool {
    public:

        NaivePool()
          :NaivePool(8, 1)
        {
        }

        /**
         * I initially forgot interface completeness... :(
         */
        NaivePool(int preAlloc)
          :NaivePool(8, preAlloc)
        {
        }

        /**
         * Allocate a pool with some initial storage space.
         *
         * @param bSize number of blocks to store in each internal pool.
         * @param preAlloc number of blocks to preallocate (this value is
         *      rounded up).
         */
        NaivePool(int bSize, int preAlloc)
        {
            nextAvailBlock = {0, 0};

            numPools = (preAlloc + bSize - 1) / bSize;

            thePools = new T*[numPools];
            blocksPerPool = bSize;

            for (int i = 0; i < numPools; i++) {
                thePools[i] = new T[blocksPerPool];
            }
        }

        /**
         * Deallocate all pools
         */
        ~NaivePool()
        {
            int i = 0;

            while (i < numPools && thePools[i] != nullptr) {
                delete[] thePools[i]; // Mistake -> delete vs delete[]

                // Mistake -> I forgot the increment step
                i++;
            }

            delete[] thePools;
        }

        /*
         * I am technically violating the rule of the Big-3 by not overloading
         * the assignment operator.
         */

        /**
         * Return a read/write pointer to the next available memory block.
         */
        T* getNext()
        {
            // Mistake 2 -> over aggressive int vs int& optimization.
            const int poolIdx = nextAvailBlock.first;
            const int blockIdx = nextAvailBlock.second;

            reserveNext();
            return &thePools[poolIdx][blockIdx];
        }

        friend
        void swap(NaivePool& lhs, NaivePool& rhs)
        {
            using std::swap;

            swap(lhs.thePools, rhs.thePools);
            swap(lhs.numPools, rhs.numPools);
            swap(lhs.blocksPerPool, rhs.blocksPerPool);
            swap(lhs.nextAvailBlock, rhs.nextAvailBlock);
        }

    private:
        /**
         * Reserve the next available position. If all internal (sub) pools
         * have been exhausted, prepare another set of pools by:
         * <ol>
         *  <li> Doubling the size of thePools. </li>
         *  <li> Allocating a pool at thePools[numPools]. </li>
         *  <li>
         *      Setting thePools indices numPools + 1 to 2*numPools to nullptr.
         *  </li>
         * </ol>
         */
        void reserveNext()
        {
            int& poolIdx = nextAvailBlock.first;
            int& blockIdx = nextAvailBlock.second;

            blockIdx++;

            // Handle the cases where `thePools` will not need to be resized.
            if (blockIdx < blocksPerPool) {
                #ifdef POOLDEBUG
                std::cerr << "Easy Case -> No alloc\n";
                #endif
                return;
            }

            if (poolIdx + 1 < this->numPools) {
                #ifdef POOLDEBUG
                std::cerr << "Easy Case -> New Pool Start\n";
                #endif
                poolIdx += 1;
                blockIdx = 0;

                if (thePools[poolIdx] == nullptr) {
                    thePools[poolIdx] = new T[blocksPerPool];
                }

                #ifdef POOLDEBUG
                std::cerr << "Easy Case -> New Pool Done\n";
                #endif
                return;
            }

            // If this point is reached, more pools need to be allocated.
            #ifdef POOLDEBUG
            std::cerr << "The fun has begun!\n";
            #endif
            // Once more pools are allocated, these indices will represent the
            // next available position.
            poolIdx = numPools;
            blockIdx = 0;

            numPools *= 2; // This can be optimized with a bit-shift.
            T** newPools = new T*[numPools];

            for (int i = 0; i < poolIdx; i++) {
                newPools[i] = thePools[i];
            }

            newPools[poolIdx] = new T[blocksPerPool];

            for (int i = poolIdx + 1; i < numPools; i++) {
                newPools[i] = nullptr;
            }

            // Delete *only* the outer layer of pointers.
            delete[] thePools;

            thePools = newPools;
        }

    private:
        /**
         * Datatype to store the <pool offset, internal pool idx> pair.
         */
        using PosPair = std::pair<int, int>;

        /**
         * An array of arraypools
         */
        T** thePools;

        /**
         * Current number of internal pools.
         */
        int numPools;

        /**
         * Number of blocks in each sub-pool (internal pool or chunk).
         */
        int blocksPerPool;

        /**
         * Next available block.
         */
        PosPair nextAvailBlock;
};

#endif

