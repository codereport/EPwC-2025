#include <vector>
#include <cstddef>
#include <utility>
#include <iostream>

// binary counter stuff
template <typename T, typename I, typename Op>
// requires BinaryOperation(Op, T) and Associative(Op)
// and I is a ForwardIterator and ValueType(I) == T
// Precondition: carry != zero
auto add_to_counter(I first, I last, Op op, T const& zero, T carry) -> T {
    while (first != last) {
        if (*first == zero) {
            *first = carry;
            return zero;
        }
        carry = op(*first, carry);
        *first = zero;
        ++first;
    }

    return carry;
}

template <typename T, typename I, typename Op>
// requires BinaryOperation(Op, T) and Associative(Op)
// and I is a ForwardIterator and ValueType(I) == T
auto reduce_counter(I first, I last, Op op, T const& zero) -> T {
    // skip initial zeros
    while (first != last && *first == zero) ++first;

    if (first == last) return zero;

    T result = *first;
    ++first;
    while (first != last) {
        if (*first != zero) {
            result = op(result, *first);
        }
        ++first;
    }

    return result;
}

template <typename Op, typename T>
class binary_counter {
    private:
        std::vector<T> counter;
        Op op;
        T zero;

    public:
        binary_counter(Op op, T zero) : op{std::move(op)}, zero{std::move(zero)} {}

        void reserve(size_t n) {
            counter.reserve(n);
        }

        void add(T x) {
            x = add_to_counter(counter.begin(), counter.end(), op, zero, x);
            if (x != zero) {
                counter.push_back(x);
            }
        }

        auto reduce() -> T {
            return reduce_counter(counter.begin(), counter.end(), op, zero);
        }
};

// list_pool stuff
template <typename T, typename N = size_t>
class list_pool {
    public:
        using list_type = N;
    private:
        struct node_t {
            T value;
            list_type next;
        };

        std::vector<node_t> pool;


        template <typename Self>
        auto&& node(this Self&& self, list_type x) {
            return std::forward<Self>(self).pool[x - 1];
        }

        auto new_list() -> list_type {
            pool.push_back(node_t());
            return list_type(pool.size());
        }

        list_type free_list;
    public:
        using size_type = std::vector<node_t>::size_type;
        
        auto end() const -> list_type {
            return list_type(0);
        }

        auto is_end(list_type x) const -> bool {
            return x == end();
        }

        auto empty() const -> bool {
            return pool.empty();
        }

        auto size() const -> size_type {
            return pool.size();
        }

        auto capacity() const -> size_type {
            return pool.capacity();
        }

        void reserve(size_type n) {
            pool.reserve(n);
        }

        list_pool() {
            free_list = end();
        }

        list_pool(size_type n) {
            list_pool();
            reserve(n);
        }

        template <typename Self>
        auto&& value(this Self&& self, list_type x) {
            return std::forward<Self>(self).node(x).value;
        }

        template <typename Self>
        auto&& next(this Self&& self, list_type x) {
            return std::forward<Self>(self).node(x).next;
        }
        
        auto free(list_type x) -> list_type {
            list_type tmp = next(x);
            next(x) = free_list;
            free_list = x;
            return tmp;
        }
        
        auto free(list_type front, list_type back) -> list_type {
            if (is_end(front)) return back;
            list_type tmp = next(back);
            next(back) = free_list;
            free_list = front;
            return tmp;
        }

        auto allocate(T const& val, list_type x) -> list_type {
            list_type list = free_list;
            if (is_end(free_list)) {
                list = new_list();
            } else {
                free_list = next(free_list);
            }
            value(list) = val;
            next(list) = x;
            return list;
        }

};

template <typename T, typename N>
using list_type = typename list_pool<T, N>::list_type;

template <typename T, typename N>
auto push_front(list_pool<T, N>& pool, list_type<T,N> front, list_type<T, N> back, T const& value) {
    auto new_node = pool.allocate(value, front);
    if (pool.is_end(front)) return std::make_pair(new_node, new_node);
    return std::make_pair(new_node, back);
}


template <typename T, typename N>
auto push_back(list_pool<T, N>& pool, list_type<T,N> front, list_type<T, N> back, T const& value) {
    auto new_node = pool.allocate(value, pool.end());
    if (pool.is_end(front)) return std::make_pair(new_node, new_node);
    pool.next(back) = new_node;
    return std::make_pair(front, new_node);
}

template <typename T, typename N>
auto free_list(list_pool<T, N>& pool, list_type<T,N> x) {
    while (!pool.is_end(x)) x = pool.free(x);
}

template <typename T, typename N, typename Compare>
auto min_element_list(list_pool<T, N>& pool, list_type<T,N> x, Compare cmp) {
    if (pool.is_end(x)) return x;
    auto current_min = x;
    x = pool.next(x);
    while (!pool.is_end(x)) {
        if (cmp(pool.value(x), pool.value(current_min))) {
            current_min = x;
        }
        x = pool.next(x);
    }
    return current_min;
}

// min_element stuff
template <typename Cmp>
class cmp_deref {
    private:
        Cmp cmp;
    public:
        cmp_deref(Cmp cmp) : cmp{std::move(cmp)} {}
        template <typename I>
        auto operator()(I x, I y) -> bool {
            return cmp(*x, *y);
        }
};

template <typename I, typename Cmp>
// requires BinaryOperation(Op, T) and Associative(Op)
// and I is a ForwardIterator and ValueType(I) == T
auto min_element_binary(I first, I last, Cmp cmp) -> I {
    if (first == last) return last;

    cmp_deref cmp_d(cmp);
    auto op = [&cmp_d](I x, I y) {
        if (cmp_d(y, x)) {
            return y;
        }
        return x;
    };
    binary_counter counter{std::move(op), last};
    counter.reserve(16);

    while (first != last) {
        counter.add(first);
        ++first;
    }
    return counter.reduce();
}

template <typename I, typename N>
using min12_elt_type = std::pair<I, list_type<I, N>>;

template <typename I, typename N>
auto combine(list_pool<I, N>& pool, min12_elt_type<I, N> winner, min12_elt_type<I, N> loser) -> min12_elt_type<I, N> {
    free_list(pool, loser.second);
    return {winner.first, pool.allocate(loser.first, winner.second)};
}

template <typename I, typename Cmp>
auto min_element12(I first, I last, Cmp cmp) -> std::pair<I, I> {
    // check if range has at most one element
    if (first == last || std::next(first) == last) {
        return std::make_pair(first, first);
    }

    // now the range will have at least 2 elements, so
    // min1 list below will not be empty
    cmp_deref cmp_d(cmp);
    using elt_type = min12_elt_type<I, size_t>;
    list_pool<I, size_t> pool;
    auto op = [&cmp_d, &pool](elt_type x , elt_type y) -> elt_type {
        if (cmp_d(y.first, x.first)) {
            return combine(pool, y, x);

        } else {
            return combine(pool, x, y);
        }
    };
    binary_counter counter{std::move(op), std::move(std::make_pair(last, pool.end()))};
    counter.reserve(16);
    while (first != last) {
        counter.add(std::move(std::make_pair(first, pool.end())));
        ++first;
    }
    auto [min1, min1_list] = counter.reduce();
    auto min2 = pool.value(min_element_list(pool, min1_list, cmp_d));

    return std::make_pair(min1, min2);
}

auto main() -> int {
    std::vector v{3, 8, 0, 7, 9, 1, 2, 5};
    auto f = v.begin();
    auto res = min_element_binary(v.begin(), v.end(), std::less<int>{});
    std::cout << "Min: " << *res << " at position: " << std::distance(f, res) << '\n';
    auto [min1, min2] = min_element12(v.begin(), v.end(), std::less<int>{});
    std::cout << "Min1: " << *min1 << " at position: " << std::distance(f, min1) << '\n';
    std::cout << "Min2: " << *min2 << " at position: " << std::distance(f, min2) << '\n';
}



