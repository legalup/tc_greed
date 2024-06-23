/*
Given an integer array nums, find the
subarray
with the largest sum, and return its sum.
 */

class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int maxsum = numeric_limits<int>::min();
        int currsum(0);

        for(int nn : nums){
            currsum += nn;
            maxsum = max(currsum, maxsum);
            if(currsum<0) currsum = 0;
        }

        return maxsum;
        
    }
};
