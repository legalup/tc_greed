

#include <iostream>
#include <fstream>
#include <sstream>
#include <iterator>
#include <limits>
#include <queue>
#include <vector>
#include <tuple>
#include <unordered_map>
#include <map>
#include <set>
#include <string>
#include <algorithm>
#include <bitset>
#include <numeric>
#include <cstring>
#include <cstdlib>
#include <cmath>
        
using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fie(a, b) for(int i=((int)(a)); i <= ((int)(b)); i++)
#define fje(a, b) for(int j=((int)(a)); j <= ((int)(b)); j++)
#define fke(a, b) for(int k=((int)(a)); k <= ((int)(b)); k++)
#define a2z(v)            (v).begin(),(v).end()

#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using pii=pair<int,int>;
using ll=long long;
using ull=unsigned long long;
using bits=bitset<50>;

template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

//#define MAXN 100
//int dp[MAXN][MAXN][MAXN][MAXN];
//memset(dp, -1, sizeof(dp));

struct CoinFlipping2
{
    vector<string> layout; 
  int desiredCount; 
  vector<int> correctHeads(vector<string> alayout, int desiredCount)
  {
    this->layout = alayout; 
    this->desiredCount = desiredCount;
    int R = layout.size();
    int C = layout[0].size();

    auto getnumheadsincol = [&](int col)->pii{
      int sum(0),sumflipped(0);
      for(auto rr : layout){
        sum += (rr[col] == 'H') ? 1 : 0;
        sumflipped += (rr[col] == 'T') ? 1 : 0;
      };

      return make_pair(sum,sumflipped);
    };


    auto canPartition = [&](int sum, bits &flippy)->bool{
      vector<vector<bool>> dp(C, vector<bool>(sum + 1,false));

      vector<vector<bits>> flips(C,vector<bits>(sum+1,0LL));

      // with only one number, we can form a subset only when the required sum is equal to its value
      auto num0 = getnumheadsincol(0);
      dp[0][num0.first]=true;
      flips[0][num0.first] = 0LL;
      dp[0][num0.second]=true;
      flips[0][num0.second] = 0LL;
      flips[0][num0.second].set(0);

      // process all subsets for all sums
      for (int i = 1; i < C; i++) {
        pii z = getnumheadsincol(i);
        int nhs = z.first;
        int nts = z.second;
        for (int s = 0; s <= sum; s++) {
          // without contributing
          if (s >= nts && dp[i - 1][s - nts]) {
            dp[i][s] = true;
            flips[i][s] |= flips[i-1][s-nts];
            flips[i][s].set(i);
          }
          //adding to previous sum
          else if (s >= nhs && dp[i - 1][s - nhs]){
            dp[i][s] = true;
            flips[i][s] |= flips[i-1][s-nhs];            
          }
        }
      }
      
      // the bottom-right corner will have our answer.
      flippy = flips[C-1][sum];

      return dp[C - 1][sum];
    };


    int rows2flip = 0;
    int numrows2flip = (1 << R);
    vector<string> orig = layout;

    for(; rows2flip < numrows2flip; rows2flip++){

      layout = orig;
      bits bb(rows2flip);
      fi(0,R){
        if(bb.test(i)){
          flip(i);
        }
      }

      bits ret;
      if(canPartition(desiredCount,ret))
     {
        vi ret1;
        fi(0,R) if(bb.test(i)) ret1.push_back(i);
        fi(0,C) if(ret.test(i)) ret1.push_back(R+i);
        return ret1;
      }
    };

    vi ret;
    ret.push_back(-1);
    return ret;

  };

  void flip(int row){
    for(auto & ch : layout[row]){
      if(ch == 'H') ch ='T';
      else if(ch == 'T') ch ='H';
    }
  }
 
};

        
