

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
#include <map>
#include <unordered_map>
#include <utility>

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


using vi=vector<int>;
using si=set<int>;
using vd=vector<double>;
using graph = map<int,multiset<int>>;

using ull=long long;
using pii = pair<ull,ull>;
using tri=tuple<int,int,int>;
using pq = priority_queue<int, vector<int>, less<int>>;


template <typename T, class Container = vector<T>,
        class Compare = less<typename Container::value_type>>
class priority_queue_with_remove : public priority_queue<T, Container, Compare> {
public:
    bool remove(const T &valueToRemove) {
        auto itr = std::find(this->c.begin(), this->c.end(), valueToRemove);
        if (itr == this->c.end()) {
            return false;
        }
        this->c.erase(itr);
        // ideally we should not be rebuilding the heap as we are removing only one element
        // a custom implementation of priority queue can adjust only one element in O(logN)
        std::make_heap(this->c.begin(), this->c.end(), this->comp);
        return true;
    }
};

struct gtpii
{
    bool operator()(const pii & p1, const pii & p2)
    {
        auto & [a1,idx1] = p1;
        auto & [a2,idx2] = p2;
        if(a1 != a2) return a1>a2;
        return idx1 > idx2;
    }
};

using pqpii = priority_queue<pii, vector<pii>, gtpii>;


template<typename T>
void run(T & cc){

  int N,S;
  cc >> N;
  cc >> S;

  vector<tri> vt;
  ull si, ai, bi;
  pqpii as, bs;
  ull ha(0LL),hb(0LL);
  ull massa(0LL), massb(0LL);
  fi(0, N) {
      cc >> si;
      cc >> ai;
      cc >> bi;
      if(ai>bi){
          massa += si;
          ha += ai*si;
          as.emplace(ai-bi,si);
      }else{
          massb += si;
          hb += bi*si;
          bs.emplace(bi-ai,si);
      }
  }

  cout << resolve(as,ha,massa,bs,hb,massb,S) << endl;


}
int
main(int argc, char**argv)
{

  ios_base::sync_with_stdio(false); cout.setf(ios::fixed); cout.precision(20);
   
 //#define TEST

#ifdef TEST
  ifstream myfile ("data.txt");
  run(myfile);

  
#else
  run(cin);
#endif
  
}
 
