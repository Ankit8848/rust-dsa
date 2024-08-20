/**
 * [26] Remove Duplicates from Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element 
 * appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
 *
 * Example 1:
 *
 *
 * Given nums = [1,1,2],
 *
 * Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
 *
 * It doesn't matter what you leave beyond the returned length.
 *
 * Example 2:
 *
 *
 * Given nums = [0,0,1,1,1,2,2,3,3,4],
 *
 * Your function should return length = 5, with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.
 *
 * It doesn't matter what values are set beyond the returned length.
 *
 *
 *
 */

 // problem link: https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// solution :

 impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
        // standard library Vec::dedup() , removes consecutive repeated elements in the vector.
        
        nums.dedup();
        nums.len() as i32
        
    }
}