# Binary Search
Binary search is the divide and conquer algo, simply divide by half, and check, if the value is > the item being searched, we know that the value is not the item so we bump it up by one.
If the value is < the item being searched, we also know that the value is not the item so we bump it down by one.
hence the 
```rust
// if value > item
low = mid + 1;
// if item < value
high = mid - 1;
```

# Selection Sort
First sorting-algo with O(n^2), the book I'm reading through suggested to init a new Vec/List and append the smallest member from the read Vec/List to the new Vec/List.
I've changed it so that the selection sort function changes the Vec in place.
Honestly, borderline wrote C code.<br></br>
Explaining the loop,
```rust
for i in 0..length {
    for j in i..length {
        // logic
    }
}
```
- The first `i`-based for loop is to iterate the over the entire length of the array, we do this to ensure we are placing the smallest member in the first position, second smallest in the second position
and so on and so forth.

- The following `j`-based for loop is to iterate the remaining ***unsorted*** array, we know that at the end of the `j` loop we would have found the smallest member that is not placed into their
correct position. We iterate from `i` up, because we know that `i - 1` has been sorted in the previous iteration.
