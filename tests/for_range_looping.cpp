// use google tests
#include <gtest/gtest.h>
#include <vector>

class MyContainer
{
public:
    size_t GetSize() const
    {
        return data.size();
    }
    int &GetAt(size_t index)
    {
        return data[index];
    }
    void Add(int value)
    {
        data.push_back(value);
    }

private:
    std::vector<int> data;
};

TEST(ForRangeLooping, HappyPath)
{
    std::vector<int> vec = {1, 2, 3};

    // int is too small, should be size_t;
    for (int i = 0; i < vec.size(); ++i)
    {
        ASSERT_EQ(vec[i], i + 1);
    }

    for (auto i = vec.begin(); i != vec.end(); ++i)
    {
        ASSERT_EQ(*i, i - vec.begin() + 1);
    }

    for (auto value : vec)
    {
        ASSERT_EQ(value, value);
    }
}

// Create a customer iterator for going through this type of container with range based iterator
class MyContainerIterator
{
public:
    MyContainerIterator(MyContainer &container, size_t index) : container(container), index(index) {}

    bool operator!=(const MyContainerIterator &other) const
    {
        return index != other.index;
    }

    int &operator*()
    {
        return container.GetAt(index);
    }

    MyContainerIterator &operator++()
    {
        ++index;
        return *this;
    }

private:
    MyContainer &container;
    size_t index;
};

class MyContainerRange
{
public:
    MyContainerRange(MyContainer &container) : container(container) {}

    MyContainerIterator begin()
    {
        return MyContainerIterator(container, 0);
    }

    MyContainerIterator end()
    {
        return MyContainerIterator(container, container.GetSize());
    }

private:
    MyContainer &container;
};

MyContainerRange all_values(MyContainer &container)
{
    return MyContainerRange(container);
}

TEST(ForRangeLooping, MyContainer)
{
    MyContainer vec;
    vec.Add(1);
    vec.Add(2);
    vec.Add(3);

    // int is too small, should be size_t;
    int size = vec.GetSize();
    for (int i = 0; i < size; i++)
    {
        auto value = vec.GetAt(i);
        ASSERT_EQ(value, i + 1);
    }

    int external_index = 1;
    for (auto value : MyContainerRange(vec))
    {
        ASSERT_EQ(value, external_index);
        ++external_index;
    }

    external_index = 1;
    for (auto value : all_values(vec))
    {
        ASSERT_EQ(value, external_index);
        ++external_index;
    }
}
