
#include <bits/stdc++.h>
using namespace std;


#define fi(a, b) for(int i=((int)(a)); i < ((int)(b)); i++)
#define fj(a, b) for(int j=((int)(a)); j < ((int)(b)); j++)
#define fk(a, b) for(int k=((int)(a)); k < ((int)(b)); k++)
#define fori(a) for(int i=((int)(0)); i < ((int)(a.size())); i++)
#define forj(a) for(int j=((int)(0)); j < ((int)(a.size())); j++)
#define a2z(v)            (v).begin(),(v).end()
#define isin(a, aset) (aset.count((a))>0)
#define isnotin(a, aset) (aset.count((a))==0)
#define len(v)   (v).size()
#define forzin(acont) for(auto z : acont)

using vi=vector<int>;
using si=set<int>;
using vd=vector<double>;
using graph = map<int,multiset<int>>;

using ull=unsigned long long;
using ll = long long;
using pii = pair<int,int>;
using tri=tuple<int,int,int>;
using pq = priority_queue<int, vector<int>, less<int>>;
using board = vector<string>;



const int nrows=27;
const int ncols=122;


namespace std
{
  std::ostream& operator<<(std::ostream& strm,
                           const std::pair<int,int>& kvPair)
    {
      strm << "(" << kvPair.first << "," << kvPair.second << ")";
      return strm;
    }
}


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

void
split( vector<string> & theStringVector,  /* Altered/returned value */
       const  string  & theString,
       const  string & theDelimiters)
{
  assert(!theDelimiters.empty());
  char buffy[theString.size()];
  strcpy(buffy,theString.c_str());

  char  * str1 = strtok(buffy,theDelimiters.c_str());
  theStringVector.push_back(str1);

  while((str1 = strtok(0,theDelimiters.c_str()))) theStringVector.push_back(str1);
  return;

}


inline int gcd(int a, int b) { if (b==0) return a; return gcd(b, a%b); }
inline int lcm(int a, int b) { return (a*b)/gcd(a,b);                  }

//////////////////////////////////
// start here//
//////////////////////////

struct storm
{
  storm(int r, int c, char cc): row(r),col(c)
  {
    ch.push_back(cc);
  };

  storm() = default;
  ~storm() = default;
  
  int row;
  int col;
  int cols=ncols-2;
  int rows=nrows-2;
  string ch;

  
  pii get_pos(int time)
  {
    if(ch[0] == '>'){
      int nucol = (col-1+time) % cols;
      return {row,nucol+1};
    }
   
    if(ch[0] == 'v'){
      int nurow = (row-1+time) % rows;
      return {nurow+1,col};
    }
   
    if(ch[0] == '<'){
      int nucol = (col-1-time) % cols;
      if(nucol<0) nucol += cols;
      return {row,nucol+1};
    }
   
    int nurow = (row-1-time) % rows;
    if(nurow<0) nurow+= rows;
    return {nurow+1,col};
   
  };
};

void print(storm * st){
  printf("(%d,%d) with %s\n",st->row,st->col,st->ch.c_str());
}

vector<pii> getnbrs(pii z){
  auto [r,c] = z;
  vector<pii> ret;
  if(r>0) ret.push_back({r-1,c});
  if(c>0) ret.push_back({r,c-1});
  if(r<nrows-1) ret.push_back({r+1,c});
  if(c<ncols-1) ret.push_back({r,c+1});
  ret.push_back(z);
  return ret;
}



template<typename T>
void run(T & cc){

  vector<string> lines;
  for (std::string line; std::getline(cc, line); ) 
  {
    if(!line.empty())
    lines.push_back(line);
  }

  assert(nrows == len(lines));
  assert(ncols == len(lines[0]));


  vector<storm*> storms;
  for(int r=0; r<nrows;r++)
    for(int c=0; c<ncols; c++){
      
      char ch = lines[r][c];
      if(ch == 'v' || ch == '>' || ch == '<' || ch == '^' ){
        storm * st = new storm(r,c,ch);
        storms.push_back(st);
      }
    }


  //verify
  forzin(storms){
    print(z);
  }

  

  
  pii orig = {0,1};
  pii dest = {nrows-1,ncols-2};
  
  int timeconstant = lcm(nrows-2,ncols-2);
  printf("nrows %d, ncols %d, time constant is: %d\n", nrows, ncols, timeconstant);

  //lets compute all the masks
  vector<board> boards;
  for(int i=0; i<timeconstant; i++){
    string zrow = "";
    fi(0,ncols) zrow.push_back('.');
    board bb = board(nrows,zrow);


    forzin(storms){
      auto pos = z->get_pos(i);
      bb[pos.first][pos.second] = '#';
    }

    fi(0,nrows){
      bb[i][0] = '#';
      bb[i][ncols-1] = '#';
    }

    fi(0,ncols){
      bb[0][i] = '#';
      bb[nrows-1][i] = '#';
    }

    bb[orig.first][orig.second] = '.';
    bb[dest.first][dest.second] = '.';

    boards.push_back(bb);
    
  };

  

  cout << "dest is: " << dest << endl;
  cout << "timeconstant is: " << timeconstant << endl;
  

  auto getkids = [&](pii start, pii destiny, int timeoffset, bool viz)->int{

                   map<pii, int> best_times;
                   deque<pii> q;
                   q.push_back(start);
                   
                   //cout << "here in getkids. timeoffset is: " << timeoffset << endl;
                   for(int time=0; time<1000; time++){
                     int mytime = (time + timeoffset) % timeconstant;
                     board tmp = boards[mytime];
                     int sz = len(q);
                     set<pii> done; //only for this iter. keep track of whats one q

                     fi(0,sz){
                       auto z = q.front();
                       q.pop_front();
                       if isin(z,done) continue;
                       auto [r,c] = z;
                       for(auto nbr: getnbrs(z)){

                         if(nbr == destiny){
                           return time+1;
                         }

                         if(boards[mytime][nbr.first][nbr.second]!='#' && isnotin(nbr,done)){
                           q.push_back(nbr);
                           //tmp[nbr.first][nbr.second] = 'E';
                         }

                         
                       }
                       done.insert(z);
                     }

                     //////////////////////////////
                     //printing this board out
                     if(viz){
                       forzin(tmp){
                         cout << z << endl;
                       }

                       char ch;
                       cin >> ch;
                     }
                     // done with printing
                     
                     
                   }
                  

                   return -1;
                 };


  auto gettraveltime = [&](pii start, pii destiny, int timestart, bool viz)->int{

                       
                         return  getkids(start,destiny,timestart,viz);
                        
                      

                       };

auto t1 = gettraveltime(orig,dest,1, false);
cout << "t1 " << t1 << endl;

  int mintime = 100000000;
  map<int,int> t2time, t3time;

  fi(0,timeconstant) t2time[i] = gettraveltime(dest,orig,i, false);
  cout << "finished computing dest to orig" << endl;
  fi(0,timeconstant) t3time[i] = gettraveltime(orig,dest,i, false);
  cout << "finished computing orig to dest" << endl;
    
  fi(1,timeconstant)fj(1,timeconstant){

    int t2 = t2time[(i+t1)%timeconstant];
    int tot = t1+i+t2+j+t3time[(j+t2+i)%timeconstant];
    if(tot < mintime){
      mintime = min(mintime,tot);
      printf(" t2 : %d, total trip time: %d\n",t2,tot);
    }
  };

  cout << "mintime is: " << mintime << endl;
           
  //cout << "travel time is: " << t1+t2+t3 << endl;
};

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
 
