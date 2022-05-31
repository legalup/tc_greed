
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
#include <list>
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
#define isin(a, aset) (aset.count((a))>0)
#define notin(a, aset) (!aset.count((a))>0)
#define a2z(v)            (v).begin(),(v).end()
#define len(v)   (v).size()

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
  cout << endl;
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

//#define MAXN 100
//int dp[MAXN][MAXN][MAXN][MAXN];
//memset(dp, -1, sizeof(dp));

//class that generates all ways of putting
//k things into n buckets
class partition_generator_1
{
 public:
  partition_generator_1(int numobjs, int buckets ):k(buckets-1),N(buckets+numobjs)
  {
    for(int i=k+1;i>=1;i--) state.push_back(N-i);
  }

  //return num objects per bucket
  vi next()
  {
    vi ret;
    if(isdone) return ret;
    
    
    isdone=true;
    //find first i can move back 1
    if(state[0]>0){
      state[0]--;
      isdone=false;
    }
    else{
      fi(1,k){
        if(state[i]-state[i-1]>1){
          isdone=false;
          state[i]--;
          fje(1,i) state[i-j]=state[i]-j;
          break;
        }
      }
    }

    ret.push_back(state[0]);
    fi(1,k) ret.push_back(state[i]-state[i-1]-1);
    ret.push_back(N-state[k-1]-2);

    return ret;
   
  }

  int k;
  int N;
  vi state;
  bool isdone=false;
  
};


//class that generates all ways of putting
//k or less things into n buckets
class partition_generator_2
{
 public:
  partition_generator_2(int numobjs,int buckets):_buckets(buckets),_numobjs(numobjs),_curr_numobjs(numobjs)
  {
    pg1 = new partition_generator_1(numobjs,buckets);
  }

  //return num objects per bucket
  vi next()
  {
    if(pg1==nullptr) return vi();
    
    vi ret = pg1->next();
    if(!ret.empty()){
      return ret;
    }
    delete pg1;
    _curr_numobjs--;
    if(_curr_numobjs>0){
      pg1 = new partition_generator_1(_curr_numobjs, _buckets);
      return pg1->next();
    }
    else{
      pg1 = nullptr;
      vi ret(_buckets,0);
      isdone = true;
      return ret;
    }
      
  }

  int _buckets;
  int _numobjs;
  int _curr_numobjs;
  partition_generator_1 * pg1;
  bool isdone=false;
  
};



int main(int argc, char **argv) {

  partition_generator_2 pg(3,7);
  while(!pg.isdone) {
    
    //    printv(pg.state);
    printv(pg.next());
    //cout << "--------------" << endl;
  }
  
    //cout << "here is argv:" << argv[1] << endl;
    


    
}
