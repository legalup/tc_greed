


#include <bits/stdc++.h>
using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fie(a, b) for(int i=((int)(a)); i <= ((int)(b)); i++)
#define fje(a, b) for(int j=((int)(a)); j <= ((int)(b)); j++)
#define fke(a, b) for(int k=((int)(a)); k <= ((int)(b)); k++)
#define isin(a, aset) (aset.count((a))>0)
#define isnotin(a, aset) (aset.count((a))==0)
#define a2z(v)            (v).begin(),(v).end()
#define len(v)   (v).size()
#define forzin(acont) for(auto z : acont)
  
#define MAXTOP 10000000 //this is the biggest topcoder can do

using di=deque<int>;
using vi=vector<int>;
using si=set<int>;
using pii=pair<int,int>;
using ll=long long;
using ull=unsigned long long;
using bits=bitset<50>;
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

template <class T>
void printv(vector<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}


template <class T>
void printd(deque<T> v)
{
  copy(a2z(v),ostream_iterator<T>(cout,","));
}

std::ostream& operator<<(std::ostream& strm, const std::pair<int,int>& kvPair)
{
 strm << "(" << kvPair.first << "," << kvPair.second << ")";
 return strm;
}

/////////////////////////////////////
// start here
///////////////////////////////////////


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
   
 #define TEST

#ifdef TEST
  ifstream myfile ("data.txt");
  run(myfile);

  
#else
  run(cin);
#endif
  
}
 
