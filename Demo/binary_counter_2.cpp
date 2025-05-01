#include <iostream>
#include <vector>


template <typename T, typename Op>
class binary_counter {

private:                          // State:
  std::vector<T> counter;         // vector for the counter
              Op op;              // op
               T zero;            // zero
public:
  binary_counter(const Op& op, const T& zero) : op(op), zero(zero) { counter.reserve(24); }

  void add(T carry) {
    auto first = counter.begin();
    auto last  = counter.end();

    while (first != last) {
        if (*first == zero) { *first = carry;
                               carry = zero;
                               break;
                            }
        carry  = op(*first, carry);
        *first = zero;
        ++first;
    }
    if (carry != zero) counter.push_back(carry);
  }


  T reduce() {                      // returns: value of the counter
    auto first = counter.begin();
    auto last  = counter.end();

    if (first == last)  return zero;

    T result = *first;
    while (++first != last)
        if (*first != zero)  result = op(*first, result);
    return result;
  }
};



        // compares two iterators and returns the one pointing to the smallest element
        template <typename Compare>
        class min_op {
        private:
          Compare cmp;
        public:
          min_op(const Compare& cmp) : cmp(cmp) {}

          template <typename I> I
          operator()(const I& x, const I& y) { return cmp(*y, *x) ? y : x; }
        };


        template <typename I, typename Compare>
        // requires I is a ForwardIterator and Compare is a StrictWeakOrdering on ValueType(I)

        I min_element_binary(I first, I last, Compare cmp) {

          binary_counter<I, min_op<Compare>>   min_counter(cmp, last);

          while (first != last) min_counter.add(first++);
          return min_counter.reduce();
        }


int main() {

  // plugin whatever numbers you want to test with
  auto data = std::vector<int>{ 9,   13,  7,  124, 32, 17, 8, 32, 3, 237, 417, 41, 42,  13, 14, 15 };
  auto end = data.end();
  auto min = min_element_binary(data.begin(), data.end(), std::less<int>());

  if (min == end) { std::cout << "No elements" << std::endl; }
             else { std::cout << "Min is " << *min << std::endl; }

  getchar();
}

