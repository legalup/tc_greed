/*
Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

You have the following three operations permitted on a word:

    Insert a character
    Delete a character
    Replace a character


 */

class Solution {
public:

    map<tuple<int,int>, int> cache;

    int levenshteinRecursive(const string& str1, const string& str2, int m, int n)
    {

        if(cache.contains({m,n})) return cache[{m,n}];
    
        // str1 is empty
        if (m == 0) {
            cache[{m,n}] = n;
            return n;
        }
        // str2 is empty
        if (n == 0) {
            cache[{m,n}] = m;
            return m;
        }
    
        if (str1[m - 1] == str2[n - 1]) {
            int ret = levenshteinRecursive(str1, str2, m - 1,n - 1);
            cache[{m,n}] = ret;
            return ret;
        }
    
        auto ret =  1
            + min(
    
                // Insert
                levenshteinRecursive(str1, str2, m, n - 1),
                min(
    
                // Remove
                levenshteinRecursive(str1, str2, m - 1,n),
    
                // Replace
                levenshteinRecursive(str1, str2, m - 1, n - 1)));

        cache[{m,n}] = ret;
        return ret;
    }

    int minDistance(string word1, string word2) {

        return levenshteinRecursive(word1, word2, word1.size(),word2.size());
    }
};
