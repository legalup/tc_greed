

#include <iostream>
#include <fstream>
#include <sstream>
#include <iterator>
#include <limits>
#include <queue>
#include <vector>
#include <tuple>
#include <unordered_map>
#include <set>
#include <string>
#include <algorithm>
#include <bitset>
#include <numeric>


using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fori(a) for(int i=((int)(0)); i < ((int)(a.size())); i++)
#define forj(a) for(int j=((int)(0)); j < ((int)(a.size())); j++)
#define a2z(v)            (v).begin(),(v).end()
#define isin(b,a)        find(all(a),(b))!=(a).end()
#define isnotin(b,a)        find(all(a),(b))==(a).end()
#define len(v)   (v).size()

#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using ull=unsigned long long;
using pii=pair<int,int>;


template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

#define MAXN 50




class  ABCPath{
  int dp[MAXN][MAXN];
  vector<string> mygrid;
  
  int find_longest(int row, int col)
  {
    if(dp[row][col] >=0) return dp[row][col];
    char ch = mygrid[row][col];
    vector<pii> nbrs;
    int numrows = mygrid.size();
    int numcols = mygrid[0].size();
    for(int r=row-1; r<=row+1; r++)
      for(int c=col-1; c<=col+1; c++){
        if(r==row && c==col) continue;
        if(c>=0 && c<numcols && r>=0 && r<numrows){
          char nxtch = mygrid[r][c];
          if(nxtch-ch == 1) nbrs.push_back(make_pair(r,c));
        }
      }

    priority_queue<int> lengths;
    for(auto pr : nbrs){
      lengths.push(find_longest(pr.first,pr.second));
    };
    
    int longest(0);
    if(!lengths.empty()) longest = lengths.top()+1;
    dp[row][col] = longest;
    return longest;
  };
  
 public:
  
  int length(vector <string> grid)
  {
    memset(dp, -1, sizeof(dp));
    mygrid = grid;
    priority_queue<int> pq;

    int nrs = grid.size();
    int ncs = grid[0].size();
    for(int r=0; r<nrs; r++)
      for(int c=0; c<ncs; c++)
        if(grid[r][c]=='A') pq.push(find_longest(r,c));

    if(pq.empty()) return 0;
    return pq.top()+1;
  }
};


// int
// main(int argc, char**argv)
// {

//   ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
//   string str = "abc";

//   int sz = str.size();
//   cout << "the last char of str is:" << str[sz-1] << endl;
    
  
// }
 
