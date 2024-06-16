/*
There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

    For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.

Return true if you can finish all courses. Otherwise, return false.

 

Example 1:

Input: numCourses = 2, prerequisites = [[1,0]]
Output: true
Explanation: There are a total of 2 courses to take. 
To take course 1 you should have finished course 0. So it is possible.

Example 2:

Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
Output: false
Explanation: There are a total of 2 courses to take. 
To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.

 

Constraints:

    1 <= numCourses <= 2000
    0 <= prerequisites.length <= 5000
    prerequisites[i].length == 2
    0 <= ai, bi < numCourses
    All the pairs prerequisites[i] are unique.


 */

class Solution {
public:

    set<int> visited;
    map<int,vector<int>> graph;

    bool has_cycle(int v, deque<int> stk){
        stk.push_back(v);

        if(graph.contains(v)) for(auto nbr : graph[v]){
                //make sure not in stack already. if so, have a cycle
                if(find(stk.begin(), stk.end(),nbr)!=stk.end()) return true;

                //if not, put it in stack, and dfs
                if(visited.count(nbr)==0) {
                    if(has_cycle(nbr,stk)) return true;;
                }
            }

        visited.insert(v);
        return false;
    }

    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        deque<int> stk;
        for(auto pr : prerequisites){
            int a = pr[0];
            int b = pr[1];
            if(graph.count(a)==0){
                graph[a] = {b};
            }
            else graph[a].push_back(b);
        }

        for(int v=0; v< numCourses; v++){
            if(visited.count(v)==0) {
                if(has_cycle(v,stk)) return false;
            }
        }

        return true;
    }
};
