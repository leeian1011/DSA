Binary search is the divide and conquer algo, simply divide by half, and check, if the value is > the item being searched, we know that the value is not the item so we bump it up by one.
If the value is < the item being searched, we also know that the value is not the item so we bump it down by one.
hence the 
```rust
// if value > item
low = mid + 1;
// if item < value
high = mid - 1;
```
